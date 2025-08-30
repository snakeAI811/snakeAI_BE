import React, { useState, useEffect } from 'react';
import { useWalletContext } from '../contexts/WalletContext';
import { userApi } from '../pages/patron/services/apiService';

interface PendingRewards {
  pending_rewards: number;
  total_reward_entries: number;
  can_sync: boolean;
  wallet_address: string | null;
  sync_required_before_claim: boolean;
}

interface RewardSyncComponentProps {
  onSyncSuccess?: () => void;
  onSyncError?: (error: string) => void;
}

const RewardSyncComponent: React.FC<RewardSyncComponentProps> = ({
  onSyncSuccess,
  onSyncError
}) => {
  const { connected, publicKey, signMessage } = useWalletContext();
  const [pendingRewards, setPendingRewards] = useState<PendingRewards | null>(null);
  const [loading, setLoading] = useState(false);
  const [syncing, setSyncing] = useState(false);
  const [syncStatus, setSyncStatus] = useState<string | null>(null);

  const fetchPendingRewards = async () => {
    if (!connected) return;

    try {
      setLoading(true);
      const response = await userApi.getPendingRewards();
      if (response.success && response.data) {
        setPendingRewards(response.data);
      }
    } catch (err) {
      console.error('Error fetching pending rewards:', err);
      onSyncError?.('Failed to fetch pending rewards');
    } finally {
      setLoading(false);
    }
  };

  const handleSyncRewards = async () => {
    if (!connected || !publicKey || !signMessage) {
      onSyncError?.('Wallet not connected or does not support message signing');
      return;
    }

    try {
      setSyncing(true);
      setSyncStatus('Preparing sync transaction...');

      // Create a message to sign for verification
      const message = `Sync rewards for wallet: ${publicKey} at ${new Date().toISOString()}`;
      const encodedMessage = new TextEncoder().encode(message);
      
      // Sign the message
      const signature = await signMessage(encodedMessage);
      const signatureBase64 = btoa(String.fromCharCode(...Array.from(signature)));

      setSyncStatus('Submitting sync request...');

      // Call the sync API
      const response = await userApi.syncRewardsToChain({
        user_signature: signatureBase64
      });

      if (response.success && response.data) {
        setSyncStatus('Sync completed successfully!');
        onSyncSuccess?.();
        
        // Refresh pending rewards
        setTimeout(() => {
          fetchPendingRewards();
          setSyncStatus(null);
        }, 2000);
      } else {
        throw new Error(response.error || 'Sync failed');
      }
    } catch (err: any) {
      console.error('Error syncing rewards:', err);
      setSyncStatus(null);
      onSyncError?.(err.message || 'Failed to sync rewards');
    } finally {
      setSyncing(false);
    }
  };

  useEffect(() => {
    if (connected) {
      fetchPendingRewards();
    }
  }, [connected]);

  if (!connected) {
    return (
      <div className="card">
        <div className="card-body text-center">
          <h5 className="card-title">🔄 Reward Sync</h5>
          <p className="text-muted">Please connect your wallet to view pending rewards</p>
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="card">
        <div className="card-body text-center">
          <h5 className="card-title">🔄 Reward Sync</h5>
          <div className="spinner-border text-primary" role="status">
            <span className="visually-hidden">Loading...</span>
          </div>
          <p className="mt-2">Loading pending rewards...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="card">
      <div className="card-body">
        <h5 className="card-title">🔄 Reward Sync</h5>
        
        {pendingRewards && (
          <>
            <div className="row mb-3">
              <div className="col-md-6">
                <div className="text-center">
                  <div className="fs-4 fw-bold text-warning">
                    {pendingRewards.pending_rewards.toLocaleString()}
                  </div>
                  <small className="text-muted">Pending Rewards (SNAKE)</small>
                </div>
              </div>
              <div className="col-md-6">
                <div className="text-center">
                  <div className="fs-4 fw-bold text-info">
                    {pendingRewards.total_reward_entries}
                  </div>
                  <small className="text-muted">Reward Entries</small>
                </div>
              </div>
            </div>

            {syncStatus && (
              <div className="alert alert-info">
                <div className="d-flex align-items-center">
                  {syncing && (
                    <div className="spinner-border spinner-border-sm me-2" role="status">
                      <span className="visually-hidden">Loading...</span>
                    </div>
                  )}
                  {syncStatus}
                </div>
              </div>
            )}

            <div className="alert alert-warning">
              <strong>⚠️ Important:</strong> You must sync your off-chain rewards to the blockchain before claiming. 
              This operation requires a small transaction fee that you will pay.
            </div>

            {pendingRewards.can_sync ? (
              <div className="d-grid gap-2">
                <button
                  className="btn btn-primary"
                  onClick={handleSyncRewards}
                  disabled={syncing}
                >
                  {syncing ? (
                    <>
                      <span className="spinner-border spinner-border-sm me-2" role="status"></span>
                      Syncing Rewards...
                    </>
                  ) : (
                    <>🔄 Sync {pendingRewards.pending_rewards.toLocaleString()} SNAKE to Blockchain</>
                  )}
                </button>
                <small className="text-muted text-center">
                  You will pay the transaction fee for this operation
                </small>
              </div>
            ) : (
              <div className="alert alert-info">
                <strong>ℹ️ No rewards to sync</strong><br />
                Mine more tweets to accumulate rewards that can be synced to the blockchain.
              </div>
            )}

            <div className="mt-3">
              <button
                className="btn btn-outline-secondary btn-sm"
                onClick={fetchPendingRewards}
                disabled={loading}
              >
                🔄 Refresh
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default RewardSyncComponent;