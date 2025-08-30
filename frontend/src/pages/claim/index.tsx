import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import ResponsiveMenu from "../../components/ResponsiveMenu";
import BatchClaimComponent from "../../components/BatchClaimComponent";
import RewardSyncComponent from "../../components/RewardSyncComponent";
import { useWalletContext } from '../../contexts/WalletContext';
import { tokenApi, userApi } from '../patron/services/apiService';

interface ClaimPageProps {
  page_number?: number;
}

interface MiningStats {
  total_tweets: number;
  phase1_count: number;
  phase2_count: number;
  pending_rewards: number;
  total_rewards_claimed: number;
  accumulated_rewards: number;
  current_phase: any;
}

function ClaimPage({ page_number = 1 }: ClaimPageProps) {
  const navigate = useNavigate();
  const { connected, publicKey } = useWalletContext();

  const [walletBalance, setWalletBalance] = useState<number>(0);
  const [miningStats, setMiningStats] = useState<MiningStats | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const fetchWalletBalance = async () => {
    if (!connected) return;

    try {
      const response = await tokenApi.getTokenInfo();
      if (response.success && response.data) {
        setWalletBalance(response.data.balance);
      }
    } catch (err) {
      console.error('Error fetching wallet balance:', err);
    }
  };

  const fetchMiningStats = async () => {
    if (!connected) return;

    try {
      setLoading(true);
      const response = await userApi.getTweetMiningStatus();
      if (response.success && response.data) {
        setMiningStats(response.data);
      }
    } catch (err) {
      console.error('Error fetching mining stats:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (connected) {
      fetchWalletBalance();
      fetchMiningStats();
    }
  }, [connected]);

  const handleClaimSuccess = () => {
    setSuccess('🎉 Tokens claimed successfully! Your wallet balance has been updated.');
    fetchWalletBalance();
    fetchMiningStats();
    
    // Clear success message after 5 seconds
    setTimeout(() => {
      setSuccess(null);
    }, 5000);
  };

  const handleClaimError = (errorMessage: string) => {
    setError(errorMessage);
    
    // Clear error message after 10 seconds
    setTimeout(() => {
      setError(null);
    }, 10000);
  };

  return (
    <div className="w-100 p-3" style={{ height: "100vh" }}>
      <div className="d-flex gap-4 justify-content-center" style={{ height: "calc(100vh-60px)", paddingTop: '55px' }}>
        
        <div className="custom-content">
          <div className="w-100">
            <div className="d-flex justify-content-between align-items-center">
              <div className="fs-1" style={{ lineHeight: 'normal' }}>🎁 Claim Your Mining Rewards</div>
            </div>
          </div>
          
          <div className="custom-border-y custom-content-height d-flex flex-column px-3">
            <div className="w-100" style={{ minHeight: '86vh' }}>

              {/* Success Message */}
              {success && (
                <div className="alert alert-success alert-dismissible fade show" role="alert">
                  {success}
                  <button type="button" className="btn-close" onClick={() => setSuccess(null)}></button>
                </div>
              )}

              {/* Error Message */}
              {error && (
                <div className="alert alert-danger alert-dismissible fade show" role="alert">
                  {error}
                  <button type="button" className="btn-close" onClick={() => setError(null)}></button>
                </div>
              )}

              {/* Wallet Status */}
              {connected && publicKey && (
                <div className="alert alert-success mb-4">
                  <div className="row">
                    <div className="col-md-6">
                      <strong>✅ Wallet Connected:</strong> {publicKey.slice(0, 8)}...{publicKey.slice(-8)}
                    </div>
                    <div className="col-md-6">
                      <strong>💰 Current Balance:</strong> {walletBalance.toLocaleString()} SNAKE tokens
                    </div>
                  </div>
                </div>
              )}

              {/* Mining Statistics */}
              {miningStats && (
                <div className="row mb-4">
                  <div className="col-md-12">
                    <div className="card">
                      <div className="card-body">
                        <h5 className="card-title">📊 Your Mining Statistics</h5>
                        <div className="row">
                          <div className="col-md-3">
                            <div className="text-center">
                              <div className="fs-4 fw-bold text-primary">{miningStats.total_tweets}</div>
                              <small className="text-muted">Total Tweets Mined</small>
                            </div>
                          </div>
                          <div className="col-md-3">
                            <div className="text-center">
                              <div className="fs-4 fw-bold text-success">{miningStats.phase1_count}</div>
                              <small className="text-muted">Phase 1 Tweets</small>
                            </div>
                          </div>
                          <div className="col-md-3">
                            <div className="text-center">
                              <div className="fs-4 fw-bold text-info">{miningStats.phase2_count}</div>
                              <small className="text-muted">Phase 2 Tweets</small>
                            </div>
                          </div>
                          <div className="col-md-3">
                            <div className="text-center">
                              <div className="fs-4 fw-bold text-warning">{miningStats.total_rewards_claimed}</div>
                              <small className="text-muted">Total Claimed</small>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              )}

              {/* Reward Sync Component */}
              <div className="row justify-content-center mb-4">
                <div className="col-lg-8">
                  <RewardSyncComponent 
                    onSyncSuccess={() => {
                      setSuccess('🔄 Rewards synced successfully! You can now claim your tokens.');
                      fetchMiningStats();
                      setTimeout(() => setSuccess(null), 5000);
                    }}
                    onSyncError={handleClaimError}
                  />
                </div>
              </div>

              {/* Main Batch Claim Component */}
              <div className="row justify-content-center">
                <div className="col-lg-8">
                  <BatchClaimComponent 
                    onSuccess={handleClaimSuccess}
                    onError={handleClaimError}
                  />
                </div>
              </div>

              {/* Information Section */}
              <div className="row mt-4">
                <div className="col-md-12">
                  <div className="card">
                    <div className="card-body">
                      <h5 className="card-title">ℹ️ How the New Claiming System Works</h5>
                      <div className="row">
                        <div className="col-md-4">
                          <h6>🔄 Before TCE (Token Claim Event)</h6>
                          <ul>
                            <li>Mining rewards are logged off-chain</li>
                            <li>No transaction fees for each tweet mined</li>
                            <li>Rewards accumulate in your account</li>
                            <li>Better user experience with instant feedback</li>
                          </ul>
                        </div>
                        <div className="col-md-4">
                          <h6>🔄 Reward Sync (Required Step)</h6>
                          <ul>
                            <li>Sync off-chain rewards to blockchain</li>
                            <li>You pay the transaction fee for syncing</li>
                            <li>Required before claiming tokens</li>
                            <li>One-time operation per sync batch</li>
                          </ul>
                        </div>
                        <div className="col-md-4">
                          <h6>🎯 After TCE</h6>
                          <ul>
                            <li>Claim all synced rewards at once</li>
                            <li>Pay transaction fee for claiming</li>
                            <li>Secure on-chain token distribution</li>
                            <li>Transparent and verifiable claims</li>
                          </ul>
                        </div>
                      </div>
                      <div className="mt-3">
                        <div className="alert alert-info">
                          <strong>💡 Benefits:</strong> This system reduces friction during the mining phase while maintaining 
                          security and transparency for the final token distribution. The reward sync step ensures your 
                          off-chain rewards are properly recorded on the blockchain before claiming, giving you full control 
                          over when to pay transaction fees.
                        </div>
                        <div className="alert alert-warning">
                          <strong>⚠️ Important:</strong> You must sync your rewards to the blockchain before you can claim them. 
                          This is a user-paid operation that ensures your rewards are properly recorded on-chain.
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              {/* Navigation */}
              <div className="row mt-4">
                <div className="col-md-12 text-center">
                  <button 
                    className="btn btn-outline-primary me-3" 
                    onClick={() => navigate('/tweet-mining')}
                  >
                    ← Back to Tweet Mining
                  </button>
                  <button 
                    className="btn btn-outline-secondary" 
                    onClick={() => navigate('/patron-framework')}
                  >
                    Go to Patron Framework →
                  </button>
                </div>
              </div>

            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ClaimPage;