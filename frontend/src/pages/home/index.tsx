
import { useNavigate } from "react-router-dom";

import ResponsiveMenu from "../../components/ResponsiveMenu";
import SimpleWalletConnection from "../patron/components/SimpleWalletConnection";
import { useAuth } from "../../contexts/AuthContext";
import { useWalletContext } from "../../contexts/WalletContext";
import { usePhantom } from "../../hooks/usePhantom";
import './index.css';

function Home() {
    // eslint-disable-next-line no-empty-pattern
    const { } = usePhantom();
    const navigate = useNavigate();
    const { user } = useAuth();
    const { connected } = useWalletContext();

    return (
        <div className="w-100 p-3" style={{ height: '100vh' }}>
            <div className="d-flex gap-3" style={{ height: "calc(100vh-60px)", paddingTop: '35px' }}>
                {/* Menu Begin */}
                <ResponsiveMenu />
                {/* Menu End */}
               
                <div className="custom-content">
                    <div className="w-100">
                        <div className="fs-1" style={{ lineHeight: 'normal' }}>
                            🎮 DASHBOARD
                        </div>
                        <div className="fs-6 text-muted mb-3">
                            Welcome back, {user?.twitter_username}! Here's your guide to Snake AI.
                        </div>
                        <hr className="border border-dashed border-black border-3 opacity-100" />
                    </div>
                    
                    <div className="custom-border-y custom-content-height">
                        {/* Welcome Section */}
                        <div className="row g-4 mb-4">
                            <div className="col-lg-8">
                                <div className="card border-3 border-dashed h-100">
                                    <div className="card-body">
                                        <h3 className="card-title mb-3">🚀 Getting Started Guide</h3>
                                        <div className="row g-3">
                                            <div className="col-md-6">
                                                <div className={`card border-2 ${connected ? 'border-success bg-light-success' : 'border-warning'}`}>
                                                    <div className="card-body text-center">
                                                        <div className="fs-2 mb-2">{connected ? '✅' : '🔗'}</div>
                                                        <h6 className="card-title">1. Connect Wallet</h6>
                                                        <p className="small text-muted">Link your Phantom wallet to access all features</p>
                                                        {!connected && <SimpleWalletConnection />}
                                                        {connected && <span className="badge bg-success">✓ Wallet Connected</span>}
                                                    </div>
                                                </div>
                                            </div>
                                            <div className="col-md-6">
                                                <div className="card border-2 border-info" style={{ cursor: 'pointer' }} onClick={() => navigate('/tweet-mining')}>
                                                    <div className="card-body text-center">
                                                        <div className="fs-2 mb-2">🐦</div>
                                                        <h6 className="card-title">2. Start Mining</h6>
                                                        <p className="small text-muted">Tweet about Snake AI to earn tokens</p>
                                                        <button className="btn btn-info btn-sm">Start Mining →</button>
                                                    </div>
                                                </div>
                                            </div>
                                            <div className="col-md-6">
                                                <div className="card border-2 border-primary" style={{ cursor: 'pointer' }} onClick={() => navigate('/patron-framework')}>
                                                    <div className="card-body text-center">
                                                        <div className="fs-2 mb-2">👑</div>
                                                        <h6 className="card-title">3. Choose Role</h6>
                                                        <p className="small text-muted">Become a Staker or apply for Patron status</p>
                                                        <button className="btn btn-primary btn-sm">Select Role →</button>
                                                    </div>
                                                </div>
                                            </div>
                                            <div className="col-md-6">
                                                <div className="card border-2 border-success" style={{ cursor: 'pointer' }} onClick={() => navigate('/claim')}>
                                                    <div className="card-body text-center">
                                                        <div className="fs-2 mb-2">🎁</div>
                                                        <h6 className="card-title">4. Claim Rewards</h6>
                                                        <p className="small text-muted">Collect your earned Snake tokens</p>
                                                        <button className="btn btn-success btn-sm">Claim Now →</button>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            <div className="col-lg-4">
                                <div className="card border-3 border-dashed h-100">
                                    <div className="card-body">
                                        <h3 className="card-title mb-3">📊 Your Status</h3>
                                        <div className="mb-3">
                                            <div className="d-flex justify-content-between align-items-center mb-2">
                                                <span>🐦 Tweets Mined:</span>
                                                <span className="badge bg-info">0</span>
                                            </div>
                                            <div className="d-flex justify-content-between align-items-center mb-2">
                                                <span>🪙 Tokens Earned:</span>
                                                <span className="badge bg-warning">0</span>
                                            </div>
                                            <div className="d-flex justify-content-between align-items-center mb-2">
                                                <span>👑 Current Role:</span>
                                                <span className="badge bg-secondary">None</span>
                                            </div>
                                            <div className="d-flex justify-content-between align-items-center">
                                                <span>🔗 Wallet:</span>
                                                <span className={`badge ${connected ? 'bg-success' : 'bg-danger'}`}>
                                                    {connected ? 'Connected' : 'Not Connected'}
                                                </span>
                                            </div>
                                        </div>
                                        <hr />
                                        <h5 className="mb-3">⚡ Quick Actions</h5>
                                        <div className="d-grid gap-2">
                                            <button onClick={() => navigate('/tweet-mining')} className="btn btn-info btn-sm">
                                                🐦 Tweet Mining
                                            </button>
                                            <button onClick={() => navigate('/patron-framework')} className="btn btn-warning btn-sm">
                                                👑 Patron Framework
                                            </button>
                                            <button onClick={() => navigate('/meme-generation')} className="btn btn-primary btn-sm">
                                                🎨 Generate Memes
                                            </button>
                                            <button onClick={() => navigate('/get-started')} className="btn btn-outline-secondary btn-sm">
                                                📖 Learn More
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        {/* Mining Progress & Tips */}
                        <div className="row g-4 mb-4">
                            <div className="col-lg-8">
                                <div className="card border-3 border-dashed">
                                    <div className="card-body">
                                        <h3 className="card-title mb-3">⛏️ Current Mining Phase</h3>
                                        <div className="d-flex align-items-center">
                                            <div className="bg-black p-3 px-4 text-white rounded-3 me-4" style={{ minWidth: '180px'}}>
                                                <div className="fs-4 fw-bold">PHASE 1</div>
                                                <div className="fs-6">MINING EPOCH</div>
                                            </div>
                                            <div className="flex-fill">
                                                <div className="d-flex justify-content-between mb-2">
                                                    <span><strong>Mining Progress:</strong></span>
                                                    <span className="text-muted">5,000,031 / 1,000,000 tweets</span>
                                                </div>
                                                <div className="progress" style={{ height: '20px' }}>
                                                    <div className="progress-bar bg-success" role="progressbar" style={{ width: "50%" }}>
                                                        50%
                                                    </div>
                                                </div>
                                                <div className="text-muted small mt-1">
                                                    Phase 1 ends when 1M tweets are mined or time expires
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div className="col-lg-4">
                                <div className="card border-3 border-dashed">
                                    <div className="card-body">
                                        <h5 className="card-title mb-3">💡 Pro Tips</h5>
                                        <ul className="list-unstyled">
                                            <li className="mb-2">
                                                <span className="badge bg-primary me-2">1</span>
                                                <small>Tweet daily for consistent rewards</small>
                                            </li>
                                            <li className="mb-2">
                                                <span className="badge bg-success me-2">2</span>
                                                <small>Use hashtags like #SnakeAI for better engagement</small>
                                            </li>
                                            <li className="mb-2">
                                                <span className="badge bg-warning me-2">3</span>
                                                <small>Connect wallet before mining to save progress</small>
                                            </li>
                                            <li>
                                                <span className="badge bg-info me-2">4</span>
                                                <small>Apply for Patron status for exclusive benefits</small>
                                            </li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Home;
