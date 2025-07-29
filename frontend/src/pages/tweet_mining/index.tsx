import React, { useState, useEffect } from 'react';
import ResponsiveMenu from "../../components/ResponsiveMenu";
import WalletGuard from "../../components/WalletGuard";
import { useAuth } from '../../contexts/AuthContext';
import { userApi } from '../patron/services/apiService';
import { Transaction, Connection, clusterApiUrl } from '@solana/web3.js';
import { Buffer } from 'buffer';

interface Tweet {
    id: string;
    tweet_id: string;
    content: string;
    created_at: string;
    rewarded: boolean;
    reward_amount?: number;
}

interface MiningStats {
    current_stage: string;
    total_reward_amount: number;
    new_reward_amount: number;
    total_tweets: number;
    rewarded_tweets: number;
    pending_tweets: number;
}

function TweetMiningPage() {
    const { user } = useAuth();
    const [tweets, setTweets] = useState<Tweet[]>([]);
    const [miningStats, setMiningStats] = useState<MiningStats | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const [isMining, setIsMining] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);

    useEffect(() => {
        if (user) {
            loadMiningStats();
            loadUserTweets();
        }
    }, [user]);

    const loadMiningStats = async () => {
        try {
            const response = await userApi.getTweetMiningStatus();
            if (response.success && response.data) {
                setMiningStats({
                    current_stage: "Phase 2", // Current mining phase
                    total_reward_amount: response.data.total_rewards_claimed * 1000000000, // Convert to lamports
                    new_reward_amount: response.data.pending_rewards * 1000000000,
                    total_tweets: response.data.total_tweets,
                    rewarded_tweets: response.data.total_rewards_claimed,
                    pending_tweets: response.data.pending_rewards
                });
            }
        } catch (err) {
            console.error('Failed to load mining stats:', err);
        }
    };

    const loadUserTweets = async () => {
        try {
            const response = await userApi.getTweets(0, 50);
            if (response.success && response.data) {
                const tweetsData = response.data.map(tweet => ({
                    id: tweet.id,
                    tweet_id: tweet.tweet_id,
                    content: tweet.content || `Tweet content from @${tweet.twitter_username}`,
                    created_at: tweet.created_at,
                    rewarded: false, // Will be updated based on rewards data
                    reward_amount: 1000000000 // 1 token in lamports
                }));
                setTweets(tweetsData);
            }
        } catch (err) {
            console.error('Failed to load tweets:', err);
        }
    };

    const handleStartMining = async () => {
        if (!user?.twitter_username) {
            setError('Please authenticate with Twitter/X first');
            return;
        }

        setIsLoading(true);
        setIsMining(true);
        setError(null);
        setSuccess(null);

        try {
            // Simulate fetching tweets from Twitter API
            const response = await userApi.startTweetMining();
            
            if (response.success) {
                setSuccess(`Successfully started mining! Found ${response.data?.tweets_found || 0} tweets.`);
                await loadUserTweets();
                await loadMiningStats();
            } else {
                setError(response.error || 'Failed to start mining');
            }
        } catch (err) {
            setError('Failed to start mining. Please try again.');
        } finally {
            setIsLoading(false);
            setIsMining(false);
        }
    };

    const handleClaimReward = async (tweetId: string) => {
        setIsLoading(true);
        setError(null);
        setSuccess(null);

        try {
            const response = await userApi.claimTweetReward(tweetId);
            
            if (response.success && response.data) {
                // Decode the base64 transaction
                const transactionData = response.data;
                
                // Check if Phantom wallet is available
                const solana = (window as any).solana;
                if (!solana || !solana.isPhantom) {
                    throw new Error('Phantom wallet not found. Please install Phantom wallet.');
                }

                // Ensure wallet is connected
                if (!solana.isConnected) {
                    await solana.connect();
                }

                // Decode base64 transaction
                const transactionBuffer = Buffer.from(transactionData, 'base64');
                
                setSuccess('Transaction received. Signing with wallet...');
                
                try {
                    // Create Solana connection (use devnet for development, mainnet-beta for production)
                    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
                    
                    // Deserialize the transaction
                    const transaction = Transaction.from(transactionBuffer);
                    
                    // Sign the transaction with Phantom wallet
                    const signedTransaction = await solana.signTransaction(transaction);
                    
                    // Submit the signed transaction to Solana network
                    const signature = await connection.sendRawTransaction(
                        signedTransaction.serialize(),
                        {
                            skipPreflight: false,
                            preflightCommitment: 'confirmed'
                        }
                    );
                    
                    setSuccess(`Reward claimed successfully! Transaction: ${signature}`);
                    console.log('Transaction signature:', signature);
                    
                    // Wait for confirmation
                    await connection.confirmTransaction(signature, 'confirmed');
                    
                    // Update local state to mark as rewarded
                    setTweets(prev => prev.map(tweet => 
                        tweet.tweet_id === tweetId 
                            ? { ...tweet, rewarded: true }
                            : tweet
                    ));
                    
                    await loadMiningStats();
                } catch (walletError: any) {
                    console.error('Wallet signing error:', walletError);
                    setError(`Failed to process transaction: ${walletError?.message || walletError}`);
                }
            } else {
                setError(response.error || 'Failed to claim reward');
            }
        } catch (err) {
            console.log(err)
            setError('Failed to claim reward. Please try again.');
        } finally {
            setIsLoading(false);
        }
    };

    const formatTokenAmount = (lamports: number) => {
        return (lamports / 1000000000).toFixed(2);
    };

    return (
        <div className="w-100 p-3" style={{ height: "100vh" }}>
            <div className="d-flex gap-4" style={{ height: "calc(100vh-60px)", paddingTop: '55px' }}>
                <ResponsiveMenu />
                
                <div className="custom-content">
                    <div className="w-100">
                        <div className="d-flex justify-content-between align-items-center ">
                            <div className="fs-1" style={{ lineHeight: 'normal' }}>
                                MINE TWEETS
                            </div>
                            <div className="text-end">
                                <div className="fs-6 text-muted">
                                    Connected: @{user?.twitter_username || 'Not authenticated'}
                                </div>
                            </div>
                        </div>
                    </div>

                    <div className="custom-border-y custom-content-height d-flex flex-column p-3">
                        <WalletGuard>
                            {/* Mining Stats Panel */}
                            {miningStats && (
                                <div className="row mb-4">
                                    <div className="col-12">
                                        <div className="card bg-light">
                                            <div className="card-body">
                                                <h5 className="card-title"> Mining Statistics</h5>
                                                <div className="row">
                                                    <div className="col-md-3">
                                                        <div className="text-center">
                                                            <div className="fs-4 fw-bold text-primary">
                                                                {miningStats.current_stage}
                                                            </div>
                                                            <small className="text-muted">Current Stage</small>
                                                        </div>
                                                    </div>
                                                    <div className="col-md-3">
                                                        <div className="text-center">
                                                            <div className="fs-4 fw-bold text-success">
                                                                {formatTokenAmount(miningStats.total_reward_amount)} SNAKE
                                                            </div>
                                                            <small className="text-muted">Total Rewards</small>
                                                        </div>
                                                    </div>
                                                    <div className="col-md-3">
                                                        <div className="text-center">
                                                            <div className="fs-4 fw-bold text-warning">
                                                                {formatTokenAmount(miningStats.new_reward_amount)} SNAKE
                                                            </div>
                                                            <small className="text-muted">Pending Rewards</small>
                                                        </div>
                                                    </div>
                                                    <div className="col-md-3">
                                                        <div className="text-center">
                                                            <div className="fs-4 fw-bold text-info">
                                                                {miningStats.total_tweets}
                                                            </div>
                                                            <small className="text-muted">Total Tweets</small>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            )}

                            {/* Start Mining Section */}
                            <div className="row mb-4">
                                <div className="col-12">
                                    <div className="card">
                                        <div className="card-body text-center">
                                            <h5 className="card-title"> Tweet Mining Control</h5>
                                            <p className="card-text">
                                                Start mining to fetch your recent tweets and earn SNAKE tokens for each tweet!
                                            </p>
                                            
                                            {error && (
                                                <div className="alert alert-danger" role="alert">
                                                    {error}
                                                </div>
                                            )}
                                            
                                            {success && (
                                                <div className="alert alert-success" role="alert">
                                                    {success}
                                                </div>
                                            )}

                                            <button
                                                className={`btn btn-lg ${isMining ? 'btn-warning' : 'btn-primary'}`}
                                                onClick={handleStartMining}
                                                disabled={isLoading || !user?.twitter_username}
                                            >
                                                {isLoading ? (
                                                    <>
                                                        <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                                                        Mining...
                                                    </>
                                                ) : isMining ? (
                                                    '⚡ Mining Active'
                                                ) : (
                                                    ' Start Mining'
                                                )}
                                            </button>
                                            
                                            {!user?.twitter_username && (
                                                <div className="mt-3">
                                                    <small className="text-muted">
                                                        Please authenticate with Twitter/X to start mining
                                                    </small>
                                                </div>
                                            )}
                                        </div>
                                    </div>
                                </div>
                            </div>

                            {/* Tweets List */}
                            <div className="row">
                                <div className="col-12">
                                    <div className="card">
                                        <div className="card-header d-flex justify-content-between align-items-center">
                                            <h5 className="mb-0">Your Tweets ({tweets.length})</h5>
                                            <button 
                                                className="btn btn-outline-primary btn-sm"
                                                onClick={loadUserTweets}
                                                disabled={isLoading}
                                            >
                                                🔄 Refresh
                                            </button>
                                        </div>
                                        <div className="card-body" style={{ maxHeight: '60vh', overflowY: 'auto' }}>
                                            {tweets.length > 0 ? (
                                                <div className="row">
                                                    {tweets.map((tweet, index) => (
                                                        <div key={tweet.id} className="col-12 mb-3">
                                                            <div className={`card ${tweet.rewarded ? 'border-success' : 'border-primary'}`}>
                                                                <div className="card-body">
                                                                    <div className="d-flex justify-content-between align-items-start">
                                                                        <div className="flex-grow-1">
                                                                            <h6 className="card-title mb-2">
                                                                                Tweet #{index + 1}
                                                                                {tweet.rewarded && (
                                                                                    <span className="badge bg-success ms-2">Rewarded</span>
                                                                                )}
                                                                            </h6>
                                                                            <p className="card-text mb-2">{tweet.content}</p>
                                                                            <small className="text-muted">
                                                                                {new Date(tweet.created_at).toLocaleString()} • 
                                                                                ID: {tweet.tweet_id}
                                                                            </small>
                                                                        </div>
                                                                        <div className="d-flex flex-column gap-2 ms-3">
                                                                            <button
                                                                                className={`btn btn-sm ${
                                                                                    tweet.rewarded 
                                                                                        ? 'btn-success disabled' 
                                                                                        : 'btn-outline-success'
                                                                                }`}
                                                                                onClick={() => handleClaimReward(tweet.tweet_id)}
                                                                                disabled={isLoading || tweet.rewarded}
                                                                                title={
                                                                                    tweet.rewarded 
                                                                                        ? 'Reward already claimed' 
                                                                                        : `Claim ${formatTokenAmount(tweet.reward_amount || 0)} SNAKE`
                                                                                }
                                                                            >
                                                                                {tweet.rewarded ? (
                                                                                    '✅ Claimed'
                                                                                ) : (
                                                                                    `💰 Claim ${formatTokenAmount(tweet.reward_amount || 0)}`
                                                                                )}
                                                                            </button>
                                                                            <a
                                                                                href={`https://twitter.com/intent/retweet?tweet_id=${tweet.tweet_id}`}
                                                                                target="_blank"
                                                                                rel="noopener noreferrer"
                                                                                className="btn btn-outline-primary btn-sm"
                                                                                title="Retweet"
                                                                            >
                                                                                🔄 RT
                                                                            </a>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    ))}
                                                </div>
                                            ) : (
                                                <div className="text-center py-5">
                                                    <div className="fs-1 mb-3">🐦</div>
                                                    <h5 className="text-muted">No tweets found</h5>
                                                    <p className="text-muted">
                                                        Click "Start Mining" to fetch your recent tweets and start earning rewards!
                                                    </p>
                                                </div>
                                            )}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </WalletGuard>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default TweetMiningPage;
