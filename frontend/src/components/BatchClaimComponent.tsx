import React, { useState, useEffect } from 'react';
import { Connection, Transaction } from '@solana/web3.js';
import { userApi } from '../pages/patron/services/apiService';
import { useWalletContext } from '../contexts/WalletContext';

const SOLANA_RPC_URL = process.env.REACT_APP_SOLANA_RPC_URL || 'https://api.devnet.solana.com';

interface BatchClaimComponentProps {
    onSuccess?: () => void;
    onError?: (error: string) => void;
}

interface AccumulatedRewards {
    accumulated_rewards: number;
    last_claim_timestamp: number;
    can_claim: boolean;
}

const BatchClaimComponent: React.FC<BatchClaimComponentProps> = ({ onSuccess, onError }) => {
    const { connected, publicKey, connect } = useWalletContext();
    const [accumulatedRewards, setAccumulatedRewards] = useState<AccumulatedRewards | null>(null);
    const [loading, setLoading] = useState(false);
    const [claiming, setClaiming] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const fetchAccumulatedRewards = async () => {
        if (!connected) return;
        
        try {
            setLoading(true);
            const response = await userApi.getAccumulatedRewards();
            if (response.success && response.data) {
                setAccumulatedRewards(response.data);
            } else {
                setError(response.error || 'Failed to fetch accumulated rewards');
            }
        } catch (err) {
            setError('Failed to fetch accumulated rewards');
            console.error('Error fetching accumulated rewards:', err);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        if (connected) {
            fetchAccumulatedRewards();
        }
    }, [connected]);

    const handleBatchClaim = async () => {
        if (!connected || !accumulatedRewards || accumulatedRewards.accumulated_rewards <= 0) {
            return;
        }

        setClaiming(true);
        setError(null);

        try {
            const response = await userApi.batchClaim();
            if (response.success && response.data) {
                const transactionBase64 = response.data;
                const solana = (window as any).solana;
                
                if (!solana || !solana.isPhantom) {
                    throw new Error('Phantom wallet not found. Please install Phantom.');
                }
                
                if (!solana.isConnected) {
                    await solana.connect();
                }

                const connection = new Connection(SOLANA_RPC_URL, 'confirmed');

                // Get fresh blockhash and rebuild transaction
                const { blockhash } = await connection.getLatestBlockhash('confirmed');
                const transactionBuffer = Buffer.from(transactionBase64, 'base64');
                const transaction = Transaction.from(transactionBuffer);

                // Update with fresh blockhash
                transaction.recentBlockhash = blockhash;
                transaction.feePayer = solana.publicKey;

                const signedTransaction = await solana.signTransaction(transaction);

                // Send transaction
                const signature = await connection.sendRawTransaction(signedTransaction.serialize(), {
                    skipPreflight: false,
                    preflightCommitment: 'confirmed',
                    maxRetries: 0
                });

                // Wait for confirmation
                const confirmation = await connection.confirmTransaction({
                    signature,
                    blockhash,
                    lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight
                }, 'confirmed');

                if (confirmation.value.err) {
                    throw new Error(`Transaction failed: ${confirmation.value.err}`);
                }

                // Reset accumulated rewards
                setAccumulatedRewards(prev => prev ? { ...prev, accumulated_rewards: 0 } : null);
                
                if (onSuccess) {
                    onSuccess();
                }
            } else {
                throw new Error(response.error || 'Failed to create batch claim transaction');
            }
        } catch (err) {
            const errorMessage = err instanceof Error ? err.message : 'Failed to claim tokens';
            setError(errorMessage);
            if (onError) {
                onError(errorMessage);
            }
            console.error('Error claiming tokens:', err);
        } finally {
            setClaiming(false);
        }
    };

    if (!connected) {
        return (
            <div className="card">
                <div className="card-body text-center">
                    <h5>Connect Wallet to View Accumulated Rewards</h5>
                    <button className="btn btn-primary" onClick={connect}>
                        Connect Phantom Wallet
                    </button>
                </div>
            </div>
        );
    }

    if (loading) {
        return (
            <div className="card">
                <div className="card-body text-center">
                    <div className="spinner-border" role="status">
                        <span className="visually-hidden">Loading...</span>
                    </div>
                    <p className="mt-2">Loading accumulated rewards...</p>
                </div>
            </div>
        );
    }

    return (
        <div className="card">
            <div className="card-body">
                <h5 className="card-title"> Accumulated Mining Rewards</h5>
                
                {accumulatedRewards && (
                    <div className="mb-3">
                        <div className="row">
                            <div className="col-md-6">
                                <div className="card bg-primary text-white">
                                    <div className="card-body text-center">
                                        <div className="fs-2 fw-bold">
                                            {accumulatedRewards.accumulated_rewards.toLocaleString()}
                                        </div>
                                        <small>SNAKE Tokens Ready to Claim</small>
                                    </div>
                                </div>
                            </div>
                            <div className="col-md-6">
                                <div className="card">
                                    <div className="card-body text-center">
                                        <div className="fs-2 fw-bold">
                                            {accumulatedRewards.can_claim ? ' Ready' : ' Waiting for TCE'}
                                        </div>
                                        <small>Claim Status</small>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                )}

                {error && (
                    <div className="alert alert-danger">
                        {error}
                    </div>
                )}

                <div className="text-center">
                    {accumulatedRewards && accumulatedRewards.accumulated_rewards > 0 ? (
                        accumulatedRewards.can_claim ? (
                            <button 
                                className="btn btn-success btn-lg"
                                onClick={handleBatchClaim}
                                disabled={claiming}
                            >
                                {claiming ? (
                                    <>
                                        <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                                        Claiming...
                                    </>
                                ) : (
                                    `Claim ${accumulatedRewards.accumulated_rewards.toLocaleString()} SNAKE Tokens`
                                )}
                            </button>
                        ) : (
                            <div className="sn-success py-2 px-2 rounded align-items-center justify-content-center">
                                <h6> Token Claim Event (TCE) Not Yet Active</h6>
                                <p className='mb-0'>Your rewards are safely accumulated off-chain. You'll be able to claim all your tokens in a single transaction once the TCE begins!</p>
                            </div>
                        )
                    ) : (
                        <div className="alert alert-secondary">
                            <h6>No Accumulated Rewards</h6>
                            <p>Start mining tweets to accumulate rewards!</p>
                        </div>
                    )}
                </div>

                <div className="mt-3">
                    <small className="text-muted">
                         <strong>How it works:</strong> Your mining rewards are accumulated off-chain to save on transaction fees. 
                        After the Token Claim Event (TCE), you can claim all your rewards in a single transaction!
                    </small>
                </div>
            </div>
        </div>
    );
};

export default BatchClaimComponent;