import React, { createContext, useContext, ReactNode, useState, useEffect, useCallback } from 'react';
import Cookies from 'js-cookie';
import { userApi, setAuthErrorHandler } from '../pages/patron/services/apiService';
import { useNavigate } from 'react-router-dom';

// Phantom wallet types
interface PhantomProvider {
  isPhantom: boolean;
  publicKey: {
    toString(): string;
  } | null;
  isConnected: boolean;
  connect(): Promise<{ publicKey: { toString(): string } }>;
  disconnect(): Promise<void>;
  signMessage(message: Uint8Array): Promise<{ signature: Uint8Array }>;
  on(event: string, callback: Function): void;
  off(event: string, callback: Function): void;
}

interface User {
  id: string;
  twitter_username?: string;
  wallet_address?: string;
}

interface AuthContextState {
  isAuthenticated: boolean;
  user: User | null;
  sessionId: string | null;
  isWalletConnected: boolean;
  walletAddress: string | null;
  isConnecting: boolean;
  login: (sessionId: string) => void;
  logout: () => void;
  setUser: (user: User | null) => void;
  connectWallet: () => Promise<void>;
  disconnectWallet: () => Promise<void>;
  signMessage: (message: string) => Promise<string | null>;
}

interface AuthContextProviderProps {
  children: ReactNode;
}

declare global {
  interface Window {
    phantom?: {
      solana?: PhantomProvider;
    };
  }
}

const AuthContext = createContext<AuthContextState>({
  isAuthenticated: false,
  user: null,
  sessionId: null,
  isWalletConnected: false,
  walletAddress: null,
  isConnecting: false,
  login: () => { },
  logout: () => { },
  setUser: () => { },
  connectWallet: async () => { },
  disconnectWallet: async () => { },
  signMessage: async () => null,
});

export const useAuth = () => useContext(AuthContext);

