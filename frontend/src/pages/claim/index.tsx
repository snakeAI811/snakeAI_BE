import React, { useState, useEffect, useCallback } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import ResponsiveMenu from "../../components/ResponsiveMenu";
import { useWalletContext } from '../../contexts/WalletContext';
import { tokenApi, userApi } from '../patron/services/apiService';

interface ClaimPageProps {
  page_number?: number;
}

interface ClaimData {
  id: string;
  amount: number;
  phase: 'Phase1' | 'Phase2';
  timestamp: string;
  claimed: boolean;
  userId: string;
  tweetId?: string;
  twitterUsername?: string;
}

function ClaimPage({ page_number = 1 }: ClaimPageProps) {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { connected, publicKey, connect } = useWalletContext();

  const [claimData, setClaimData] = useState<ClaimData | null>(null);
  const [walletBalance, setWalletBalance] = useState<number>(0);
  const [loading, setLoading] = useState(false);
  const [claiming, setClaiming] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);

  const fetchClaimData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      if (!id) {
        setError('No reward ID provided');
        return;
      }

      const response = await userApi.getRewardById(id);

      if (response.success && response.data) {
        const rewardData = response.data;
        const claimData: ClaimData = {
          id: rewardData.id,
          amount: rewardData.reward_amount,
          phase: (rewardData.phase === 'Phase2' ? 'Phase2' : 'Phase1') as 'Phase1' | 'Phase2',
          timestamp: rewardData.created_at,
          claimed: !!rewardData.transaction_signature, // If transaction exists, it's been claimed
          userId: rewardData.user_id,
          tweetId: rewardData.tweet_id
        };
        setClaimData(claimData);
      } else {
        setError(response.error || 'Failed to fetch reward data');
      }
    } catch (err) {
      setError('Failed to fetch claim data');
      console.error('Error fetching claim data:', err);
    } finally {
      setLoading(false);
    }
  }, [id]);

  const fetchWalletBalance = useCallback(async () => {
    if (!connected) return;

    try {
      const response = await tokenApi.getTokenInfo();
      if (response.success && response.data) {
        setWalletBalance(response.data.balance);
      }
    } catch (err) {
      console.error('Error fetching wallet balance:', err);
    }
  }, [connected]);

  useEffect(() => {
    if (id) {
      fetchClaimData();
    }
  }, [id, fetchClaimData]);

  useEffect(() => {
    if (connected) {
      fetchWalletBalance();
    }
  }, [connected, fetchWalletBalance]);

  const validateClaim = () => {
    if (!claimData) return { valid: false, message: "No claim data available" };
    if (!connected) return { valid: false, message: "Please connect your wallet first" };
    if (!publicKey) return { valid: false, message: "Wallet not properly connected" };
    if (!claimData.tweetId) return { valid: false, message: "No tweet ID found for this reward" };
    if (claimData.claimed) return { valid: false, message: "This reward has already been claimed" };
    if (!claimData.amount || claimData.amount <= 0) return { valid: false, message: "Invalid reward amount" };
    
    return { valid: true, message: "" };
  };

  const handleClaim = async () => {
    const validation = validateClaim();
    if (!validation.valid) {
      setError(validation.message);
      return;
    }

    setClaiming(true);
    setError(null);

    try {
      // Check if reward is still available before claiming
      const rewardCheck = await userApi.getRewardById(claimData!.id);
      if (!rewardCheck.success || !rewardCheck.data) {
        throw new Error('Reward not found or no longer available');
      }

      if (rewardCheck.data.transaction_signature) {
        throw new Error('This reward has already been claimed');
      }

      if (!rewardCheck.data.available) {
        throw new Error('This reward is no longer available for claiming');
      }

      // Attempt to claim the reward
      const response = await userApi.claimTweetReward(claimData!.tweetId!);

      if (response.success) {
        setSuccess(true);
        setClaimData(prev => prev ? { ...prev, claimed: true } : null);
        await fetchWalletBalance(); // Refresh wallet balance after claim
        console.log('Claim successful:', response.data);
      } else {
        throw new Error(response.error || 'Claim failed');
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to claim tokens';
      setError(errorMessage);
      console.error('Error claiming tokens:', err);
      
      // If the error is about authentication, provide specific guidance
      if (errorMessage.includes('401') || errorMessage.includes('unauthorized') || errorMessage.includes('session')) {
        setError('You need to log in to your account to claim this reward. Please visit the main site and log in first.');
      }
    } finally {
      setClaiming(false);
    }
  };



  if (loading) {
    return (
      <div className="w-100 p-3" style={{ height: "100vh" }}>
        <div className="d-flex justify-content-center align-items-center" style={{ height: "100%" }}>
          <div className="text-center">
            <div className="spinner-border" role="status">
              <span className="visually-hidden">Loading...</span>
            </div>
            <p className="mt-2">Loading claim data...</p>
          </div>
        </div>
      </div>
    );
  }

  // if (error && !claimData) {
  //   return (
  //     <div className="w-100 p-3" style={{ height: "100vh" }}>
  //       <div className="d-flex justify-content-center align-items-center" style={{ height: "100%" }}>
  //         <div className="text-center">
  //           <div className="alert alert-danger">
  //             <h4>Error</h4>
  //             <p>{error}</p>
  //             <button className="primary-btn" onClick={fetchClaimData}>
  //               Try Again
  //             </button>
  //           </div>
  //         </div>
  //       </div>
  //     </div>
  //   );
  // }

  return (
    <div className="w-100 p-3" style={{ height: "100vh" }}>
      <div className="d-flex gap-4 justify-content-center" style={{ height: "calc(100vh-60px)", paddingTop: '55px' }}>
        
        <div className="custom-content" >
          <div className="w-100">
            <div className="d-flex justify-content-between align-items-center">
              <div className="fs-1" style={{ lineHeight: 'normal' }}>🎁 Claim Your Twitter Mining Reward</div>
            </div>
          </div>
          <div className="custom-border-y custom-content-height d-flex flex-column px-3">
            <div className="w-100" style={{ minHeight: '86vh' }}>

              {/* Header */}
              <div className="w-100">
                {claimData?.tweetId && (
                  <div className="alert alert-info mb-3">
                    <div className="d-flex align-items-center">
                      <span className="me-2">🐦</span>
                      <div>
                        <strong>Qualifying Tweet:</strong>{' '}
                        <a
                          href={`https://twitter.com/i/status/${claimData.tweetId}`}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-decoration-none"
                        >
                          View on Twitter →
                        </a>
                      </div>
                    </div>
                  </div>
                )}
              </div>

              {/* Wallet Connection & Validation */}
              {!connected ? (
                <div className="w-100 mb-4">
                  <div className="alert alert-warning text-center">
                    <h4>🔗 Connect Your Phantom Wallet</h4>
                    <p>Please connect your Phantom wallet to get reward</p>
                    <button className="primary-btn btn-lg" onClick={connect}>
                      Connect Phantom Wallet
                    </button>
                  </div>
                </div>
              ) : (
                <div className="w-100 mb-4">
                  <div className="alert alert-success">
                    <div className="row">
                      <div className="col-md-6">
                        <strong>✅ Wallet Connected:</strong> {publicKey?.slice(0, 8)}...{publicKey?.slice(-8)}
                      </div>
                      <div className="col-md-6">
                        <strong>💰 Current Balance:</strong> {walletBalance.toLocaleString()} SNAKE tokens
                      </div>
                    </div>
                  </div>
                  
                  {/* Claim Readiness Check */}
                  {claimData && (
                    <div className="alert alert-info">
                      <h6>📋 Claim Status Check:</h6>
                      <div className="row">
                        <div className="col-md-4">
                          <small>
                            {claimData.tweetId ? '✅' : '❌'} Tweet ID: {claimData.tweetId ? 'Found' : 'Missing'}
                          </small>
                        </div>
                        <div className="col-md-4">
                          <small>
                            {claimData.amount > 0 ? '✅' : '❌'} Reward Amount: {claimData.amount > 0 ? `${claimData.amount.toLocaleString()} SNAKE` : 'Invalid'}
                          </small>
                        </div>
                        <div className="col-md-4">
                          <small>
                            {!claimData.claimed ? '✅' : '❌'} Status: {!claimData.claimed ? 'Ready to claim' : 'Already claimed'}
                          </small>
                        </div>
                      </div>
                    </div>
                  )}
                </div>
              )}

                {/* Success Message */}
                {success && (
                  <div className="w-100 mb-4">
                    <div className="alert alert-success">
                      <h4>🎉 Claim Successful!</h4>
                      <p>Your tokens have been claimed successfully and your role has been updated.</p>
                      <button className="btn btn-outline-success" onClick={() => navigate('/patron-framework')}>
                        Go to Patron Framework
                      </button>
                    </div>
                  </div>
                )}

              {/* Main Content */}
              {claimData && connected && !success && (
                <div className="row justify-content-center">
                  {/* Claim Summary */}
                  <div className="col-lg-8">
                    <div className="card border-3 border-primary">
                      <div className="card-body">
                        <h3 className="card-title mb-4 text-center">📊 Claim Your Reward</h3>

                        <div className="mb-4">
                          <div className="row g-3">
                            <div className="col-md-6">
                              <div className="card bg-primary text-white">
                                <div className="card-body text-center">
                                  <div className="fs-2 fw-bold">{claimData.amount.toLocaleString()}</div>
                                  <small>SNAKE Tokens to Claim</small>
                                </div>
                              </div>
                            </div>
                            <div className="col-md-6">
                              <div className="card">
                                <div className="card-body text-center">
                                  <div className={`badge bg-${claimData.phase === 'Phase1' ? 'success' : 'info'} fs-6 mb-2`}>
                                    {claimData.phase}
                                  </div>
                                  <div><small className="text-muted">Mining Phase</small></div>
                                </div>
                              </div>
                            </div>
                          </div>
                        </div>

                        {/* Current Balance Display */}
                        <div className="mb-4">
                          <div className="card">
                            <div className="card-body">
                              <div className="row">
                                <div className="col-md-6">
                                  <h6>💰 Current Balance</h6>
                                  <div className="fs-5 fw-bold text-success">{walletBalance.toLocaleString()} SNAKE</div>
                                </div>
                                <div className="col-md-6">
                                  <h6>🎁 After Claim</h6>
                                  <div className="fs-5 fw-bold text-primary">{(walletBalance + claimData.amount).toLocaleString()} SNAKE</div>
                                </div>
                              </div>
                            </div>
                          </div>
                        </div>

                        {/* Error Display */}
                        {error && (
                          <div className="alert alert-danger">
                            <h6>❌ Claim Failed</h6>
                            <p><strong>Error:</strong> {error}</p>
                            {error.includes('log in') || error.includes('session') || error.includes('unauthorized') ? (
                              <div className="mt-2">
                                <small><strong>Solution:</strong> Please visit <a href="/" className="text-decoration-none">the main site</a> and log in to your account first, then return to this claim page.</small>
                              </div>
                            ) : error.includes('already been claimed') ? (
                              <div className="mt-2">
                                <small><strong>Info:</strong> This reward has already been claimed. Each reward can only be claimed once.</small>
                              </div>
                            ) : error.includes('not found') || error.includes('no longer available') ? (
                              <div className="mt-2">
                                <small><strong>Info:</strong> This reward is no longer available. It may have expired or been claimed already.</small>
                              </div>
                            ) : (
                              <div className="mt-2">
                                <small><strong>Suggestion:</strong> Please check your wallet connection and try again. If the problem persists, contact support.</small>
                              </div>
                            )}
                          </div>
                        )}

                        {/* Claim Button */}
                        <div className="d-grid">
                          <button
                            className="primary-btn btn-lg"
                            onClick={handleClaim}
                            disabled={claiming || claimData.claimed || !claimData.tweetId}
                          >
                            {claiming ? (
                              <>
                                <span className="spinner-border spinner-border-sm me-2" role="status" />
                                Claiming...
                              </>
                            ) : claimData.claimed ? (
                              'Already Claimed'
                            ) : (
                              `🎁 Claim ${claimData.amount.toLocaleString()} SNAKE Tokens`
                            )}
                          </button>
                        </div>

                        {/* Twitter Mining Info */}
                        <div className="mt-4">
                          <div className="card border-info">
                            <div className="card-body">
                              <h6 className="card-title">🐦 How Twitter Mining Works</h6>
                              <ul className="mb-0">
                                <li><small>1. Follow @playSnakeAI on Twitter</small></li>
                                <li><small>2. Tweet mentioning @playSnakeAI with #MineTheSnake</small></li>
                                <li><small>3. Our system automatically detects qualifying tweets</small></li>
                                <li><small>4. You get a claim link sent to you on Twitter</small></li>
                                <li><small>5. One reward per 24 hours per user</small></li>
                              </ul>
                            </div>
                          </div>
                        </div>

                        {/* Important Notes */}
                        <div className="mt-3">
                          <div className="card border-warning">
                            <div className="card-body">
                              <h6 className="card-title">⚠️ Important Notes</h6>
                              <ul className="mb-0">
                                <li><small><strong>Authentication Required:</strong> You must be logged in to your Snake AI account to claim rewards</small></li>
                                <li><small><strong>Wallet Connection:</strong> Your wallet must be connected and the same one linked to your account</small></li>
                                <li><small><strong>Transaction Fees:</strong> Make sure you have enough SOL for transaction fees</small></li>
                                <li><small><strong>One-Time Claim:</strong> Each tweet can only be claimed once</small></li>
                                <li><small><strong>Blockchain Confirmation:</strong> This action requires blockchain confirmation</small></li>
                              </ul>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ClaimPage;
