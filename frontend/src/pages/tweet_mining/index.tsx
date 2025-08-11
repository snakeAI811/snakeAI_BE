import React, { useState, useEffect } from 'react';
import ResponsiveMenu from "../../components/ResponsiveMenu";
import WalletGuard from "../../components/WalletGuard";
import { useAuth } from '../../contexts/AuthContext';
import { userApi } from '../patron/services/apiService';
import { Transaction, Connection, clusterApiUrl, Cluster } from '@solana/web3.js';
import { Buffer } from 'buffer';
import { SOLANA_RPC_URL } from '../../config/program';
import { useErrorHandler } from '../../hooks/useErrorHandler';


interface Tweet {
    id: string;
    tweet_id: string;
    content: string;
    created_at: string;
    rewarded: boolean;
    reward_amount?: number;
    reward_id?: string; // Add reward_id to track the actual reward
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
    const [rewards, setRewards] = useState<any[]>([]);
    const { handleError, isUserRejection } = useErrorHandler();
    

    useEffect(() => {
        if (user) {
            loadMiningStats();
            loadData();
        }
    }, [user]);

    const loadData = async () => {
        await loadUserRewards();
        await loadUserTweets();
    };

    const loadMiningStats = async () => {
        try {
            const response = await userApi.getTweetMiningStatus();
            if (response.success && response.data) {
                setMiningStats({
                    current_stage: 'Phase ' + response.data.current_phase || 'Phase', // Current mining phase
                    total_reward_amount: response.data.total_rewards_claimed,
                    new_reward_amount: response.data.pending_rewards,
                    total_tweets: response.data.total_tweets,
                    rewarded_tweets: response.data.total_rewards_claimed,
                    pending_tweets: response.data.pending_rewards
                });
            }
        } catch (err) {
            console.error('Failed to load mining stats:', err);
        }
    };

    const loadUserRewards = async () => {
        try {
            const response = await userApi.getRewards(0, 100);
            if (response.success && response.data) {
                setRewards(response.data);
            }
        } catch (err) {
            console.error('Failed to load rewards:', err);
        }
    };

    const loadUserTweets = async () => {
        try {
            const response = await userApi.getTweets(0, 50);
            if (response.success && response.data) {
                const tweetsData = response.data
                    .map(tweet => {
                        // Find corresponding reward for this tweet
                        const correspondingReward = rewards.find(reward =>
                            reward.tweet_id === tweet.id
                        );

                        return {
                            id: tweet.id,
                            tweet_id: tweet.tweet_id,
                            content: tweet.content || `Tweet content from @${tweet.twitter_username}`,
                            created_at: tweet.created_at,
                            rewarded: correspondingReward ? !correspondingReward.available : false,
                            reward_amount: correspondingReward ? correspondingReward.reward_amount : 0,
                            reward_id: correspondingReward?.id
                        };
                    })
                    .filter(tweet => tweet.reward_id); // Only keep tweets that have rewards

                setTweets(tweetsData);
            }
        } catch (err) {
            console.error('Failed to load tweets:', err);
        }
    };

    const setRewardFlag = async (tweet_id: string) => {
        const response = await userApi.setRewardFlag(tweet_id);
        if (response.success && response.data) {
            return 'success'
        }
    }

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
            // const response = await userApi.startTweetMining();

            // if (response.success) {
            //     setSuccess(`Successfully started mining! Found ${response.data?.tweets_found || 0} tweets.`);
            await loadData();
            await loadMiningStats();
            // } else {
            //     setError(response.error || 'Failed to start mining');
            // }
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
                const transactionBase64 = response.data;
                const solana = (window as any).solana;
                if (!solana || !solana.isPhantom) {
                    throw new Error('Phantom wallet not found. Please install Phantom.');
                }
                if (!solana.isConnected) {
                    await solana.connect();
                }

                const connection = new Connection(SOLANA_RPC_URL, 'confirmed');

                // ✅ Get fresh blockhash and rebuild transaction
                const { blockhash } = await connection.getLatestBlockhash('confirmed');
                const transactionBuffer = Buffer.from(transactionBase64, 'base64');
                const transaction = Transaction.from(transactionBuffer);

                // ✅ Update with fresh blockhash
                transaction.recentBlockhash = blockhash;
                transaction.feePayer = solana.publicKey;

                setSuccess('Signing transaction...');

                const signedTransaction = await solana.signTransaction(transaction);

                // ✅ Send with better error handling
                const signature = await connection.sendRawTransaction(signedTransaction.serialize(), {
                    skipPreflight: false,
                    preflightCommitment: 'confirmed',
                    maxRetries: 0 // Prevent automatic retries
                });

                setSuccess(`Reward claimed successfully! Transaction: ${signature}`);

                // Wait for confirmation
                const confirmation = await connection.confirmTransaction({
                    signature,
                    blockhash,
                    lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight
                }, 'confirmed');

                if (confirmation.value.err) {
                    throw new Error(`Transaction failed: ${confirmation.value.err}`);
                }

                // ✅ Mark as rewarded locally
                setTweets(prev =>
                    prev.map(tweet =>
                        tweet.tweet_id === tweetId
                            ? { ...tweet, rewarded: true }
                            : tweet
                    )
                );

                // ✅ Update rewards state
                setRewards(prev =>
                    prev.map(reward =>
                        reward.tweet_id === tweets.find(t => t.tweet_id === tweetId)?.id
                            ? { ...reward, available: false, claimed: true }
                            : reward
                    )
                );


                setRewardFlag(tweetId);

                // ✅ Reload data from backend to ensure consistency
                setTimeout(async () => {
                    // await loadData();
                    await loadMiningStats();
                }, 3000);
            }
        } catch (err) {
            console.error(err);
            if (!isUserRejection(err)) {
                handleError(err, 'Failed to select role');
            }
            setError('Failed to claim reward. Please try again.');
        } finally {
            setIsLoading(false);
        }
    };

    const formatTokenAmount = (amount: number) => {
        return amount.toString();
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
                                        <div className="card">
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
                                                                {formatTokenAmount(miningStats.rewarded_tweets)}
                                                            </div>
                                                            <small className="text-muted">Total Rewards</small>
                                                        </div>
                                                    </div>
                                                    <div className="col-md-3">
                                                        <div className="text-center">
                                                            <div className="fs-4 fw-bold text-warning">
                                                                {formatTokenAmount(miningStats.pending_tweets)}
                                                            </div>
                                                            <small className="text-muted">Pending Rewards</small>
                                                        </div>
                                                    </div>
                                                    <div className="col-md-3">
                                                        <div className="text-center">
                                                            <div className="fs-4 fw-bold text-info">
                                                                {formatTokenAmount(miningStats.rewarded_tweets + miningStats.pending_tweets)}
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
                                                className={`${isMining ? 'second-btn' : 'primary-btn'}`}
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
                                                className="primary-btn py-1"
                                                onClick={loadData}
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
                                                                                className={`btn btn-sm ${tweet.rewarded
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
                                                                                    `Claim ${formatTokenAmount(tweet.reward_amount || 0)} SNAKE`
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
