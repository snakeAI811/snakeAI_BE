import React, { useState, useEffect, useCallback } from 'react';
import { useWalletContext } from '../../contexts/WalletContext';
import { tokenApi, userApi, roleApi } from '../patron/services/apiService';
import { Connection, PublicKey, Transaction } from '@solana/web3.js';

interface StakingData {
  balance: number;
  locked: number;
  staked: number;
  rewards: number;
  lockEndDate?: string;
  miningCount: number;
  totalPhase1Mined: number;
  totalPhase2Mined: number;
  currentPhase: 1 | 2;
  userRole: 'none' | 'staker' | 'patron';
  patronStatus: 'none' | 'Applied' | 'Approved' | 'Revoked';
  tweetMiningStatus: {
    totalTweets: number;
    pendingRewards: number;
    totalRewardsClaimed: number;
  };
  vestingInfo?: {
    totalAmount: number;
    vestedAmount: number;
    claimableAmount: number;
    yieldAccrued: number;
    isActive: boolean;
  };
}

interface SimpleStakingDashboardProps {
  connection: any;
}

const SimpleStakingDashboard: React.FC<SimpleStakingDashboardProps> = ({ connection }) => {
  const { publicKey, connected } = useWalletContext();
  const [stakingData, setStakingData] = useState<StakingData | null>(null);
  const [loading, setLoading] = useState(false);
  const [claiming, setClaiming] = useState(false);
  const [unlocking, setUnlocking] = useState(false);
  const [staking, setStaking] = useState(false);
  const [showStakeModal, setShowStakeModal] = useState(false);
  const [stakeAmount, setStakeAmount] = useState('');
  const [stakeDuration, setStakeDuration] = useState(3);

  const fetchStakingData = useCallback(async () => {
    if (!connected || !publicKey) return;
    
    try {
      setLoading(true);
      
      // Fetch token info, mining status, and user role
      const tokenResponse = await tokenApi.getTokenInfo();
      const miningResponse = await tokenApi.getMiningStatus();
      const tweetMiningResponse = await userApi.getTweetMiningStatus();
      const vestingResponse = await tokenApi.getVestingInfo();
      const roleResponse = await roleApi.getUserRole();
      
      if (tokenResponse.success && miningResponse.success && tweetMiningResponse.success) {
        // Parse user role
        let userRole: 'none' | 'staker' | 'patron' = 'none';
        let patronStatus: 'none' | 'Applied' | 'Approved' | 'Revoked' = 'none';
        
        if (roleResponse.success && roleResponse.data) {
          const roleData = roleResponse.data;
          userRole = (roleData.role?.toLowerCase() || 'none') as 'none' | 'staker' | 'patron';
          patronStatus = (roleData.status || 'none') as 'none' | 'Applied' | 'Approved' | 'Revoked';
        }

        const stakingData: StakingData = {
          balance: tokenResponse.data?.balance || 0,
          locked: tokenResponse.data?.locked || 0,
          staked: tokenResponse.data?.staked || 0,
          rewards: tokenResponse.data?.rewards || 0,
          lockEndDate: tokenResponse.data?.lockEndDate,
          miningCount: tokenResponse.data?.mining_count || 0,
          totalPhase1Mined: miningResponse.data?.total_phase1_mined || 0,
          totalPhase2Mined: miningResponse.data?.total_phase2_mined || 0,
          currentPhase: miningResponse.data?.current_phase || 1,
          userRole,
          patronStatus,
          tweetMiningStatus: {
            totalTweets: tweetMiningResponse.data?.total_tweets || 0,
            pendingRewards: tweetMiningResponse.data?.pending_rewards || 0,
            totalRewardsClaimed: tweetMiningResponse.data?.total_rewards_claimed || 0,
          },
          vestingInfo: vestingResponse.success ? {
            totalAmount: vestingResponse.data?.totalAmount || 0,
            vestedAmount: vestingResponse.data?.vestedAmount || 0,
            claimableAmount: vestingResponse.data?.claimableAmount || 0,
            yieldAccrued: vestingResponse.data?.yieldAccrued || 0,
            isActive: vestingResponse.data?.isActive || false,
          } : undefined,
        };
        
        setStakingData(stakingData);
      }
    } catch (error) {
      console.error('Error fetching staking data:', error);
    } finally {
      setLoading(false);
    }
  }, [connected, publicKey]);

  useEffect(() => {
    fetchStakingData();
  }, [fetchStakingData]);

  const handleClaimYield = async () => {
    if (!connected || !publicKey) return;
    
    try {
      setClaiming(true);
      console.log('🔄 Starting claim yield process...');
      
      const response = await tokenApi.claimYield();
      
      if (response.success) {
        if (response.data && typeof response.data === 'string') {
          console.log('🔐 Claim requires wallet signature');
          
          // Decode and sign transaction (same logic as staking)
          const transactionBytes = Uint8Array.from(atob(response.data), c => c.charCodeAt(0));
          const transaction = Transaction.from(transactionBytes);
          
          const { blockhash } = await connection.getLatestBlockhash('confirmed');
          transaction.recentBlockhash = blockhash;
          transaction.feePayer = new PublicKey(publicKey);
          
          if (typeof window !== 'undefined' && (window as any).solana) {
            const signedTransaction = await (window as any).solana.signTransaction(transaction);
            const signature = await connection.sendRawTransaction(signedTransaction.serialize(), {
              skipPreflight: false,
              preflightCommitment: 'confirmed'
            });
            
            await connection.confirmTransaction({
              signature,
              blockhash: transaction.recentBlockhash!,
              lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight
            }, 'confirmed');
            
            alert(`Yield claimed successfully! Transaction: ${signature}`);
          } else {
            throw new Error('Phantom wallet not found');
          }
        } else {
          alert('Yield claimed successfully!');
        }
        await fetchStakingData();
      } else {
        alert(`Failed to claim yield: ${response.error}`);
      }
    } catch (error) {
      console.error('Error claiming yield:', error);
      const errorMessage = error instanceof Error ? error.message : 'Failed to claim yield';
      alert(errorMessage);
    } finally {
      setClaiming(false);
    }
  };

  const handleUnlockTokens = async () => {
    if (!connected || !publicKey) return;
    
    try {
      setUnlocking(true);
      console.log('🔄 Starting unlock tokens process...');
      
      const response = await tokenApi.unlockTokens();
      
      if (response.success) {
        if (response.data && typeof response.data === 'string') {
          console.log('🔐 Unlock requires wallet signature');
          
          // Decode and sign transaction
          const transactionBytes = Uint8Array.from(atob(response.data), c => c.charCodeAt(0));
          const transaction = Transaction.from(transactionBytes);
          
          const { blockhash } = await connection.getLatestBlockhash('confirmed');
          transaction.recentBlockhash = blockhash;
          transaction.feePayer = new PublicKey(publicKey);
          
          if (typeof window !== 'undefined' && (window as any).solana) {
            const signedTransaction = await (window as any).solana.signTransaction(transaction);
            const signature = await connection.sendRawTransaction(signedTransaction.serialize(), {
              skipPreflight: false,
              preflightCommitment: 'confirmed'
            });
            
            await connection.confirmTransaction({
              signature,
              blockhash: transaction.recentBlockhash!,
              lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight
            }, 'confirmed');
            
            alert(`Tokens unlocked successfully! Transaction: ${signature}`);
          } else {
            throw new Error('Phantom wallet not found');
          }
        } else {
          alert('Tokens unlocked successfully!');
        }
        await fetchStakingData();
      } else {
        alert(`Failed to unlock tokens: ${response.error}`);
      }
    } catch (error) {
      console.error('Error unlocking tokens:', error);
      const errorMessage = error instanceof Error ? error.message : 'Failed to unlock tokens';
      alert(errorMessage);
    } finally {
      setUnlocking(false);
    }
  };

  const handleStakeTokens = async () => {
    if (!connected || !publicKey || !stakeAmount) return;
    
    const amount = parseFloat(stakeAmount);
    if (isNaN(amount) || amount <= 0) {
      alert('Please enter a valid amount');
      return;
    }
    
    if (amount > (stakingData?.balance || 0)) {
      alert('Insufficient balance');
      return;
    }

    // Validate user role and duration
    if (stakingData?.userRole === 'none') {
      alert('You must select a role (Staker or Patron) before staking tokens. Please visit the Patron Framework page to select your role.');
      return;
    }

    if (stakingData?.userRole === 'staker' && stakeDuration !== 3) {
      alert('Stakers can only lock tokens for 3 months.');
      return;
    }

    if (stakingData?.userRole === 'patron' && stakeDuration !== 6) {
      alert('Patrons can only lock tokens for 6 months.');
      return;
    }

    if (stakingData?.userRole === 'patron' && stakingData?.patronStatus !== 'Approved') {
      alert('Only approved patrons can stake tokens. Your patron status is: ' + stakingData?.patronStatus);
      return;
    }
    
    try {
      setStaking(true);
      console.log(`🔄 Starting stake process: ${amount} SNAKE for ${stakeDuration} months`);
      
      const response = await tokenApi.lockTokens(amount, stakeDuration);
      
      if (response.success && response.data) {
        if (response.requiresWalletSignature) {
          console.log('🔐 Transaction requires wallet signature');
          
          // Decode the base64 transaction
          const transactionBytes = Uint8Array.from(atob(response.data), c => c.charCodeAt(0));
          const transaction = Transaction.from(transactionBytes);
          
          console.log('🔍 Transaction decoded successfully');
          
          // Update with fresh blockhash
          const { blockhash } = await connection.getLatestBlockhash('confirmed');
          transaction.recentBlockhash = blockhash;
          transaction.feePayer = new PublicKey(publicKey);
          
          // Sign and send transaction using Phantom wallet
          if (typeof window !== 'undefined' && (window as any).solana) {
            console.log('🔐 Requesting wallet signature...');
            
            const signedTransaction = await (window as any).solana.signTransaction(transaction);
            console.log('✅ Transaction signed by wallet');
            
            const signature = await connection.sendRawTransaction(signedTransaction.serialize(), {
              skipPreflight: false,
              preflightCommitment: 'confirmed'
            });
            console.log('📤 Transaction submitted with signature:', signature);
            
            await connection.confirmTransaction({
              signature,
              blockhash: transaction.recentBlockhash!,
              lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight
            }, 'confirmed');
            
            console.log('✅ Transaction confirmed');
            alert(`Tokens staked successfully! Transaction: ${signature}`);
          } else {
            throw new Error('Phantom wallet not found. Please install Phantom wallet.');
          }
        } else {
          alert('Tokens staked successfully!');
        }
        
        setShowStakeModal(false);
        setStakeAmount('');
        await fetchStakingData(); // Refresh data
      } else {
        alert(`Failed to stake tokens: ${response.error}`);
      }
    } catch (error) {
      console.error('Error staking tokens:', error);
      const errorMessage = error instanceof Error ? error.message : 'Failed to stake tokens';
      alert(errorMessage);
    } finally {
      setStaking(false);
    }
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return 'N/A';
    return new Date(dateString).toLocaleDateString();
  };

  const getTimeRemaining = (endDate?: string) => {
    if (!endDate) return 'N/A';
    
    const now = new Date().getTime();
    const end = new Date(endDate).getTime();
    const remaining = end - now;
    
    if (remaining <= 0) return 'Unlocked';
    
    const days = Math.floor(remaining / (1000 * 60 * 60 * 24));
    const hours = Math.floor((remaining % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    
    return `${days}d ${hours}h`;
  };

  if (!connected || !publicKey) {
    return (
      <div className="d-flex justify-content-center align-items-center h-100">
        <div className="text-center">
          <h4 className="mb-3">🏦 Staking Dashboard</h4>
          <p className="text-muted">Please connect your wallet to view staking information.</p>
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="d-flex justify-content-center align-items-center h-100">
        <div className="text-center">
          <div className="spinner-border text-primary mb-3" role="status">
            <span className="visually-hidden">Loading...</span>
          </div>
          <p>Loading staking data...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="w-100 h-100 overflow-auto">
      {/* Header */}
      <div className="d-flex justify-content-between align-items-center mb-4 custom-border-bottom pb-3">
        <div>
          <small className="text-muted">Wallet: {publicKey}</small>
        </div>
        <div className="d-flex gap-2">
          <span className="badge bg-success">Connected</span>
          <span className="badge bg-primary">Phase {stakingData?.currentPhase || 1}</span>
          {stakingData?.userRole && stakingData.userRole !== 'none' && (
            <span className={`badge ${stakingData.userRole === 'staker' ? 'bg-info' : 'bg-warning'}`}>
              {stakingData.userRole === 'staker' ? '🏦 Staker' : '👑 Patron'}
              {stakingData.userRole === 'patron' && ` (${stakingData.patronStatus})`}
            </span>
          )}
          {stakingData?.userRole === 'none' && (
            <span className="badge bg-secondary">No Role</span>
          )}
        </div>
      </div>

      {/* Staking Overview */}
      <div className="row mb-4">
        <div className="col-6 col-lg-3 mb-3">
          <div className="card h-100">
            <div className="card-body text-center">
              <div className="text-primary mb-2">
                <i className="bi bi-lock-fill" style={{ fontSize: '2rem' }}></i>
              </div>
              <h6 className="card-title text-muted mb-1">Locked Tokens</h6>
              <h4 className="card-text text-primary mb-0">
                {stakingData?.locked.toLocaleString() || 0}
              </h4>
              <small className="text-muted">SNAKE</small>
            </div>
          </div>
        </div>

        <div className="col-6 col-lg-3 mb-3">
          <div className="card h-100">
            <div className="card-body text-center">
              <div className="text-success mb-2">
                <i className="bi bi-award-fill" style={{ fontSize: '2rem' }}></i>
              </div>
              <h6 className="card-title text-muted mb-1">Available Rewards</h6>
              <h4 className="card-text text-success mb-0">
                {stakingData?.rewards.toLocaleString() || 0}
              </h4>
              <small className="text-muted">SNAKE</small>
            </div>
          </div>
        </div>

        <div className="col-6 col-lg-3 mb-3">
          <div className="card h-100">
            <div className="card-body text-center">
              <div className="text-warning mb-2">
                <i className="bi bi-clock-fill" style={{ fontSize: '2rem' }}></i>
              </div>
              <h6 className="card-title text-muted mb-1">Lock Status</h6>
              <h6 className="card-text text-warning mb-0">
                {getTimeRemaining(stakingData?.lockEndDate)}
              </h6>
              <small className="text-muted">Remaining</small>
            </div>
          </div>
        </div>

        <div className="col-6 col-lg-3 mb-3">
          <div className="card h-100">
            <div className="card-body text-center">
              <div className="text-info mb-2">
                <i className="bi bi-wallet2" style={{ fontSize: '2rem' }}></i>
              </div>
              <h6 className="card-title text-muted mb-1">Token Balance</h6>
              <h4 className="card-text text-info mb-0">
                {stakingData?.balance.toLocaleString() || 0}
              </h4>
              <small className="text-muted">SNAKE</small>
            </div>
          </div>
        </div>
      </div>

      {/* Mining Statistics */}
      <div className="mb-4 custom-border-bottom pb-3">
        <h5 className="mb-3">📊 Mining Statistics</h5>
        <div className="row">
          <div className="col-12 col-md-4 mb-3">
            <div className="d-flex justify-content-between align-items-center py-2">
              <span className="text-muted">Total Tweets:</span>
              <strong>{stakingData?.tweetMiningStatus.totalTweets || 0}</strong>
            </div>
            <div className="d-flex justify-content-between align-items-center py-2">
              <span className="text-muted">Phase 1 Mined:</span>
              <strong>{stakingData?.totalPhase1Mined.toLocaleString() || 0} SNAKE</strong>
            </div>
          </div>
          <div className="col-12 col-md-4 mb-3">
            <div className="d-flex justify-content-between align-items-center py-2">
              <span className="text-muted">Phase 2 Mined:</span>
              <strong>{stakingData?.totalPhase2Mined.toLocaleString() || 0} SNAKE</strong>
            </div>
            <div className="d-flex justify-content-between align-items-center py-2">
              <span className="text-muted">Pending Rewards:</span>
              <strong>{stakingData?.tweetMiningStatus.pendingRewards.toLocaleString() || 0} SNAKE</strong>
            </div>
          </div>
          <div className="col-12 col-md-4 mb-3">
            <div className="d-flex justify-content-between align-items-center py-2">
              <span className="text-muted">Total Claimed:</span>
              <strong>{stakingData?.tweetMiningStatus.totalRewardsClaimed.toLocaleString() || 0} SNAKE</strong>
            </div>
            <div className="d-flex justify-content-between align-items-center py-2">
              <span className="text-muted">Lock End Date:</span>
              <strong>{formatDate(stakingData?.lockEndDate)}</strong>
            </div>
          </div>
        </div>
      </div>

      {/* Vesting Information */}
      {stakingData?.vestingInfo?.isActive && (
        <div className="mb-4 custom-border-bottom pb-3">
          <h5 className="mb-3">💰 Vesting Information</h5>
          <div className="row">
            <div className="col-12 col-md-6 mb-3">
              <div className="d-flex justify-content-between align-items-center py-2">
                <span className="text-muted">Total Vesting Amount:</span>
                <strong>{stakingData.vestingInfo.totalAmount.toLocaleString()} SNAKE</strong>
              </div>
              <div className="d-flex justify-content-between align-items-center py-2">
                <span className="text-muted">Vested Amount:</span>
                <strong>{stakingData.vestingInfo.vestedAmount.toLocaleString()} SNAKE</strong>
              </div>
            </div>
            <div className="col-12 col-md-6 mb-3">
              <div className="d-flex justify-content-between align-items-center py-2">
                <span className="text-muted">Claimable Amount:</span>
                <strong>{stakingData.vestingInfo.claimableAmount.toLocaleString()} SNAKE</strong>
              </div>
              <div className="d-flex justify-content-between align-items-center py-2">
                <span className="text-muted">Yield Accrued:</span>
                <strong>{stakingData.vestingInfo.yieldAccrued.toLocaleString()} SNAKE</strong>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Actions */}
      <div className="custom-border-top pt-3">
        <h5 className="mb-3">⚡ Actions</h5>
        <div className="d-flex flex-wrap gap-2">
          {/* Stake Tokens - Only show if user has balance and no existing stake */}
          {(!stakingData?.locked || stakingData.locked === 0) && (stakingData?.balance || 0) > 0 && (
            <button
              onClick={() => {
                // Set default duration based on user role
                if (stakingData?.userRole === 'staker') {
                  setStakeDuration(3);
                } else if (stakingData?.userRole === 'patron') {
                  setStakeDuration(6);
                } else {
                  setStakeDuration(0);
                }
                setShowStakeModal(true);
              }}
              disabled={staking}
              className="btn btn-warning"
            >
              <i className="bi bi-lock-fill me-2"></i>
              Stake Tokens
            </button>
          )}

          <button
            onClick={handleClaimYield}
            disabled={claiming || (stakingData?.rewards || 0) <= 0}
            className="btn btn-primary"
          >
            {claiming ? (
              <>
                <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                Claiming...
              </>
            ) : (
              <>
                <i className="bi bi-award-fill me-2"></i>
                Claim {stakingData?.rewards.toLocaleString() || 0} SNAKE Rewards
              </>
            )}
          </button>
          
          {stakingData?.lockEndDate && getTimeRemaining(stakingData.lockEndDate) === 'Unlocked' && (
            <button
              onClick={handleUnlockTokens}
              disabled={unlocking || (stakingData?.locked || 0) <= 0}
              className="btn btn-success"
            >
              {unlocking ? (
                <>
                  <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                  Unlocking...
                </>
              ) : (
                <>
                  <i className="bi bi-unlock-fill me-2"></i>
                  Unlock {stakingData?.locked.toLocaleString() || 0} SNAKE
                </>
              )}
            </button>
          )}
          
          <button
            onClick={fetchStakingData}
            disabled={loading}
            className="btn btn-outline-secondary"
          >
            {loading ? (
              <>
                <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                Refreshing...
              </>
            ) : (
              <>
                <i className="bi bi-arrow-clockwise me-2"></i>
                Refresh Data
              </>
            )}
          </button>
        </div>
      </div>

      {/* Stake Modal */}
      {showStakeModal && (
        <div className="modal d-block" style={{ backgroundColor: 'rgba(0,0,0,0.5)' }}>
          <div className="modal-dialog modal-dialog-centered">
            <div className="modal-content">
              <div className="modal-header">
                <h5 className="modal-title">🔒 Stake Tokens</h5>
                <button
                  type="button"
                  className="btn-close"
                  onClick={() => setShowStakeModal(false)}
                ></button>
              </div>
              <div className="modal-body">
                <div className="mb-3">
                  <label className="form-label">Amount to Stake</label>
                  <div className="input-group">
                    <input
                      type="number"
                      className="form-control"
                      value={stakeAmount}
                      onChange={(e) => setStakeAmount(e.target.value)}
                      placeholder="Enter amount"
                      max={stakingData?.balance || 0}
                    />
                    <span className="input-group-text">SNAKE</span>
                  </div>
                  <div className="form-text">
                    Available: {stakingData?.balance.toLocaleString() || 0} SNAKE
                  </div>
                </div>
                
                <div className="mb-3">
                  <label className="form-label">Lock Duration</label>
                  <select
                    className="form-select"
                    value={stakeDuration}
                    onChange={(e) => setStakeDuration(parseInt(e.target.value))}
                  >
                    {stakingData?.userRole === 'staker' && (
                      <option value={3}>3 months (5% APY) - Staker</option>
                    )}
                    {stakingData?.userRole === 'patron' && stakingData?.patronStatus === 'Approved' && (
                      <option value={6}>6 months (7% APY) - Patron</option>
                    )}
                    {stakingData?.userRole === 'none' && (
                      <option value={0} disabled>Please select a role first</option>
                    )}
                    {stakingData?.userRole === 'patron' && stakingData?.patronStatus !== 'Approved' && (
                      <option value={0} disabled>Patron approval required</option>
                    )}
                  </select>
                  {stakingData?.userRole === 'none' && (
                    <div className="form-text text-warning">
                      You must select a role (Staker or Patron) before staking. Visit the Patron Framework page to choose your role.
                    </div>
                  )}
                </div>

                <div className="alert alert-info">
                  <small>
                    <strong>Note:</strong> Staked tokens will be locked for the selected duration. 
                    You'll earn rewards based on the APY and can claim them periodically.
                  </small>
                </div>
              </div>
              <div className="modal-footer">
                <button
                  type="button"
                  className="btn btn-secondary"
                  onClick={() => setShowStakeModal(false)}
                >
                  Cancel
                </button>
                <button
                  type="button"
                  className="btn btn-warning"
                  onClick={handleStakeTokens}
                  disabled={staking || !stakeAmount || parseFloat(stakeAmount) <= 0}
                >
                  {staking ? (
                    <>
                      <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                      Staking...
                    </>
                  ) : (
                    'Stake Tokens'
                  )}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default SimpleStakingDashboard;
