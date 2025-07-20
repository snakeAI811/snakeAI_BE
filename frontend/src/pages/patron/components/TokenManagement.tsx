import React, { useState, useEffect } from 'react';
import { UserRole } from '../index';
import { tokenApi } from '../services/apiService';

interface TokenManagementProps {
  userRole: UserRole;
}

interface TokenInfo {
  balance: number;
  locked: number;
  staked: number;
  rewards: number;
  lockEndDate?: Date;
}

function TokenManagement({ userRole }: TokenManagementProps) {
  const [tokenInfo, setTokenInfo] = useState<TokenInfo>({
    balance: 0,
    locked: 0,
    staked: 0,
    rewards: 0
  });
  const [lockAmount, setLockAmount] = useState('');
  const [loading, setLoading] = useState(false);
  const [activeAction, setActiveAction] = useState<string | null>(null);

  useEffect(() => {
    fetchTokenInfo();
  }, []);

  const fetchTokenInfo = async () => {
    try {
      // TODO: Replace with actual API call
      // Mock data for now
      setTokenInfo({
        balance: 10000,
        locked: 5000,
        staked: 3000,
        rewards: 150,
        lockEndDate: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000) // 30 days from now
      });
    } catch (error) {
      console.error('Failed to fetch token info:', error);
    }
  };

  const handleLockTokens = async () => {
    if (!lockAmount || Number(lockAmount) <= 0) return;
    
    setLoading(true);
    setActiveAction('lock');
    
    try {
      const result = await tokenApi.lockTokens(
        Number(lockAmount) * 1000000000, // Convert to lamports
        userRole.role
      );

      if (result.success) {
        console.log('Lock tokens transaction:', result.data);
        setLockAmount('');
        fetchTokenInfo(); // Refresh token info
      } else {
        throw new Error(result.error || 'Failed to lock tokens');
      }
    } catch (error) {
      console.error('Error locking tokens:', error);
    } finally {
      setLoading(false);
      setActiveAction(null);
    }
  };

  const handleUnlockTokens = async () => {
    setLoading(true);
    setActiveAction('unlock');
    
    try {
      const result = await tokenApi.unlockTokens();

      if (result.success) {
        console.log('Unlock tokens transaction:', result.data);
        fetchTokenInfo(); // Refresh token info
      } else {
        throw new Error(result.error || 'Failed to unlock tokens');
      }
    } catch (error) {
      console.error('Error unlocking tokens:', error);
    } finally {
      setLoading(false);
      setActiveAction(null);
    }
  };

  const handleClaimRewards = async () => {
    setLoading(true);
    setActiveAction('claim');
    
    try {
      const result = await tokenApi.claimYield();

      if (result.success) {
        console.log('Claim yield transaction:', result.data);
        fetchTokenInfo(); // Refresh token info
      } else {
        throw new Error(result.error || 'Failed to claim rewards');
      }
    } catch (error) {
      console.error('Error claiming rewards:', error);
    } finally {
      setLoading(false);
      setActiveAction(null);
    }
  };

  const isLockPeriodActive = tokenInfo.lockEndDate && tokenInfo.lockEndDate > new Date();
  const canUnlock = !isLockPeriodActive && tokenInfo.locked > 0;

  return (
    <div className="w-100">
      <h3 className="mb-4">💰 Token Management</h3>

      {/* Token Overview */}
      <div className="row g-4 mb-4">
        <div className="col-md-3">
          <div className="card border-primary">
            <div className="card-body text-center">
              <h5 className="card-title text-primary">Available Balance</h5>
              <div className="fs-3">{tokenInfo.balance.toLocaleString()}</div>
              <small className="text-muted">SNAKE Tokens</small>
            </div>
          </div>
        </div>
        
        <div className="col-md-3">
          <div className="card border-warning">
            <div className="card-body text-center">
              <h5 className="card-title text-warning">Locked Tokens</h5>
              <div className="fs-3">{tokenInfo.locked.toLocaleString()}</div>
              <small className="text-muted">SNAKE Tokens</small>
            </div>
          </div>
        </div>
        
        <div className="col-md-3">
          <div className="card border-success">
            <div className="card-body text-center">
              <h5 className="card-title text-success">Staked Tokens</h5>
              <div className="fs-3">{tokenInfo.staked.toLocaleString()}</div>
              <small className="text-muted">SNAKE Tokens</small>
            </div>
          </div>
        </div>
        
        <div className="col-md-3">
          <div className="card border-info">
            <div className="card-body text-center">
              <h5 className="card-title text-info">Pending Rewards</h5>
              <div className="fs-3">{tokenInfo.rewards.toLocaleString()}</div>
              <small className="text-muted">SNAKE Tokens</small>
            </div>
          </div>
        </div>
      </div>

      {/* Lock Status */}
      {tokenInfo.lockEndDate && (
        <div className="alert alert-info mb-4">
          <strong>Lock Status:</strong> 
          {isLockPeriodActive ? (
            <>
              Tokens are locked until {tokenInfo.lockEndDate.toLocaleDateString()}
              <div className="mt-2">
                <small>
                  Time remaining: {Math.ceil((tokenInfo.lockEndDate.getTime() - Date.now()) / (1000 * 60 * 60 * 24))} days
                </small>
              </div>
            </>
          ) : (
            'Lock period has ended. You can now unlock your tokens.'
          )}
        </div>
      )}

      {/* Token Actions */}
      <div className="row g-4">
        {/* Lock Tokens */}
        <div className="col-lg-4">
          <div className="card h-100">
            <div className="card-body">
              <h5 className="card-title">🔒 Lock Tokens</h5>
              <p className="card-text">Lock tokens to earn staking rewards based on your role.</p>
              
              <div className="mb-3">
                <label className="form-label">Amount to Lock</label>
                <input
                  type="number"
                  className="form-control"
                  placeholder="Enter amount"
                  value={lockAmount}
                  onChange={(e) => setLockAmount(e.target.value)}
                  disabled={loading}
                  max={tokenInfo.balance}
                />
              </div>
              
              <button
                className="btn btn-primary w-100"
                onClick={handleLockTokens}
                disabled={loading || !lockAmount || Number(lockAmount) <= 0}
              >
                {loading && activeAction === 'lock' ? (
                  <>
                    <span className="spinner-border spinner-border-sm me-2" />
                    Locking...
                  </>
                ) : (
                  'Lock Tokens'
                )}
              </button>
              
              <small className="text-muted mt-2 d-block">
                {userRole.role === 'Staker' && 'Lock period: 3 months'}
                {userRole.role === 'Patron' && 'Lock period: 6 months'}
              </small>
            </div>
          </div>
        </div>

        {/* Unlock Tokens */}
        <div className="col-lg-4">
          <div className="card h-100">
            <div className="card-body">
              <h5 className="card-title">🔓 Unlock Tokens</h5>
              <p className="card-text">Unlock tokens after the lock period has ended.</p>
              
              <div className="mb-3">
                <div className="alert alert-secondary">
                  Locked Amount: <strong>{tokenInfo.locked.toLocaleString()} SNAKE</strong>
                </div>
              </div>
              
              <button
                className="btn btn-warning w-100"
                onClick={handleUnlockTokens}
                disabled={loading || !canUnlock}
              >
                {loading && activeAction === 'unlock' ? (
                  <>
                    <span className="spinner-border spinner-border-sm me-2" />
                    Unlocking...
                  </>
                ) : (
                  'Unlock Tokens'
                )}
              </button>
              
              <small className="text-muted mt-2 d-block">
                {!canUnlock && isLockPeriodActive && 'Tokens are still locked'}
                {!canUnlock && !isLockPeriodActive && 'No tokens to unlock'}
              </small>
            </div>
          </div>
        </div>

        {/* Claim Rewards */}
        <div className="col-lg-4">
          <div className="card h-100">
            <div className="card-body">
              <h5 className="card-title">🎁 Claim Rewards</h5>
              <p className="card-text">Claim your staking rewards and yield.</p>
              
              <div className="mb-3">
                <div className="alert alert-success">
                  Available Rewards: <strong>{tokenInfo.rewards.toLocaleString()} SNAKE</strong>
                </div>
              </div>
              
              <button
                className="btn btn-success w-100"
                onClick={handleClaimRewards}
                disabled={loading || tokenInfo.rewards <= 0}
              >
                {loading && activeAction === 'claim' ? (
                  <>
                    <span className="spinner-border spinner-border-sm me-2" />
                    Claiming...
                  </>
                ) : (
                  'Claim Rewards'
                )}
              </button>
              
              <small className="text-muted mt-2 d-block">
                APY: 5% for {userRole.role} role
              </small>
            </div>
          </div>
        </div>
      </div>

      {/* Information Panel */}
      <div className="mt-4">
        <div className="card border-info">
          <div className="card-body">
            <h6 className="card-title">📊 Staking Information</h6>
            <div className="row">
              <div className="col-md-6">
                <ul className="mb-0">
                  <li>Staking APY: 5% annually</li>
                  <li>Rewards calculated daily</li>
                  <li>Rewards can be claimed anytime</li>
                </ul>
              </div>
              <div className="col-md-6">
                <ul className="mb-0">
                  <li>Lock periods are enforced on-chain</li>
                  <li>Early unlock may incur penalties</li>
                  <li>Higher roles earn additional benefits</li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default TokenManagement;
