import React, { useState, useEffect, useCallback } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import ResponsiveMenu from "../../components/ResponsiveMenu";
import WalletGuard from "../../components/WalletGuard";
import BatchClaimComponent from "../../components/BatchClaimComponent";
import AutomatedTweetComponent from "../../components/AutomatedTweetComponent";
import { useAuth } from '../../contexts/AuthContext';
import { userApi } from '../patron/services/apiService';
import { useErrorHandler } from '../../hooks/useErrorHandler';
import { useToast } from '../../contexts/ToastContext';
import StatusBar from '../../components/StatusBar';

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
    total_tweets: number;
    phase1_count: number;
    phase2_count: number;
    pending_rewards: number;
    total_rewards_claimed: number;
    accumulated_rewards: number;
    current_phase: string;
}

function TweetMiningPage() {
    const navigate = useNavigate();
    const { id: rewardId } = useParams<{ id: string }>();
    const { user, logout } = useAuth();
    const [tweets, setTweets] = useState<Tweet[]>([]);
    const [miningStats, setMiningStats] = useState<MiningStats | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const [isMining, setIsMining] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);
    const [rewards, setRewards] = useState<any[]>([]);
    const [showEducationalModal, setShowEducationalModal] = useState(false);
    const [claimLinkReward, setClaimLinkReward] = useState<any>(null);

    const { handleError, isUserRejection } = useErrorHandler();
    const { showSuccess, showError, showInfo } = useToast();
    
    const checkClaimLinkAccess = useCallback(async (rewardId: string) => {
        try {
            console.log('🔍 Checking claim link for reward ID:', rewardId);
            console.log('👤 Current user ID:', user?.id);
            
            const response = await userApi.getRewardById(rewardId);
            console.log('📡 API Response:', response);
            
            if (response.success && response.data) {
                setClaimLinkReward(response.data);
                console.log('Reward data:', response.data);
                console.log('Reward owner ID:', response.data.user_id);
                
                // Check if the current user owns this reward
                if (response.data.user_id !== user?.id) {
                    console.log('❌ User does not own this reward - showing educational modal');
                    // User doesn't own this reward - show educational modal
                    setShowEducationalModal(true);
                } else {
                    console.log('User owns this reward');
                    // User owns this reward - show success message and highlight it
                    showSuccess('This is your reward! You can claim it below.');
                }
            } else {
                console.log('❌ API call failed:', response.error);
                showError('❌ Invalid or expired reward link.');
                setShowEducationalModal(true);
            }
        } catch (err) {
            console.error('Failed to check claim link:', err);
            showError('❌ Failed to verify reward link.');
            setShowEducationalModal(true);
        }
    }, [user?.id, showSuccess, showError]);

    useEffect(() => {
        if (user) {
            loadMiningStats();
            loadData();
            
            // If there's a reward ID in the URL, check claim link access
            if (rewardId) {
                console.log('🔄 User loaded with reward ID, checking claim link access...');
                checkClaimLinkAccess(rewardId);
            }
        }
    }, [user, rewardId, checkClaimLinkAccess]); // Added checkClaimLinkAccess as dependency

    // Handle claim link access
    useEffect(() => {
        if (rewardId && user) {
            console.log('🔄 User logged in, checking claim link access...');
            checkClaimLinkAccess(rewardId);
        }
    }, [rewardId, user?.id, checkClaimLinkAccess]); // Use user.id instead of user to trigger when user actually changes

    const loadData = async () => {
        await loadUserRewards();
        await loadUserTweets();
    };

    const loadMiningStats = async () => {
        try {
            const response = await userApi.getTweetMiningStatus();
            if (response.success && response.data) {
                setMiningStats({
                    current_phase: 'Phase ' + response.data.current_phase || 'Phase', // Current mining phase
                    total_rewards_claimed: response.data.total_rewards_claimed,
                    pending_rewards: response.data.pending_rewards,
                    total_tweets: response.data.total_tweets,
                    phase1_count: response.data.phase1_count || 0,
                    phase2_count: response.data.phase2_count || 0,
                    accumulated_rewards: response.data.accumulated_rewards || 0
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
                setSuccess(`Reward logged successfully! Accumulated rewards: ${response.data.accumulated_rewards} SNAKE tokens. You can claim all rewards after TCE.`);
                showSuccess(`Reward logged successfully! Accumulated rewards: ${response.data.accumulated_rewards} SNAKE tokens. You can claim all rewards after TCE.`);
                
                //  Mark as rewarded locally
                setTweets(prev =>
                    prev.map(tweet =>
                        tweet.tweet_id === tweetId
                            ? { ...tweet, rewarded: true }
                            : tweet
                    )
                );

                //  Update rewards state
                setRewards(prev =>
                    prev.map(reward =>
                        reward.tweet_id === tweets.find(t => t.tweet_id === tweetId)?.id
                            ? { ...reward, available: false, claimed: true }
                            : reward
                    )
                );

                setRewardFlag(tweetId);

                //  Reload data from backend to ensure consistency
                setTimeout(async () => {
                    await loadMiningStats();
                }, 1000);
            } else {
                throw new Error(response.error || 'Failed to log reward');
            }
        } catch (err) {
            console.error(err);
            setError('Failed to log reward. Please try again.');
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
                    <StatusBar title="MINE TWEETS" />

                    <div className="custom-border-y custom-content-height d-flex flex-column">
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
                                                                {miningStats.total_tweets}
                                                            </div>
                                                            <small className="text-muted">Total Tweets</small>
                                                        </div>
                                                    </div>
                                                    <div className="col-md-3">
                                                        <div className="text-center">
                                                            <div className="fs-4 fw-bold text-success">
                                                                {formatTokenAmount(miningStats.accumulated_rewards)}
                                                            </div>
                                                            <small className="text-muted">Accumulated Rewards</small>
                                                        </div>
                                                    </div>
                                                    <div className="col-md-3">
                                                        <div className="text-center">
                                                            <div className="fs-4 fw-bold text-warning">
                                                                {formatTokenAmount(miningStats.phase1_count)}
                                                            </div>
                                                            <small className="text-muted">Phase 1 Tweets</small>
                                                        </div>
                                                    </div>
                                                    <div className="col-md-3">
                                                        <div className="text-center">
                                                            <div className="fs-4 fw-bold text-info">
                                                                {formatTokenAmount(miningStats.phase2_count)}
                                                            </div>
                                                            <small className="text-muted">Phase 2 Tweets</small>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            )}

                            {/* Batch Claim Component */}
                            <div className="row mb-4">
                                <div className="col-12">
                                    
                                </div>
                            </div>

                            {/* Automated Tweet Component */}
                            <div className="row mb-4">
                                <div className="col-12">
                                    <AutomatedTweetComponent />
                                </div>
                            </div>

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
                                                    (miningStats?.total_tweets || 0) > 0 ? 'Continue Mining' : ' Start Mining'
                                                )}
                                            </button>

                                            {/* {miningStats && miningStats.accumulated_rewards > 0 && (
                                                <div className="mt-3">
                                                    <button
                                                        className="btn btn-outline-success"
                                                        onClick={() => navigate('/claim')}
                                                    >
                                                         View Claim Page ({miningStats.accumulated_rewards.toLocaleString()} SNAKE accumulated)
                                                    </button>
                                                </div>
                                            )} */}

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
                                            {/* <button
                                                className="primary-btn py-1"
                                                onClick={loadData}
                                                disabled={isLoading}
                                            >
                                                Refresh
                                            </button> */}
                                        </div>
                                        <div className="card-body" style={{ maxHeight: '60vh', overflowY: 'auto' }}>
                                            {tweets.length > 0 ? (
                                                <div className="row">
                                                    {tweets.map((tweet, index) => (
                                                        <div key={tweet.id} className="col-12 mb-3">
                                                            <div className={`card ${tweet.rewarded ? 'border-success' : 'border-primary'}`}>
                                                                <div className="card-body">
                                                                    <div className="d-md-flex justify-content-between align-items-start">
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
                                                                                        ? 'Reward already logged'
                                                                                        : `Log ${formatTokenAmount(tweet.reward_amount || 0)} SNAKE reward`
                                                                                }
                                                                            >
                                                                                {tweet.rewarded ? (
                                                                                    ' Logged'
                                                                                ) : (
                                                                                    `Log ${formatTokenAmount(tweet.reward_amount || 0)} SNAKE`
                                                                                )}
                                                                            </button>
                                                                            <a
                                                                                href={`https://twitter.com/intent/retweet?tweet_id=${tweet.tweet_id}`}
                                                                                target="_blank"
                                                                                rel="noopener noreferrer"
                                                                                className="btn btn-outline-primary btn-sm"
                                                                                title="Retweet"
                                                                            >
                                                                                View
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
                                                    <div className="fs-1 mb-3"></div>
                                                    <h5 className="text-muted">No tweets found</h5>
                                                    <p className="text-muted">
                                                        Click "{(miningStats?.total_tweets || 0) > 0 ? 'Continue Mining' : ' Start Mining'}" to fetch your recent tweets and start earning rewards!
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

            {/* Educational Modal for Non-Owners */}
            {showEducationalModal && (
                <div className="modal fade show" style={{ display: 'block', backgroundColor: 'rgba(0,0,0,0.5)' }} tabIndex={-1}>
                    <div className="modal-dialog modal-lg">
                        <div className="modal-content">
                            <div className="modal-header">
                                <h5 className="modal-title">🚫 This Reward Doesn't Belong to You</h5>
                                <button 
                                    type="button" 
                                    className="btn-close" 
                                    onClick={() => setShowEducationalModal(false)}
                                ></button>
                            </div>
                            <div className="modal-body">
                                <div className="text-center mb-4">
                                    <div className="fs-1 mb-3"></div>
                                    <h4 className="text-primary">Want to Earn Your Own SNAKE Tokens?</h4>
                                    <p className="text-muted">
                                        This reward link belongs to someone else, but you can easily mine your own tokens!
                                    </p>
                                </div>

                                <div className="row">
                                    <div className="col-md-6">
                                        <div className="card h-100">
                                            <div className="card-body text-center">
                                                <div className="fs-2 mb-3"></div>
                                                <h5 className="card-title">Step 1: Tweet Mining</h5>
                                                <p className="card-text">
                                                    Post tweets mentioning <strong>@playSnakeAI</strong> and <strong>#MineTheSnake</strong> to earn rewards!
                                                </p>
                                                <div className="alert alert-info">
                                                    <small>
                                                        <strong>Example:</strong><br/>
                                                        "Just discovered @playSnakeAI! This AI-powered snake game is amazing! #MineTheSnake #GameFi"
                                                    </small>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    <div className="col-md-6">
                                        <div className="card h-100">
                                            <div className="card-body text-center">
                                                <div className="fs-2 mb-3"></div>
                                                <h5 className="card-title">Step 2: Start Mining</h5>
                                                <p className="card-text">
                                                    Connect your Twitter account and use our mining page to automatically detect and reward your tweets.
                                                </p>
                                                <div className="alert alert-success">
                                                    <small>
                                                        <strong>Rewards:</strong><br/>
                                                        Phase 1: 60 - 375 SNAKE per tweet<br/>
                                                        Phase 2: 40 SNAKE per tweet
                                                    </small>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div className="mt-4">
                                    <div className="alert alert-warning">
                                        <h6> Mining Requirements:</h6>
                                        <ul className="mb-0">
                                            <li>Tweet must mention <strong>@playSnakeAI</strong></li>
                                            <li>Tweet must include <strong>#MineTheSnake</strong> hashtag</li>
                                            <li>Must not be a reply to another tweet</li>
                                            <li>Account must be connected via Twitter OAuth</li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                            <div className="modal-footer">
                                <button 
                                    type="button" 
                                    className="btn btn-secondary" 
                                    onClick={() => setShowEducationalModal(false)}
                                >
                                    Close
                                </button>
                                <button 
                                    type="button" 
                                    className="btn btn-primary" 
                                    onClick={() => {
                                        setShowEducationalModal(false);
                                        // If user is not authenticated, they'll need to login first
                                        if (!user?.twitter_username) {
                                            showInfo('Please connect your Twitter account first to start mining!');
                                        }
                                    }}
                                >
                                    Start Mining My Own Tokens
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
}

export default TweetMiningPage;
