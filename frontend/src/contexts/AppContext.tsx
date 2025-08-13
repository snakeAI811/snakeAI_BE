import React, {
  createContext,
  useContext,
  ReactNode,
  useState,
  useEffect,
  useCallback,
  useMemo,
} from 'react';
import { useAuth } from './AuthContext';
import { useWalletContext } from './WalletContext';
import { tokenApi, userApi, roleApi } from '../pages/patron/services/apiService';
import { UserRole } from '../pages/patron/index';
import { safeFetch } from '../utils/common';

// -----------------------
// Interfaces
// -----------------------

interface MiningStatus {
  current_phase: 1 | 2;
  phase1_tweet_count: number;
  phase2_tweet_count: number;
  total_phase1_mined: number;
  total_phase2_mined: number;
  phase2_start_date: string;
}

interface UserProfile {
  twitter_username: string;
  wallet_address: string;
  latest_claim_timestamp: string | null;
  reward_balance: number;
  claimable_rewards: number;
  tweets: number;
  likes: number;
  replies: number;
}

interface TokenInfo {
  balance: number;
  balance_ui: number;
  decimals: number;
  lockEndDate: number; // Unix timestamp
  locked: number;
  locked_ui: number;
  mining_count: number;
  mining_rewards: number;
  rewards: number;
  rewards_ui: number;
  staked: number;
  staked_ui: number;
  staking: {
    apy_rate: number;
    can_claim_yield: boolean;
    is_locked: boolean;
    lock_duration_months: number;
    user_role: string;
  };
  yield_rewards: number;
  yield_rewards_ui: number;
}

interface AppContextState {
  miningStatus: MiningStatus | null;
  userProfile: UserProfile | null;
  tokenInfo: TokenInfo | null;
  userRole: UserRole;
  loading: boolean;
  error: string | null;
  refreshAllAppData: () => Promise<void>;
  fetchMiningStatus: () => Promise<void>;
  updateUserRole: (role: UserRole) => void;
  isWalletRequired: boolean;
  showWalletWarning: boolean;
}

interface AppContextProviderProps {
  children: ReactNode;
}

// -----------------------
// Context Initialization
// -----------------------

const AppContext = createContext<AppContextState>({
  miningStatus: null,
  userProfile: null,
  tokenInfo: null,
  userRole: { role: 'none' },
  loading: false,
  error: null,
  refreshAllAppData: async () => { },
  fetchMiningStatus: async () => { },
  updateUserRole: () => { },
  isWalletRequired: false,
  showWalletWarning: false,
});

export const useAppContext = () => useContext(AppContext);

// -----------------------
// Utility Function
// -----------------------



// -----------------------
// Provider Component
// -----------------------

