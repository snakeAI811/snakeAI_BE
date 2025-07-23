import React, { useState, useEffect } from 'react';
import ResponsiveMenu from "../../components/ResponsiveMenu";

function TwitterMiningPage() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Check if user is already logged in with Twitter
    const checkAuthStatus = async () => {
      try {
        // You might want to check session or user status here
        setLoading(false);
      } catch (error) {
        console.error('Error checking auth status:', error);
        setLoading(false);
      }
    };
    
    checkAuthStatus();
  }, []);

  const handleTwitterLogin = () => {
    // Redirect to Twitter OAuth login endpoint
    window.location.href = '/api/auth/login';
  };

  if (loading) {
    return (
      <div className="w-100 p-3" style={{ height: "100vh" }}>
        <div className="d-flex justify-content-center align-items-center" style={{ height: "100%" }}>
          <div className="text-center">
            <div className="spinner-border" role="status">
              <span className="visually-hidden">Loading...</span>
            </div>
            <p className="mt-2">Loading...</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="w-100 p-3" style={{ height: "100vh" }}>
      <div className="d-flex gap-4" style={{ height: "calc(100vh-60px)", paddingTop: '55px' }}>
        {/* Menu Begin */}
        <ResponsiveMenu />
        {/* Menu End */}
        
        <div className="item-stretch" style={{ width: '100%' }}>
          <div className="w-100 d-flex justify-content-between gap-4">
            <div className="item-stretch w-100" style={{ minHeight: '86vh' }}>
              
              {/* Header */}
              <div className="w-100">
                <div className="fs-1" style={{ lineHeight: 'normal' }}>
                  🐦 Twitter Mining
                </div>
                <div className="fs-6 text-muted mb-3">
                  Mine Snake AI tokens by tweeting! Follow @playSnakeAI and use #MineTheSnake to earn rewards.
                </div>
                <hr className="border border-dashed border-black border-3 opacity-100" />
              </div>

              <div className="row g-4">
                {/* Login Section */}
                <div className="col-lg-6">
                  <div className="card border-3 border-dashed h-100">
                    <div className="card-body">
                      <h3 className="card-title mb-4">🔑 Get Started</h3>
                      
                      {!isLoggedIn ? (
                        <div className="text-center">
                          <div className="mb-4">
                            <div className="display-4 mb-3">🐦</div>
                            <h5>Connect Your Twitter Account</h5>
                            <p className="text-muted">
                              Connect with Twitter to start mining Snake AI tokens through your tweets.
                            </p>
                          </div>
                          
                          <button 
                            className="btn btn-primary btn-lg"
                            onClick={handleTwitterLogin}
                          >
                            <i className="fab fa-twitter me-2"></i>
                            Login with Twitter
                          </button>
                          
                          <div className="mt-4">
                            <small className="text-muted">
                              By connecting, you authorize us to verify your tweets and following status.
                            </small>
                          </div>
                        </div>
                      ) : (
                        <div className="text-center">
                          <div className="alert alert-success">
                            <h5>✅ Connected!</h5>
                            <p className="mb-0">Your Twitter account is connected. Start tweeting to mine tokens!</p>
                          </div>
                        </div>
                      )}
                    </div>
                  </div>
                </div>

                {/* Instructions */}
                <div className="col-lg-6">
                  <div className="card border-3 border-dashed h-100">
                    <div className="card-body">
                      <h3 className="card-title mb-4">📋 Mining Instructions</h3>
                      
                      <div className="d-flex flex-column gap-3">
                        <div className="d-flex align-items-start">
                          <div className="badge bg-primary rounded-circle me-3" style={{width: '30px', height: '30px', display: 'flex', alignItems: 'center', justifyContent: 'center'}}>
                            1
                          </div>
                          <div>
                            <h6 className="mb-1">Follow @playSnakeAI</h6>
                            <small className="text-muted">
                              You must follow our official Twitter account to qualify for rewards.
                            </small>
                          </div>
                        </div>
                        
                        <div className="d-flex align-items-start">
                          <div className="badge bg-primary rounded-circle me-3" style={{width: '30px', height: '30px', display: 'flex', alignItems: 'center', justifyContent: 'center'}}>
                            2
                          </div>
                          <div>
                            <h6 className="mb-1">Tweet with Required Tags</h6>
                            <small className="text-muted">
                              Include @playSnakeAI and #MineTheSnake in your tweet.
                            </small>
                          </div>
                        </div>
                        
                        <div className="d-flex align-items-start">
                          <div className="badge bg-primary rounded-circle me-3" style={{width: '30px', height: '30px', display: 'flex', alignItems: 'center', justifyContent: 'center'}}>
                            3
                          </div>
                          <div>
                            <h6 className="mb-1">Automatic Detection</h6>
                            <small className="text-muted">
                              Our system monitors tweets and validates eligibility in real-time.
                            </small>
                          </div>
                        </div>
                        
                        <div className="d-flex align-items-start">
                          <div className="badge bg-primary rounded-circle me-3" style={{width: '30px', height: '30px', display: 'flex', alignItems: 'center', justifyContent: 'center'}}>
                            4
                          </div>
                          <div>
                            <h6 className="mb-1">Receive Claim Link</h6>
                            <small className="text-muted">
                              We'll reply to your tweet with a unique claim link if you qualify.
                            </small>
                          </div>
                        </div>
                        
                        <div className="d-flex align-items-start">
                          <div className="badge bg-primary rounded-circle me-3" style={{width: '30px', height: '30px', display: 'flex', alignItems: 'center', justifyContent: 'center'}}>
                            5
                          </div>
                          <div>
                            <h6 className="mb-1">Claim Your Tokens</h6>
                            <small className="text-muted">
                              Click the link, connect your wallet, and claim your Snake AI tokens!
                            </small>
                          </div>
                        </div>
                      </div>
                      
                      <div className="mt-4">
                        <div className="card bg-light">
                          <div className="card-body">
                            <h6 className="text-success">💡 Pro Tips</h6>
                            <ul className="list-unstyled mb-0">
                              <li><small>• One reward per 24 hours per account</small></li>
                              <li><small>• Must maintain following status to qualify</small></li>
                              <li><small>• Tweet content can be creative - share about AI, gaming, crypto!</small></li>
                              <li><small>• Current phase: {getCurrentPhase()}</small></li>
                            </ul>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              {/* Sample Tweet */}
              <div className="row g-4 mt-2">
                <div className="col-12">
                  <div className="card border-3 border-dashed">
                    <div className="card-body">
                      <h4 className="card-title mb-3">📝 Sample Tweet</h4>
                      <div className="card bg-light">
                        <div className="card-body">
                          <div className="d-flex align-items-center mb-2">
                            <div className="rounded-circle bg-primary text-white d-flex align-items-center justify-content-center me-3" 
                                 style={{width: '40px', height: '40px'}}>
                              👤
                            </div>
                            <div>
                              <strong>YourUsername</strong>
                              <div><small className="text-muted">@yourusername</small></div>
                            </div>
                          </div>
                          <p className="mb-2">
                            Excited about the future of AI gaming! Mining some $SNAKE tokens with @playSnakeAI 🐍🚀 
                            The intersection of AI and blockchain gaming is fascinating! #MineTheSnake #AI #Gaming #Crypto
                          </p>
                          <div className="text-muted">
                            <small>2:30 PM · Today</small>
                          </div>
                        </div>
                      </div>
                      <div className="mt-3">
                        <button className="btn btn-outline-primary">
                          <i className="fab fa-twitter me-2"></i>
                          Tweet This Example
                        </button>
                      </div>
                    </div>
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

// Helper function to get current mining phase
function getCurrentPhase(): string {
  // This could be fetched from an API or config
  return "Phase 2";
}

export default TwitterMiningPage;