export const AuthContextProvider: React.FC<AuthContextProviderProps> = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [user, setUser] = useState<User | null>(null);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [isWalletConnected, setIsWalletConnected] = useState(false);
  const [walletAddress, setWalletAddress] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const [provider, setProvider] = useState<PhantomProvider | null>(null);
  
  const navigate = useNavigate();

  // Check if we're on mobile
  const isMobile = useCallback(() => {
    return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
  }, []);

  // Get Phantom provider
  const getProvider = useCallback((): PhantomProvider | null => {
    if (typeof window !== 'undefined' && 'phantom' in window) {
      const provider = window.phantom?.solana;
      if (provider?.isPhantom) {
        return provider;
      }
    }
    return null;
  }, []);

  // Mobile deep link to Phantom wallet
  const openPhantomDeepLink = useCallback(() => {
    const dappUrl = encodeURIComponent(window.location.href);
    const redirectUrl = encodeURIComponent(`${window.location.origin}/wallet-callback`);
    
    // Phantom deep link format
    const deepLink = `phantom://v1/connect?app_url=${dappUrl}&redirect_link=${redirectUrl}&cluster=mainnet-beta`;
    
    // Try to open deep link
    window.location.href = deepLink;
    
    // Fallback to app store if deep link fails
    setTimeout(() => {
      const isIOS = /iPad|iPhone|iPod/.test(navigator.userAgent);
      const storeUrl = isIOS 
        ? 'https://apps.apple.com/app/phantom-solana-wallet/id1598432977'
        : 'https://play.google.com/store/apps/details?id=app.phantom';
      window.open(storeUrl, '_blank');
    }, 2000);
  }, []);

  // Connect to Phantom wallet
  const connectWallet = useCallback(async (): Promise<void> => {
    if (isConnecting) return;
    
    setIsConnecting(true);
    
    try {
      // Check if on mobile
      if (isMobile()) {
        // Check if Phantom is installed
        const phantomProvider = getProvider();
        if (!phantomProvider) {
          openPhantomDeepLink();
          setIsConnecting(false);
          return;
        }
      }

      const phantomProvider = getProvider();
      
      if (!phantomProvider) {
        // Desktop: redirect to Phantom website
        window.open('https://phantom.app/', '_blank');
        setIsConnecting(false);
        return;
      }

      // Connect to wallet
      const response = await phantomProvider.connect();
      const walletAddress = response.publicKey.toString();
      
      setProvider(phantomProvider);
      setWalletAddress(walletAddress);
      setIsWalletConnected(true);
      
      // Store wallet address in localStorage for persistence
      localStorage.setItem('walletAddress', walletAddress);
      
      console.log('Wallet connected:', walletAddress);
      
    } catch (error) {
      console.error('Failed to connect wallet:', error);
      setIsWalletConnected(false);
      setWalletAddress(null);
    } finally {
      setIsConnecting(false);
    }
  }, [isConnecting, isMobile, getProvider, openPhantomDeepLink]);

  // Disconnect wallet
  const disconnectWallet = useCallback(async (): Promise<void> => {
    try {
      if (provider) {
        await provider.disconnect();
      }
      
      setProvider(null);
      setIsWalletConnected(false);
      setWalletAddress(null);
      
      // Clear localStorage
      localStorage.removeItem('walletAddress');
      
      console.log('Wallet disconnected');
      
    } catch (error) {
      console.error('Failed to disconnect wallet:', error);
    }
  }, [provider]);

  // Sign message with wallet
  const signMessage = useCallback(async (message: string): Promise<string | null> => {
    if (!provider || !isWalletConnected) {
      console.error('Wallet not connected');
      return null;
    }

    try {
      const encodedMessage = new TextEncoder().encode(message);
      const signedMessage = await provider.signMessage(encodedMessage);
      
      // Convert signature to base64
      const signature = Array.from(signedMessage.signature, byte => 
        String.fromCharCode(byte)
      ).join('');
      
      return btoa(signature);
    } catch (error) {
      console.error('Failed to sign message:', error);
      return null;
    }
  }, [provider, isWalletConnected]);

  // Original logout function
  const logout = useCallback(() => {
    Cookies.remove('SID');
    setSessionId(null);
    setIsAuthenticated(false);
    setUser(null);
    
    // Also disconnect wallet on logout
    disconnectWallet();
    
    navigate('/get-started');
  }, [disconnectWallet, navigate]);

  // Original fetch user data function
  const fetchUserData = useCallback(async () => {
    let isMounted = true;
    try {
      const sessionToken = Cookies.get('SID');
      if (!sessionToken) return;

      const result = await userApi.getMe();
      if (!isMounted) return;

      if (result.success) {
        setUser(result.data);
        setIsAuthenticated(true);
      } else {
        setIsAuthenticated(false);
        setSessionId(null);
        setUser(null);
        Cookies.remove('SID');
      }
    } catch (error) {
      if (!isMounted) return;
      setIsAuthenticated(false);
      setSessionId(null);
      setUser(null);
      Cookies.remove('SID');
    }
    return () => { isMounted = false; };
  }, []);

  // Enhanced login function
  const login = useCallback((newSessionId: string) => {
    Cookies.set('SID', newSessionId, { expires: 7 });
    setSessionId(newSessionId);
    setIsAuthenticated(true);
    fetchUserData();
    
    // Check if there's a stored redirect URL for claim pages
    const redirectUrl = localStorage.getItem('redirectAfterLogin');
    if (redirectUrl && redirectUrl.startsWith('/claim')) {
      localStorage.removeItem('redirectAfterLogin');
      navigate('/tweet-mining'); 
    }
  }, [fetchUserData, navigate]);

  // Initialize wallet connection on mount
  useEffect(() => {
    const initializeWallet = async () => {
      const phantomProvider = getProvider();
      if (phantomProvider) {
        setProvider(phantomProvider);
        
        // Check if wallet was previously connected
        const savedWalletAddress = localStorage.getItem('walletAddress');
        if (savedWalletAddress && phantomProvider.isConnected) {
          setWalletAddress(savedWalletAddress);
          setIsWalletConnected(true);
        }

        // Listen for wallet events
        const handleConnect = (publicKey: any) => {
          setWalletAddress(publicKey.toString());
          setIsWalletConnected(true);
          localStorage.setItem('walletAddress', publicKey.toString());
          console.log('Wallet connected via event:', publicKey.toString());
        };

        const handleDisconnect = () => {
          setWalletAddress(null);
          setIsWalletConnected(false);
          localStorage.removeItem('walletAddress');
          console.log('Wallet disconnected via event');
        };

        phantomProvider.on('connect', handleConnect);
        phantomProvider.on('disconnect', handleDisconnect);

        // Cleanup function
        return () => {
          phantomProvider.off('connect', handleConnect);
          phantomProvider.off('disconnect', handleDisconnect);
        };
      }
    };

    initializeWallet();
  }, [getProvider]);

  // Handle wallet callback from mobile deep link
  useEffect(() => {
    const handleWalletCallback = () => {
      const urlParams = new URLSearchParams(window.location.search);
      const phantomEncryptionPublicKey = urlParams.get('phantom_encryption_public_key');
      const nonce = urlParams.get('nonce');
      const data = urlParams.get('data');
      
      if (phantomEncryptionPublicKey && nonce && data) {
        // Handle the wallet connection response from mobile
        console.log('Received wallet callback from mobile');
        // You would decrypt and process the data here
        // This is a simplified version - you'd need proper decryption logic
      }
    };

    if (window.location.pathname === '/wallet-callback') {
      handleWalletCallback();
    }
  }, []);

  // Original auth initialization effects
  useEffect(() => {
    setAuthErrorHandler(() => {
      if (window.location.pathname !== '/' && !window.location.pathname.includes('get-started')) {
        window.location.href = '/get-started';
      }
      logout();
    });

    const existingSessionId = Cookies.get('SID');
    if (existingSessionId && !isAuthenticated) {
      setSessionId(existingSessionId);
      fetchUserData();
    }
  }, [logout, fetchUserData, isAuthenticated]);

  useEffect(() => {
    const existingSessionId = Cookies.get('SID');
    if (!existingSessionId && isAuthenticated) {
      setIsAuthenticated(false);
      setUser(null);
      setSessionId(null);
    }
  }, [isAuthenticated]);

  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    const sid = params.get('SID');
    if (sid) {
      login(sid); 
    }
  }, [login]);

  const value: AuthContextState = {
    isAuthenticated,
    user,
    sessionId,
    isWalletConnected,
    walletAddress,
    isConnecting,
    login,
    logout,
    setUser,
    connectWallet,
    disconnectWallet,
    signMessage,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};