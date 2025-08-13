import React, { useMemo, useCallback } from "react";

import ResponsiveMenu from "../../components/ResponsiveMenu";
import WalletDisplay from "../../components/WalletDisplay";
import { useAuth } from "../../contexts/AuthContext";
import { useWalletContext } from "../../contexts/WalletContext";
import { useAppContext } from "../../contexts/AppContext";
import './index.css';
// icons
import { ReactComponent as IconLeftLogo } from "../../svgs/logo-left.svg";

// Type definitions
interface MiningProgress {
  percentage: number;
  mined: number;
  total: number;
}

interface UserProfile {
  tweets?: number;
  reward_balance?: number;
  claimable_rewards?: number;
}

interface TokenInfo {
  balance_ui?: number;
}

interface MiningStatus {
  current_phase?: number;
  total_phase1_mined?: number;
  pending_rewards?: number;
}

interface User {
  twitter_username?: string;
}

// Component prop interfaces
interface ErrorAlertProps {
  error: string;
  onRetry: () => void;
}

interface UserStatsProps {
  userProfile: UserProfile;
  tokenInfo?: TokenInfo | null;
  miningStatus?: MiningStatus;
}

interface MiningProgressProps {
  miningStatus: MiningStatus | null;
  percentage: number;
  mined: number;
  total: number;
}

// Constants moved outside component to prevent re-creation
const PHASE_1_TARGET: number = 1000000; // 1M tweets target for Phase 1
const FALLBACK_VALUES: MiningProgress = {
  percentage: 0,
  mined: 0,
  total: PHASE_1_TARGET
};

// Extracted components for better organization
const LoadingSpinner: React.FC = () => (
  <div className="w-100 p-3 d-flex justify-content-center align-items-center" style={{ height: '100vh' }}>
    <div className="text-center">
      <div className="fs-3 mb-3">Loading Profile...</div>
      <div className="spinner-border text-primary" role="status">
        <span className="visually-hidden">Loading...</span>
      </div>
    </div>
  </div>
);

const ErrorAlert: React.FC<ErrorAlertProps> = ({ error, onRetry }) => (
  <div className="alert alert-danger d-flex align-items-center gap-2 mb-3" role="alert">
    <i className="bi bi-exclamation-circle-fill" />
    <div className="flex-grow-1">{error}</div>
    <button 
      className="btn btn-outline-danger btn-sm"
      onClick={onRetry}
      type="button"
    >
      Retry
    </button>
  </div>
);

const UserStats: React.FC<UserStatsProps> = ({ userProfile, tokenInfo, miningStatus }) => (
  <div className="d-flex gap-3 mt-3">
    <div className="bg-white-custom p-3 rounded flex-fill">
      <div className="fs-6 text-muted">Your Tweets</div>
      <div className="fs-4 fw-bold">
        {(userProfile?.tweets || 0).toLocaleString()}
      </div>
    </div>
    <div className="bg-white-custom p-3 rounded flex-fill">
      <div className="fs-6 text-muted">Claimable Tweets</div>
      <div className="fs-4 fw-bold">
        {(userProfile?.claimable_rewards || 0).toLocaleString()}
      </div>
    </div>
    <div className="bg-white-custom p-3 rounded flex-fill">
      <div className="fs-6 text-muted">Reward Balance</div>
      <div className="fs-4 fw-bold">
        {(userProfile?.reward_balance || 0).toFixed(4)} SNAKE
      </div>
    </div>
    {tokenInfo && (
      <div className="bg-white-custom p-3 rounded flex-fill">
        <div className="fs-6 text-muted">Token Balance</div>
        <div className="fs-4 fw-bold">
          {(tokenInfo.balance_ui || 0).toFixed(4)} SNAKE
        </div>
      </div>
    )}
  </div>
);

