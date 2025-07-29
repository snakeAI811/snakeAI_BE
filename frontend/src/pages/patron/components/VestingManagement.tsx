import React, { useState, useEffect } from 'react';
import { UserRole } from '../index';
import { tokenApi } from '../services/apiService';
import { useToast } from '../../../contexts/ToastContext';
import { useWalletContext } from '../../../contexts/WalletContext';
import { Connection, Transaction, PublicKey } from '@solana/web3.js';
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { SOLANA_RPC_URL, TOKEN_MINT } from '../../../config/program';

interface VestingManagementProps {
    userRole: UserRole;
}

interface VestingInfo {
    totalAmount: number;
    vestedAmount: number;
    startTime: Date;
    endTime: Date;
    vestingType: 'Staker' | 'Patron';
    isActive: boolean;
    lastClaimTime: Date;
    yieldRate: number;
    claimableAmount: number;
    yieldAccrued: number;
}

function VestingManagement({ userRole }: VestingManagementProps) {
    const { showSuccess, showError, showInfo } = useToast();
    const { publicKey, connected } = useWalletContext();
    
    const connection = new Connection(SOLANA_RPC_URL, {
        commitment: 'confirmed',
        confirmTransactionInitialTimeout: 120000,
        disableRetryOnRateLimit: false,
    });

    const [vestingInfo, setVestingInfo] = useState<VestingInfo | null>(null);
    const [vestingAmount, setVestingAmount] = useState('');
    const [loading, setLoading] = useState(false);
    const [activeAction, setActiveAction] = useState<string | null>(null);
    const [showBalanceWarning, setShowBalanceWarning] = useState(false);
    const [tokenBalance, setTokenBalance] = useState(0);

    useEffect(() => {
        if (connected) {
            fetchVestingInfo();
            fetchTokenBalance();
        }
    }, [connected]);

    const fetchTokenBalance = async () => {
        if (!connected || !publicKey) return;
        
        try {
            const userTokenAccount = await getAssociatedTokenAddress(TOKEN_MINT, new PublicKey(publicKey));
            const tokenBalance = await connection.getTokenAccountBalance(userTokenAccount);
            setTokenBalance(tokenBalance.value.uiAmount || 0);
        } catch (error) {
            console.error('Failed to fetch token balance:', error);
        }
    };

    const fetchVestingInfo = async () => {
        setLoading(true);
        try {
            const response = await tokenApi.getVestingInfo();
            if (response.success && response.data) {
                setVestingInfo({
                    totalAmount: response.data.totalAmount,
                    vestedAmount: response.data.vestedAmount,
                    startTime: new Date(response.data.startTime * 1000),
                    endTime: new Date(response.data.endTime * 1000),
                    vestingType: response.data.vestingType,
                    isActive: response.data.isActive,
                    lastClaimTime: new Date(response.data.lastClaimTime * 1000),
                    yieldRate: response.data.yieldRate,
                    claimableAmount: response.data.claimableAmount || 0,
                    yieldAccrued: response.data.yieldAccrued || 0
                });
            } else {
                setVestingInfo(null);
            }
        } catch (error) {
            console.error('Failed to fetch vesting info:', error);
            setVestingInfo(null);
        } finally {
            setLoading(false);
        }
    };

    const validateVestingAmount = (amount: string) => {
        const numAmount = Number(amount);
        if (numAmount > tokenBalance) {
            setShowBalanceWarning(true);
            return false;
        } else {
            setShowBalanceWarning(false);
            return true;
        }
    };

    const handleAmountChange = (value: string) => {
        setVestingAmount(value);
        if (value) {
            validateVestingAmount(value);
        } else {
            setShowBalanceWarning(false);
        }
    };

    const handleCreateVesting = async () => {
        const numAmount = Number(vestingAmount);

        if (!connected || !publicKey) {
            showError('Please connect your wallet first');
            return;
        }

        if (!vestingAmount || numAmount <= 0) {
            showError('Please enter a valid amount to vest');
            return;
        }

        if (numAmount > tokenBalance) {
            showError(`Insufficient balance. You have ${tokenBalance.toLocaleString()} tokens available.`);
            return;
        }

        if (numAmount < 100) {
            showError('Minimum vesting amount is 100 tokens');
            return;
        }

        if (userRole.role === 'none') {
            showError('Please select a role before creating a vesting schedule');
            return;
        }

        setLoading(true);
        setActiveAction('create');
        showInfo(`Creating vesting schedule for ${numAmount.toLocaleString()} tokens...`);

        try {
            const result = await tokenApi.createVestingSchedule(
                numAmount * 1000000000 // Convert to lamports
            );

            if (result.success && result.data) {
                if ('requiresWalletSignature' in result && result.requiresWalletSignature) {
                    showInfo('Please approve the transaction in your wallet...');
                    
                    const transactionBytes = Uint8Array.from(atob(result.data), c => c.charCodeAt(0));
                    const transaction = Transaction.from(transactionBytes);
                    
                    const { blockhash } = await connection.getLatestBlockhash('confirmed');
                    transaction.recentBlockhash = blockhash;
                    transaction.feePayer = new PublicKey(publicKey);
                    
                    if (typeof window !== 'undefined' && (window as any).solana) {
                        const signedTransaction = await (window as any).solana.signTransaction(transaction);
                        
                        showInfo('Submitting transaction to blockchain...');
                        const signature = await connection.sendRawTransaction(signedTransaction.serialize(), {
                            skipPreflight: false,
                            preflightCommitment: 'confirmed'
                        });
                        
                        showInfo('Waiting for blockchain confirmation...');
                        await connection.confirmTransaction({
                            signature,
                            blockhash: transaction.recentBlockhash!,
                            lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight
                        }, 'confirmed');
                        
                        showSuccess(
                            `Successfully created vesting schedule for ${numAmount.toLocaleString()} tokens!`
                        );
                        setVestingAmount('');
                        setShowBalanceWarning(false);
                        fetchVestingInfo();
                        fetchTokenBalance();
                    } else {
                        throw new Error('Phantom wallet not found');
                    }
                } else {
                    showSuccess(
                        `Successfully created vesting schedule for ${numAmount.toLocaleString()} tokens!`
                    );
                    setVestingAmount('');
                    setShowBalanceWarning(false);
                    fetchVestingInfo();
                    fetchTokenBalance();
                }
            } else {
                throw new Error(result.error || 'Failed to create vesting schedule');
            }
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'Failed to create vesting schedule';
            showError(errorMessage);
            console.error('Error creating vesting:', error);
        } finally {
            setLoading(false);
            setActiveAction(null);
        }
    };

    const handleClaimVested = async () => {
        if (!vestingInfo) return;

        setLoading(true);
        setActiveAction('claim');
        showInfo('Processing claim vested tokens transaction...');

        try {
            const result = await tokenApi.claimVestedTokens();

            if (result.success && result.data) {
                if ('requiresWalletSignature' in result && result.requiresWalletSignature) {
                    showInfo('Please approve the transaction in your wallet...');
                    
                    const transactionBytes = Uint8Array.from(atob(result.data), c => c.charCodeAt(0));
                    const transaction = Transaction.from(transactionBytes);
                    
                    const { blockhash } = await connection.getLatestBlockhash('confirmed');
                    transaction.recentBlockhash = blockhash;
                    transaction.feePayer = new PublicKey(publicKey!);
                    
                    if (typeof window !== 'undefined' && (window as any).solana) {
                        const signedTransaction = await (window as any).solana.signTransaction(transaction);
                        
                        showInfo('Submitting transaction to blockchain...');
                        const signature = await connection.sendRawTransaction(signedTransaction.serialize(), {
                            skipPreflight: false,
                            preflightCommitment: 'confirmed'
                        });
                        
                        showInfo('Waiting for blockchain confirmation...');
                        await connection.confirmTransaction({
                            signature,
                            blockhash: transaction.recentBlockhash!,
                            lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight
                        }, 'confirmed');
                        
                        showSuccess(`Successfully claimed vested tokens!`);
                        fetchVestingInfo();
                        fetchTokenBalance();
                    } else {
                        throw new Error('Phantom wallet not found');
                    }
                } else {
                    showSuccess('Successfully claimed vested tokens!');
                    fetchVestingInfo();
                    fetchTokenBalance();
                }
            } else {
                throw new Error(result.error || 'Failed to claim vested tokens');
            }
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : 'Failed to claim vested tokens';
            showError(errorMessage);
            console.error('Error claiming vested tokens:', error);
        } finally {
            setLoading(false);
            setActiveAction(null);
        }
    };

    const getVestingPeriod = () => {
        if (userRole.role === 'staker') return 3;
        if (userRole.role === 'patron') return 6;
        return 3; // default
    };

    const getEstimatedAPY = () => {
        if (userRole.role === 'staker') return '5%';
        if (userRole.role === 'patron') return '0% (6-month commitment)';
        return '0%';
    };

    const calculateVestingProgress = () => {
        if (!vestingInfo) return 0;
        const now = Date.now();
        const start = vestingInfo.startTime.getTime();
        const end = vestingInfo.endTime.getTime();
        
        if (now <= start) return 0;
        if (now >= end) return 100;
        
        return ((now - start) / (end - start)) * 100;
    };

    const isVestingComplete = vestingInfo && Date.now() >= vestingInfo.endTime.getTime();
    const vestingProgress = calculateVestingProgress();

    return (
        <div className="w-100">
            <div className="mb-4">
                <h4>🏦 Enhanced Vesting System</h4>
                <p className="text-muted">
                    Create time-locked vesting schedules with yield rewards for Stakers
                </p>
            </div>

            {/* Vesting Overview */}
            {vestingInfo ? (
                <div className="row g-4 mb-4">
                    <div className="col-md-3">
                        <div className="card border-primary">
                            <div className="card-body text-center">
                                <h6 className="card-title text-primary">Total Vested</h6>
                                <div className="fs-4">{vestingInfo.totalAmount.toLocaleString()}</div>
                                <small className="text-muted">SNAKE Tokens</small>
                            </div>
                        </div>
                    </div>

                    <div className="col-md-3">
                        <div className="card border-success">
                            <div className="card-body text-center">
                                <h6 className="card-title text-success">Claimed</h6>
                                <div className="fs-4">{vestingInfo.vestedAmount.toLocaleString()}</div>
                                <small className="text-muted">SNAKE Tokens</small>
                            </div>
                        </div>
                    </div>

                    <div className="col-md-3">
                        <div className="card border-warning">
                            <div className="card-body text-center">
                                <h6 className="card-title text-warning">Claimable</h6>
                                <div className="fs-4">{vestingInfo.claimableAmount.toLocaleString()}</div>
                                <small className="text-muted">SNAKE Tokens</small>
                            </div>
                        </div>
                    </div>

                    <div className="col-md-3">
                        <div className="card border-info">
                            <div className="card-body text-center">
                                <h6 className="card-title text-info">Yield Earned</h6>
                                <div className="fs-4">{vestingInfo.yieldAccrued.toLocaleString()}</div>
                                <small className="text-muted">SNAKE Tokens</small>
                            </div>
                        </div>
                    </div>
                </div>
            ) : (
                <div className="alert alert-info mb-4">
                    <strong>No active vesting schedule</strong><br />
                    Create a vesting schedule to start earning yield rewards.
                </div>
            )}

            {/* Vesting Progress */}
            {vestingInfo && (
                <div className="card mb-4">
                    <div className="card-body">
                        <h6 className="card-title">📊 Vesting Progress</h6>
                        <div className="progress mb-3" style={{ height: '25px' }}>
                            <div
                                className={`progress-bar ${isVestingComplete ? 'bg-success' : 'bg-primary'}`}
                                role="progressbar"
                                style={{ width: `${vestingProgress}%` }}
                            >
                                {vestingProgress.toFixed(1)}%
                            </div>
                        </div>
                        <div className="row">
                            <div className="col-md-3">
                                <strong>Start Date:</strong><br />
                                {vestingInfo.startTime.toLocaleDateString()}
                            </div>
                            <div className="col-md-3">
                                <strong>End Date:</strong><br />
                                {vestingInfo.endTime.toLocaleDateString()}
                            </div>
                            <div className="col-md-3">
                                <strong>Type:</strong><br />
                                {vestingInfo.vestingType}
                            </div>
                            <div className="col-md-3">
                                <strong>Status:</strong><br />
                                <span className={`badge ${isVestingComplete ? 'bg-success' : 'bg-warning'}`}>
                                    {isVestingComplete ? 'Complete' : 'Active'}
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            )}

            <div className="row g-4">
                {/* Create Vesting */}
                {!vestingInfo && (
                    <div className="col-lg-6">
                        <div className="card h-100">
                            <div className="card-body">
                                <h5 className="card-title">🔒 Create Vesting Schedule</h5>
                                <p className="card-text">Lock tokens in a vesting schedule based on your role.</p>

                                <div className="mb-3">
                                    <label className="form-label">Amount to Vest</label>
                                    <input
                                        type="number"
                                        className={`form-control ${showBalanceWarning ? 'is-invalid' : ''}`}
                                        placeholder="Enter amount (min. 100 tokens)"
                                        value={vestingAmount}
                                        onChange={(e) => handleAmountChange(e.target.value)}
                                        disabled={loading}
                                        max={tokenBalance}
                                        min="100"
                                    />
                                    {showBalanceWarning && (
                                        <div className="invalid-feedback">
                                            ⚠️ Insufficient balance! You have {tokenBalance.toLocaleString()} tokens available.
                                        </div>
                                    )}
                                    <div className="text-muted small mt-1">
                                        Available: {tokenBalance.toLocaleString()} SNAKE tokens
                                    </div>
                                </div>

                                <div className="mb-3">
                                    <div className="alert alert-info">
                                        <strong>Role: {userRole.role}</strong><br />
                                        <div className="row mt-2">
                                            <div className="col-6">
                                                <strong>Period:</strong><br />
                                                {getVestingPeriod()} months
                                            </div>
                                            <div className="col-6">
                                                <strong>APY:</strong><br />
                                                {getEstimatedAPY()}
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <button
                                    className="btn btn-primary w-100"
                                    onClick={handleCreateVesting}
                                    disabled={loading || !vestingAmount || Number(vestingAmount) <= 0 || showBalanceWarning || Number(vestingAmount) < 100}
                                >
                                    {loading && activeAction === 'create' ? (
                                        <>
                                            <span className="spinner-border spinner-border-sm me-2" />
                                            Creating...
                                        </>
                                    ) : (
                                        'Create Vesting Schedule'
                                    )}
                                </button>

                                <div className="mt-2">
                                    <small className="text-muted d-block">
                                        🔒 Tokens will be vested over {getVestingPeriod()} months
                                    </small>
                                    {userRole.role === 'staker' && (
                                        <small className="text-success d-block">
                                            💰 Earn 5% APY yield during vesting period
                                        </small>
                                    )}
                                </div>
                            </div>
                        </div>
                    </div>
                )}

                {/* Claim Vested Tokens */}
                {vestingInfo && (
                    <div className="col-lg-6">
                        <div className="card h-100">
                            <div className="card-body">
                                <h5 className="card-title">💎 Claim Vested Tokens</h5>
                                <p className="card-text">Claim your vested tokens and earned yield.</p>

                                <div className="mb-3">
                                    <div className="alert alert-success">
                                        <div className="row">
                                            <div className="col-6">
                                                <strong>Claimable:</strong><br />
                                                {vestingInfo.claimableAmount.toLocaleString()} SNAKE
                                            </div>
                                            <div className="col-6">
                                                <strong>Yield:</strong><br />
                                                {vestingInfo.yieldAccrued.toLocaleString()} SNAKE
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <button
                                    className="btn btn-success w-100"
                                    onClick={handleClaimVested}
                                    disabled={loading || vestingInfo.claimableAmount <= 0}
                                >
                                    {loading && activeAction === 'claim' ? (
                                        <>
                                            <span className="spinner-border spinner-border-sm me-2" />
                                            Claiming...
                                        </>
                                    ) : (
                                        'Claim Vested Tokens'
                                    )}
                                </button>

                                <small className="text-muted mt-2 d-block">
                                    {vestingInfo.claimableAmount <= 0 && 'No tokens available to claim yet'}
                                </small>
                            </div>
                        </div>
                    </div>
                )}
            </div>

            {/* Information Panel */}
            <div className="mt-4">
                <div className="card border-info">
                    <div className="card-body">
                        <h6 className="card-title">📚 Vesting Information</h6>
                        <div className="row">
                            <div className="col-md-6">
                                <ul className="mb-0">
                                    <li><strong>Staker Vesting:</strong> 3 months with 5% APY</li>
                                    <li><strong>Patron Vesting:</strong> 6 months commitment</li>
                                    <li>Linear vesting after cliff period</li>
                                </ul>
                            </div>
                            <div className="col-md-6">
                                <ul className="mb-0">
                                    <li>Yield rewards calculated continuously</li>
                                    <li>Claim vested tokens anytime</li>
                                    <li>Early exit may incur penalties</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default VestingManagement;