export const AppContextProvider: React.FC<AppContextProviderProps> = ({ children }) => {
  const { isAuthenticated } = useAuth();
  const { connected, publicKey } = useWalletContext();

  const [miningStatus, setMiningStatus] = useState<MiningStatus | null>(null);
  const [userProfile, setUserProfile] = useState<UserProfile | null>(null);
  const [tokenInfo, setTokenInfo] = useState<TokenInfo | null>(null);
  const [userRole, setUserRole] = useState<UserRole>({ role: 'none' });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const isWalletRequired = isAuthenticated && !connected;
  const showWalletWarning = isWalletRequired;

  const { getMiningStatus, getTokenInfo } = tokenApi;
  const { getProfile } = userApi;

  // Wrap getMiningStatus to always return a compatible object for safeFetch
  const getMiningStatusWrapped = async () => {
    const result = await getMiningStatus();
    return {
      success: result.success,
      data: result.data ?? null,
      error: result.error,
    };
  };

  const fetchMiningStatus = useCallback(
    () => safeFetch(getMiningStatusWrapped, setMiningStatus, 'fetch mining status'),
    []
  );

  // Wrap getProfile to always return a compatible object for safeFetch
  const getProfileWrapped = async () => {
    const result = await getProfile();
    return {
      success: result.success,
      data: result.data ?? null,
      error: result.error,
    };
  };

  const fetchUserProfile = useCallback(
    () => isAuthenticated && safeFetch(getProfileWrapped, setUserProfile, 'fetch user profile'),
    [isAuthenticated]
  );

  // Wrap getTokenInfo to always return a compatible object for safeFetch
  const getTokenInfoWrapped = async () => {
    const result = await getTokenInfo();
    return {
      success: result.success,
      data: result.data ?? null,
      error: result.error,
    };
  };

  const fetchTokenInfo = useCallback(
    () => isAuthenticated && safeFetch(getTokenInfoWrapped, setTokenInfo, 'fetch token info'),
    [isAuthenticated]
  );

  const fetchUserRole = useCallback(async () => {
    if (!isAuthenticated || !connected) {
      setUserRole({ role: 'none' });
      return;
    }

    try {
      const result = await roleApi.getUserRole();
      if (result.success && result.data) {
        const backendRole = result.data.role?.toLowerCase();
        let mappedRole: UserRole['role'] = 'none';
        if (backendRole === 'staker') mappedRole = 'staker';
        else if (backendRole === 'patron') mappedRole = 'patron';
        setUserRole({ ...result.data, role: mappedRole });
      } else {
        console.error('Failed to fetch user role:', result.error);
        setUserRole({ role: 'none' });
      }
    } catch (error) {
      console.error('Error fetching user role:', error);
      setUserRole({ role: 'none' });
    }
  }, [isAuthenticated, connected]);

  const updateUserRole = useCallback((newRole: UserRole) => {
    setUserRole(newRole);
  }, []);

  const refreshAllAppData = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      await Promise.all([
        fetchMiningStatus(),
        fetchUserProfile(),
        fetchTokenInfo(),
        fetchUserRole(),
      ]);
    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to refresh data');
    } finally {
      setLoading(false);
    }
  }, [fetchMiningStatus, fetchUserProfile, fetchTokenInfo, fetchUserRole]);

  // Wallet address update
  useEffect(() => {
    const updateWalletAddress = async () => {
      if (connected && publicKey && isAuthenticated) {
        try {
          if (userProfile?.wallet_address === publicKey) return;
          const result = await userApi.setWalletAddress(publicKey);
          if (result.success) {
            await fetchUserProfile();
          }
        } catch (error) {
          const isAlreadySetError =
            error instanceof Error &&
            (error.message.includes('already set') || error.message.includes('already using'));
          if (!isAlreadySetError) {
            console.error('Failed to update wallet address:', error);
          }
        }
      }
    };

    updateWalletAddress();
  }, [connected, publicKey, isAuthenticated, fetchUserProfile, userProfile?.wallet_address]);

  // Initial + reset on auth change
  useEffect(() => {
    if (isAuthenticated) {
      refreshAllAppData();
    } else {
      setUserProfile(null);
      setTokenInfo(null);
      setMiningStatus(null);
    }
  }, [isAuthenticated, refreshAllAppData]);

  // Optional: periodic mining status refresh
  // useEffect(() => {
  //   const interval = setInterval(() => {
  //     if (isAuthenticated) {
  //       fetchMiningStatus();
  //     }
  //   }, 30000); // 30s

  //   return () => clearInterval(interval);
  // }, [isAuthenticated, fetchMiningStatus]);

  const value = useMemo(
    () => ({
      miningStatus,
      userProfile,
      tokenInfo,
      userRole,
      loading,
      error,
      refreshAllAppData,
      fetchMiningStatus,
      updateUserRole,
      isWalletRequired,
      showWalletWarning,
    }),
    [
      miningStatus,
      userProfile,
      tokenInfo,
      userRole,
      loading,
      error,
      refreshAllAppData,
      fetchMiningStatus,
      updateUserRole,
      isWalletRequired,
      showWalletWarning,
    ]
  );

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
};
