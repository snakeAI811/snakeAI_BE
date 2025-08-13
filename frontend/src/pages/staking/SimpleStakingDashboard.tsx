import React, { useState, useEffect, useCallback } from 'react';
import { useWalletContext } from '../../contexts/WalletContext';
import { tokenApi, userApi, roleApi } from '../patron/services/apiService';
import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import * as spl_associated_token_account from '@solana/spl-token';
import { useErrorHandler } from '../../hooks/useErrorHandler';
import { useToast } from '../../contexts/ToastContext';
import { useAppContext } from '../../contexts/AppContext';
import { PROGRAM_ID, TOKEN_MINT } from '../../config/program';

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
  const { handleError,handleWarning } = useErrorHandler();
  const { showSuccess, showError, showInfo } = useToast();
  const { userRole } = useAppContext();

  // Debug function to check on-chain balance directly
  const checkOnChainBalance = useCallback(async () => {
    if (!connected || !publicKey || !connection) return;
    
    try {
      const mint = new PublicKey(TOKEN_MINT); // Actual token mint from backend
      const userTokenAta = spl_associated_token_account.getAssociatedTokenAddressSync(
        mint,
        new PublicKey(publicKey)
      );
      
      const balance = await connection.getTokenAccountBalance(userTokenAta);
      console.log('🔍 Direct on-chain balance check:', balance.value.uiAmount, mint);
      return balance.value.uiAmount;
    } catch (error) {
      console.error('❌ Error checking on-chain balance:', error);
      return null;
    }
  }, [connected, publicKey, connection]);

  // Debug function to check user claim account state
  const checkUserClaimState = useCallback(async () => {
    if (!connected || !publicKey || !connection) return;
    
    try {
      const programId = new PublicKey(PROGRAM_ID); // From backend env
      const [userClaimPda] = PublicKey.findProgramAddressSync(
        [Buffer.from('user_claim'), new PublicKey(publicKey).toBuffer()],
        programId
      );
      
      console.log('🔍 User Claim PDA:', userClaimPda.toString());
      
      const accountInfo = await connection.getAccountInfo(userClaimPda);
      if (accountInfo) {
        console.log('✅ User Claim account exists');
        console.log('📊 Account data length:', accountInfo.data.length);
        console.log('💰 Account lamports:', accountInfo.lamports, programId);
      } else {
        console.log('❌ User Claim account does not exist');
      }
      
      return accountInfo;
    } catch (error) {
      console.error('❌ Error checking user claim state:', error);
      return null;
    }
  }, [connected, publicKey, connection]);

  // Make debug functions available globally for testing
  useEffect(() => {
    if (typeof window !== 'undefined') {
      (window as any).checkOnChainBalance = checkOnChainBalance;
      (window as any).checkUserClaimState = checkUserClaimState;
    }
  }, [checkOnChainBalance, checkUserClaimState]);

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

      // Debug logging for balance tracking
      console.log('🔍 Token Response:', tokenResponse);
      if (tokenResponse.success && tokenResponse.data) {
        console.log('💰 Current Balance:', tokenResponse.data.balance);
        console.log('🔒 Locked Amount:', tokenResponse.data.locked);
        console.log('📊 Staked Amount:', tokenResponse.data.staked);
      }

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
          balance: tokenResponse.data?.balance_ui || 0,
          locked: tokenResponse.data?.locked_ui || 0,
          staked: tokenResponse.data?.staked_ui || 0,
          rewards: tokenResponse.data?.yield_rewards_ui || 0,
          lockEndDate: tokenResponse.data?.lockEndDate !== undefined ? String(tokenResponse.data.lockEndDate) : undefined,
          miningCount: tokenResponse.data?.mining_count || 0,
          totalPhase1Mined: tweetMiningResponse.data?.phase1_count || 0,
          totalPhase2Mined: tweetMiningResponse.data?.phase2_count || 0,
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
      handleError(error, 'Failed to fetch staking data');
      console.error('Error fetching staking data:', error);
    } finally {
      setLoading(false);
    }
  }, [connected, publicKey]);

  useEffect(() => {
    fetchStakingData();
  }, []);

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

            showSuccess(`Yield claimed successfully! Transaction: ${signature}`)
          } else {
            showError('Phantom wallet not found');
            throw new Error('Phantom wallet not found');
          }
        } else {
          showSuccess('Yield claimed successfully!');
        }
        await fetchStakingData();
      } else {
        showError(`Failed to claim yield: ${response.error || 'Unknown error'}`);
      }
    } catch (error) {
      console.error('Error claiming yield:', error);
      handleError(error, 'Error claiming yield');
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

            showSuccess(`Tokens unlocked successfully! Transaction: ${signature}`);
          } else {
            throw new Error('Phantom wallet not found');
          }
        } else {
          showSuccess('Tokens unlocked successfully!');
        }
        await fetchStakingData();
      } else {
        showError(`Failed to unlock tokens: ${response.error}`);
      }
    } catch (error) {
      console.error('Error unlocking tokens:', error);
      handleError(error, 'Error unlocking tokens');

    } finally {
      setUnlocking(false);
    }
  };

  const handleStakeTokens = async () => {
    if (!connected || !publicKey || !stakeAmount) return;

    const amount = parseFloat(stakeAmount);
    if (isNaN(amount) || amount <= 0) {
      showSuccess('Please enter a valid amount');
      return;
    }

    if (amount > (stakingData?.balance || 0)) {
      showError('Insufficient balance');
      return;
    }

    if (amount < 5000) {
      showError('Minimum stake amount is 5000 SNAKE');
      return;
    }

    try {
      setStaking(true);
      console.log(`🔄 Starting stake process: ${amount} SNAKE for ${stakeDuration} months`);
      console.log('💰 Balance before staking:', stakingData?.balance);

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

            // Wait for transaction confirmation with more robust checking
            const latestBlockhash = await connection.getLatestBlockhash();
            await connection.confirmTransaction({
              signature,
              blockhash: transaction.recentBlockhash!,
              lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
            }, 'finalized'); // Use 'finalized' for stronger confirmation

            // Verify transaction was successful
            const txResult = await connection.getTransaction(signature, {
              commitment: 'finalized'
            });
            
            if (txResult?.meta?.err) {
              throw new Error(`Transaction failed: ${JSON.stringify(txResult.meta.err)}`);
            }
            
            console.log('✅ Transaction confirmed with finalized commitment');
            console.log('📋 Transaction result:', txResult?.meta);
            
            // Log detailed transaction information
            if (txResult?.meta) {
              console.log('💸 Pre-token balances:', txResult.meta.preTokenBalances);
              console.log('💰 Post-token balances:', txResult.meta.postTokenBalances);
              console.log('📝 Log messages:', txResult.meta.logMessages);
              
              // Check if balances actually changed
              const preBalance = txResult.meta.preTokenBalances?.find((b: any) => b.owner === publicKey);
              const postBalance = txResult.meta.postTokenBalances?.find((b: any) => b.owner === publicKey);
              
              if (preBalance && postBalance) {
                const balanceChange = (postBalance.uiTokenAmount.uiAmount || 0) - (preBalance.uiTokenAmount.uiAmount || 0);
                console.log(`💹 Balance change: ${balanceChange} tokens`);
                
                if (balanceChange === 0) {
                  console.warn('⚠️ WARNING: Balance did not change after transaction!');
                }
              }
            }
            
            showSuccess(`Tokens staked successfully! Transaction: ${signature}`);
          } else {
            throw new Error('Phantom wallet not found. Please install Phantom wallet.');
          }
        } else {
          showSuccess('Tokens staked successfully!');
        }

        setShowStakeModal(false);
        setStakeAmount('');
        
        // Wait longer for blockchain state to update, then refresh data multiple times
        setTimeout(async () => {
          // Check on-chain balance directly before fetching from backend
          const directBalance = await checkOnChainBalance();
          console.log('🔍 Direct on-chain balance after staking:', directBalance);
          
          await fetchStakingData();
          
          // Refresh again after another 3 seconds to ensure we get updated data
          setTimeout(async () => {
            const directBalance2 = await checkOnChainBalance();
            console.log('🔍 Direct on-chain balance (second check):', directBalance2);
            await fetchStakingData();
          }, 3000);
        }, 3000);
      } else {
        showError(`Failed to stake tokens: ${response.error}`);
      }
    } catch (error) {
      console.error('Error staking tokens:', error);
      handleError(error, 'Error staking tokens');

    } finally {
      setStaking(false);
    }
  };

  const formatDate = (dateInput?: string | number) => {
    if (!dateInput) return 'N/A';
    const date = typeof dateInput === 'number'
      ? new Date(dateInput)
      : new Date(dateInput);
    return date.toLocaleDateString();
  };

  const getTimeRemaining = (endTimestamp?: string | number) => {
    if (!endTimestamp) return 'N/A';

    const now = new Date().getTime();
    // Convert Unix timestamp (seconds) to milliseconds
    const endTime = typeof endTimestamp === 'string' 
      ? parseInt(endTimestamp) * 1000 
      : endTimestamp * 1000;
    const remaining = endTime - now;

    if (remaining <= 0) return 'Unlocked';

    const days = Math.floor(remaining / (1000 * 60 * 60 * 24));
    const hours = Math.floor((remaining % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    const minutes = Math.floor(remaining / (1000 * 60 )) ;

    return `${days}d ${hours}h ${minutes}m`;
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
    <div className="w-100 h-100">
      {/* Header */}
      <div className="d-flex justify-content-between align-items-center mb-4 custom-border-bottom pb-3">
        <div>
          <small className="text-muted">Wallet: {publicKey}</small>
        </div>
        <div className="d-flex gap-2 align-items-center">
          <button 
            className="btn btn-outline-primary btn-sm" 
            onClick={fetchStakingData}
            disabled={loading}
            title="Refresh balance"
          >
            <i className="bi bi-arrow-clockwise"></i> Refresh
          </button>
          <span className="badge bg-success">Connected</span>
          <span className="badge bg-primary">Phase {stakingData?.currentPhase || 1}</span>
          {userRole?.role && userRole.role !== 'none' && (
            <span className={`badge ${userRole.role === 'staker' ? 'bg-info' : 'bg-warning'}`}>
              {userRole.role === 'staker' ? '🏦 Staker' : '👑 Patron'}
              {userRole.role === 'patron' && ` (${userRole.status})`}
            </span>
          )}
          {userRole?.role === 'none' && (
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
              <h6 className="card-title text-muted mb-1">Claimable Rewards</h6>
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
        <div className="d-flex">
          <div className="col-12 col-md-4 mb-3  ">
            <div className='me-3 px-3 border border-3 border-dashed '>
              <div className="d-flex justify-content-between align-items-center py-2">
                <span className="text-muted">Total Tweets:</span>
                <strong>{stakingData?.tweetMiningStatus.totalTweets || 0}</strong>
              </div>
              <div className="d-flex justify-content-between align-items-center py-2">
                <span className="text-muted">Pending Rewards:</span>
                <strong>{stakingData?.tweetMiningStatus.pendingRewards.toLocaleString() || 0} </strong>
              </div>
            </div>
          </div>
          <div className="col-12 col-md-4 mb-3  ">
            <div className='me-3 px-3 border border-3 border-dashed '>
              <div className="d-flex justify-content-between align-items-center py-2">
                <span className="text-muted">Phase 1 Mined:</span>
                <strong>{stakingData?.totalPhase1Mined.toLocaleString() || 0} </strong>
              </div>
              <div className="d-flex justify-content-between align-items-center py-2">
                <span className="text-muted">Phase 2 Mined:</span>
                <strong>{stakingData?.totalPhase2Mined.toLocaleString() || 0} </strong>
              </div>
            </div>
          </div>
          <div className="col-12 col-md-4 mb-3  ">
            <div className='px-3 border border-3 border-dashed '>
              <div className="d-flex justify-content-between align-items-center py-2">
                <span className="text-muted">Total Claimed:</span>
                <strong>{stakingData?.tweetMiningStatus.totalRewardsClaimed.toLocaleString() || 0} </strong>
                </div>
              <div className="d-flex justify-content-between align-items-center py-2">
                <span className="text-muted">Lock End Date:</span>
                <strong>{stakingData?.lockEndDate ? formatDate(parseInt(stakingData.lockEndDate) * 1000) : 'N/A'}</strong>
              </div>
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
          {/* {(!stakingData?.locked || stakingData.locked === 0) && (stakingData?.balance || 0) > 0 && ( */}
            <button
              onClick={() => {
                // Set default duration based on user role
                // if (stakingData?.userRole === 'staker') {
                //   setStakeDuration(3);
                // } else if (stakingData?.userRole === 'patron') {
                //   setStakeDuration(6);
                // } else {
                //   setStakeDuration(0);
                // }
                setShowStakeModal(true);
              }}
              disabled={staking}
              className="btn btn-warning"
            >
              <i className="bi bi-lock-fill me-2"></i>
              Stake Tokens
            </button>
          {/* )} */}

          <button
            onClick={handleClaimYield}
            disabled={claiming || (stakingData?.rewards || 0) <= 0}
            className="primary-btn"
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

          {(stakingData?.lockEndDate && getTimeRemaining(stakingData?.lockEndDate) === 'Unlocked') ? (
            <button
              onClick={handleUnlockTokens}
              disabled={unlocking || (stakingData?.locked || 0) <= 0}
              className="second-btn"
            >
              {unlocking ? (
                <>
                  <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                  Unlocking...
                </>
              ) : (
                <>
                  <i className="bi bi-unlock-fill me-2"></i>
                  Unlock {stakingData?.locked?.toLocaleString() || 0} SNAKE
                </>
              )}
            </button>
           ) : null} 

          <button
            onClick={fetchStakingData}
            disabled={loading}
            className="second-btn"
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
                      min={5000}
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
                    <option value={3}>3 months (5% APY) - Staker</option>
                    {stakingData?.patronStatus === 'Approved' && (
                      <option value={6}>6 months (7% APY) - Patron</option>
                    )}
                    {stakingData?.patronStatus !== 'Approved' && (
                      <option value={6}>6 months (5% APY) - Staker, Patron</option>
                    )}
                  </select>
                  {stakingData?.userRole === 'none' && (
                    <div className="form-text text-warning">
                      You must select a role (Staker or Patron) before staking. Visit the Profile page to choose your role.
                    </div>
                  )}
                </div>

                <div className="alert alert-info">
                  <small>
                    <strong>Important:</strong> Staked tokens will be locked for the selected duration.
                    You'll earn rewards based on the APY and can claim them periodically.
                  </small>
                  {stakingData?.userRole === 'patron' && (
                    <div className="mt-2">
                      <small>
                        <strong>Patron Requirements:</strong> This role requires ≥250k tokens, 30+ day wallet age,
                        Phase 1 mining history, and 6+ months of staking experience.
                      </small>
                    </div>
                  )}
                  {stakingData?.userRole === 'staker' && (
                    <div className="mt-2">
                      <small>
                        <strong>Staker Requirements:</strong> This role requires 3+ months of previous staking history.
                      </small>
                    </div>
                  )}
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