const MiningProgressComponent: React.FC<MiningProgressProps> = ({ 
  miningStatus, 
  percentage, 
  mined, 
  total 
}) => (
  <div className="align-items-center py-3 custom-border-top w-100 mining-progress">
    <div className="bg-black p-2 px-4 text-light-green-950 rounded-3" style={{ minWidth: '180px' }}>
      <div className="fs-4 fw-bold">
        {miningStatus?.current_phase === 1 ? 'PHASE 1:' : 'PHASE 2:'}
      </div>
      <div className="fs-6">MINING EPOCH</div>
    </div>
    <div className="custom-border-left mx-3 divide-y" style={{ height: '70px' }} />
    <div className="d-flex align-items-center">
      <div className="progress-bar-container rounded me-3">
        <div 
          className="progress-bar-fill" 
          style={{ width: `${percentage}%` }}
        />
      </div>
      <div className="progress-container">
        <div
          className="progress-fill"
          style={{ '--progress': `${percentage}` } as React.CSSProperties}
        />
      </div>
      <div className="ps-3 fs-6">
        <strong>MINING PROGRESS:</strong> {mined.toLocaleString()} OUT OF {total.toLocaleString()} TWEETS MINED
      </div>
    </div>
  </div>
);

const Home: React.FC = () => {
  // Custom hooks with proper typing
  const { user }: { user: User | null } = useAuth();
  const { connected }: { connected: boolean } = useWalletContext();
  const { 
    miningStatus, 
    userProfile, 
    tokenInfo, 
    loading, 
    error, 
    refreshAllAppData 
  }: {
    miningStatus: MiningStatus | null;
    userProfile: UserProfile | null;
    tokenInfo: TokenInfo | null;
    loading: boolean;
    error: string | null;
    refreshAllAppData: () => void;
  } = useAppContext();

  // Memoized mining progress calculation with proper typing
  const miningProgress: MiningProgress = useMemo(() => {
    if (!miningStatus) return FALLBACK_VALUES;
    
    const total: number = PHASE_1_TARGET;
    // TODO: Replace hardcoded values with real data
    const mined: number = 30000; // miningStatus.total_phase1_mined || 0;
    const percentage: number = 50; // Math.min((mined / total) * 100, 100);
    
    return { percentage, mined, total };
  }, [miningStatus]);

  // Memoized retry handler to prevent unnecessary re-renders
  const handleRetry = useCallback((): void => {
    refreshAllAppData();
  }, [refreshAllAppData]);

  // Early returns for loading state
  if (loading) {
    return <LoadingSpinner />;
  }

  const { percentage, mined, total }: MiningProgress = miningProgress;

  return (
    <div className="w-100 p-3" style={{ height: '100vh' }}>
      <div className="d-flex gap-3" style={{ height: "calc(100vh - 60px)", paddingTop: '35px' }}>
        {/* Menu */}
        <ResponsiveMenu />
               
        <div className="custom-content">
          <header className="w-100">
            <div className="d-flex justify-content-between align-items-center">
              <h1 className="fs-1 m-0" style={{ lineHeight: 'normal' }}>
                DASHBOARD
              </h1>
              <div className="text-end">
                <div className="fs-6 text-muted">
                  Connected: @{user?.twitter_username || 'Not authenticated'}
                </div>
              </div>
            </div>
          </header>

          {/* Error State */}
          {error && <ErrorAlert error={error} onRetry={handleRetry} />}
          
          <main className="custom-border-y custom-content-height d-flex flex-column">
            {/* Welcome Section */}
            <section className="h-100 d-flex flex-column justify-content-center">
              <div className="text-center fs-2 fs-lg-4 fs-xl-7 fw-bold">
                Play
              </div>
              <div className="d-flex justify-content-center align-items-center">
                <IconLeftLogo aria-label="Snake AI Logo" />
                <span className="fs-1 fs-lg-2 fs-xl-1 fw-bold">SNAKE.AI</span>
              </div>
              <div>
                <WalletDisplay />
              </div>
            </section>

            {/* Mining Progress */}
            <MiningProgressComponent 
              miningStatus={miningStatus}
              percentage={percentage}
              mined={mined}
              total={total}
            />

            {/* User Stats */}
            {connected && userProfile && (
              <UserStats userProfile={userProfile} tokenInfo={tokenInfo} miningStatus={miningStatus ?? undefined} />
            )}
          </main>
        </div>
      </div>
    </div>
  );
};

export default Home;