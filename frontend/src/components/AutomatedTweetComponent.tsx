import React, { useState, useEffect, useCallback, useRef } from 'react';
import { userApi } from '../pages/patron/services/apiService';
import { useToast } from '../contexts/ToastContext';
import BatchClaimComponent from "./BatchClaimComponent";

interface TweetTemplate {
    id: number;
    content: string;
    category?: string;
}

interface AutomatedTweetComponentProps {
    className?: string;
}

const AutomatedTweetComponent: React.FC<AutomatedTweetComponentProps> = ({ className = '' }) => {
    const [templates, setTemplates] = useState<TweetTemplate[]>([]);
    const [categories, setCategories] = useState<string[]>([]);
    const [selectedCategory, setSelectedCategory] = useState<string>('');
    const [isLoading, setIsLoading] = useState(false);
    const [isPosting, setIsPosting] = useState(false);
    const [activeTab, setActiveTab] = useState<string>('mining');
    const [lastPostTime, setLastPostTime] = useState<Date | null>(null);
    const [timeUntilNextPost, setTimeUntilNextPost] = useState<number>(0);

    // Use refs to store timer IDs
    const postCooldownTimerRef = useRef<NodeJS.Timeout | null>(null);

    const { showSuccess, showError, showInfo } = useToast();

    // Post cooldown interval in milliseconds (25 minutes)
    const POST_COOLDOWN_INTERVAL = 25 * 60 * 1000;

    const loadTweetTemplates = useCallback(async (showLoadingIndicator = true) => {
        if (showLoadingIndicator) {
            setIsLoading(true);
        }
        
        try {
            const response = await userApi.getTweetTemplates(5, selectedCategory || undefined);
            if (response.success && response.data) {
                setTemplates(response.data.templates);
                setCategories(response.data.categories);
                
                if (!showLoadingIndicator) {
                    showInfo('🔄 Tweet templates refreshed with fresh content!');
                }
            } else {
                showError('Failed to load tweet templates');
            }
        } catch (error) {
            console.error('Failed to load tweet templates:', error);
            showError('Failed to load tweet templates');
        } finally {
            if (showLoadingIndicator) {
                setIsLoading(false);
            }
        }
    }, [selectedCategory, showError, showInfo]);

    // Separate function to clear timers
    const clearTimers = useCallback(() => {
        if (postCooldownTimerRef.current) {
            clearInterval(postCooldownTimerRef.current);
            postCooldownTimerRef.current = null;
        }
    }, []);



    // Setup post cooldown timer
    const setupPostCooldownTimer = useCallback(() => {
        if (!lastPostTime) {
            setTimeUntilNextPost(0);
            return;
        }

        if (postCooldownTimerRef.current) {
            clearInterval(postCooldownTimerRef.current);
        }

        const updatePostCooldown = () => {
            const now = new Date().getTime();
            const nextPostTime = lastPostTime.getTime() + POST_COOLDOWN_INTERVAL;
            const timeLeft = Math.max(0, nextPostTime - now);
            setTimeUntilNextPost(timeLeft);

            if (timeLeft === 0) {
                if (postCooldownTimerRef.current) {
                    clearInterval(postCooldownTimerRef.current);
                    postCooldownTimerRef.current = null;
                }
                // Refresh templates when cooldown ends
                loadTweetTemplates(false);
            }
        };

        // Initial update
        updatePostCooldown();

        const cooldownTimer = setInterval(updatePostCooldown, 1000);
        postCooldownTimerRef.current = cooldownTimer;
    }, [lastPostTime, POST_COOLDOWN_INTERVAL, loadTweetTemplates]);

    // Load templates when component mounts or category changes
    useEffect(() => {
        if (activeTab === 'tweets') {
            loadTweetTemplates();
        }
    }, [selectedCategory, activeTab]);

    // Cleanup timers when component unmounts
    useEffect(() => {
        return () => {
            clearTimers();
        };
    }, [clearTimers]);

    // Setup post cooldown timer when lastPostTime changes
    useEffect(() => {
        if (activeTab === 'tweets') {
            setupPostCooldownTimer();
        }
    }, [setupPostCooldownTimer, activeTab]);

    const handlePostTweet = async (templateId: number, content: string) => {
        setIsPosting(true);
        try {
            const response = await userApi.postTweet({ template_id: templateId });
            if (response.success && response.data) {
                if (response.data.success) {
                    showSuccess(`🐦 ${response.data.message}`);
                    // Set the last post time to start cooldown
                    setLastPostTime(new Date());
                } else {
                    showError(response.data.message);
                }
            } else {
                showError('Failed to post tweet');
            }
        } catch (error) {
            console.error('Failed to post tweet:', error);
            showError('Failed to post tweet');
        } finally {
            setIsPosting(false);
        }
    };

    const handleManualRefresh = () => {
        loadTweetTemplates(true);
    };

    const formatTimeUntilRefresh = (milliseconds: number) => {
        const minutes = Math.floor(milliseconds / (1000 * 60));
        const seconds = Math.floor((milliseconds % (1000 * 60)) / 1000);
        return `${minutes}:${seconds.toString().padStart(2, '0')}`;
    };

    const isPostingAllowed = () => {
        return timeUntilNextPost === 0;
    };

    const renderMiningRewardsTab = () => (
        <div>
            <BatchClaimComponent />
        </div>
    );

    const renderTweetsTab = () => (
        <div>
            <div className="d-flex justify-content-between align-items-center mb-3">
                <p className="text-muted mb-0">
                    <small>
                        ✨ Pre-crafted tweets ready to post! Fresh templates will load automatically when your cooldown ends.
                    </small>
                </p>
                <div className="d-flex align-items-center gap-2">
                    <button
                        className="btn btn-outline-primary btn-sm"
                        onClick={handleManualRefresh}
                        disabled={isLoading}
                        title="Refresh templates now"
                    >
                        🔄
                    </button>
                </div>
            </div>

            {/* Post cooldown notification */}
            {!isPostingAllowed() && (
                <div className="alert alert-info mb-3">
                    <div className="d-flex align-items-center">
                        <span className="me-2">⏰</span>
                        <small>
                            <strong>Cooldown active:</strong> You can post your next tweet in {formatTimeUntilRefresh(timeUntilNextPost)}
                        </small>
                    </div>
                </div>
            )}

            {categories.length > 0 && (
                <div className="mb-3">
                    <label className="form-label">Filter by category:</label>
                    <select
                        className="form-select form-select-sm"
                        value={selectedCategory}
                        onChange={(e) => setSelectedCategory(e.target.value)}
                    >
                        <option value="">All categories</option>
                        {categories.map((category) => (
                            <option key={category} value={category}>
                                {category.charAt(0).toUpperCase() + category.slice(1)}
                            </option>
                        ))}
                    </select>
                </div>
            )}

            {templates.length > 0 ? (
                <div className="row">
                    {templates.map((template, index) => (
                        <div key={template.id} className="col-12 mb-3">
                            <div className="card border-light">
                                <div className="card-body">
                                    <div className="d-flex justify-content-between align-items-start">
                                        <div className="flex-grow-1 me-3 w-100">
                                            <p className="card-text mb-2">{template.content}</p>
                                            {template.category && (
                                                <span className="badge bg-secondary">
                                                    {template.category}
                                                </span>
                                            )}
                                        </div>
                                        <button
                                            className="primary-btn p-1"
                                            onClick={() => handlePostTweet(template.id, template.content)}
                                            disabled={isPosting || !isPostingAllowed()}
                                            title={!isPostingAllowed() ? `Cooldown active: ${formatTimeUntilRefresh(timeUntilNextPost)} remaining` : "Post this tweet"}
                                            style={{ width: '100px' }}
                                        >
                                            {isPosting ? (
                                                <>
                                                    <span className="spinner-border spinner-border-sm me-1" role="status" aria-hidden="true"></span>
                                                    Posting...
                                                </>
                                            ) : !isPostingAllowed() ? (
                                                <>
                                                    ⏰ {Math.ceil(timeUntilNextPost / (1000 * 60))}m
                                                </>
                                            ) : (
                                                'Post'
                                            )}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
            ) : (
                <div className="text-center py-4">
                    <div className="fs-1 mb-3">📝</div>
                    <h6 className="text-muted">No templates available</h6>
                    <p className="text-muted mb-0">
                        <small>Try refreshing or selecting a different category</small>
                    </p>
                </div>
            )}
        </div>
    );

    if (activeTab === 'tweets' && isLoading) {
        return (
            <div className={`card ${className}`}>
                <div className="card-body text-center">
                    <div className="spinner-border text-primary" role="status">
                        <span className="visually-hidden">Loading...</span>
                    </div>
                    <p className="mt-2 mb-0">Loading tweet templates...</p>
                </div>
            </div>
        );
    }

    return (
        <div className={`card ${className}`}>
            <div className="sn-bold-border">
                {/* Tab Navigation */}
                <ul className="nav nav-tabs mb-4" role="tablist">
                    <li className="nav-item" role="presentation">
                        <button
                            className={`nav-link ${activeTab === 'mining' ? 'active' : ''}`}
                            onClick={() => setActiveTab('mining')}
                            type="button"
                            role="tab"
                        >
                            ⛏️ Mining Rewards
                        </button>
                    </li>
                    <li className="nav-item" role="presentation">
                        <button
                            className={`nav-link ${activeTab === 'tweets' ? 'active' : ''}`}
                            onClick={() => setActiveTab('tweets')}
                            type="button"
                            role="tab"
                        >
                            🐦 Tweet Assistant
                        </button>
                    </li>
                </ul>

                {/* Tab Content */}
                <div className="tab-content">
                    {activeTab === 'mining' && renderMiningRewardsTab()}
                    {activeTab === 'tweets' && renderTweetsTab()}
                </div>
            </div>
        </div>
    );
};

export default AutomatedTweetComponent;