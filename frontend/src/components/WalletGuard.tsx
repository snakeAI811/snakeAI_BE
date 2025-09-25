import React, { useEffect, useState } from 'react';
import { useWalletContext } from '../contexts/WalletContext';

interface WalletGuardProps {
  children: React.ReactNode;
  showMessage?: boolean;
}

const WalletGuard: React.FC<WalletGuardProps> = ({ children, showMessage = true }) => {
  const { connected, connecting, connect } = useWalletContext();
  const [isPhantomAvailable, setIsPhantomAvailable] = useState(false);

  useEffect(() => {
    // Check if Phantom is available
    const checkPhantom = () => {
      const w = window as any;
      const hasPhantom = (w?.phantom?.solana && w.phantom.solana.isPhantom) || 
                        (w?.solana && w.solana.isPhantom);
      setIsPhantomAvailable(hasPhantom);
    };

    checkPhantom();
    
    // Recheck after a delay (for mobile loading)
    const timer = setTimeout(checkPhantom, 1000);
    return () => clearTimeout(timer);
  }, []);

  if (connected) {
    return <>{children}</>;
  }

  const isMobile = /Android|iPhone|iPad|iPod|Mobile/i.test(navigator.userAgent);
  const isInPhantomApp = navigator.userAgent.includes('Phantom') || navigator.userAgent.includes('phantom');
  const currentUrl = typeof window !== 'undefined' ? window.location.href : '';
  const phantomDeepLink = `https://phantom.app/ul/browse/${encodeURIComponent(currentUrl)}`;

  return (
    <div className="d-flex flex-column align-items-center justify-content-center" style={{ minHeight: '60vh' }}>
      {showMessage && (
        <div className="text-center mb-4">
          <h4 className="text-muted mb-3">🔗 Phantom Wallet Connection Required</h4>
          <p className="text-muted">Please connect your wallet to access this feature.</p>
        </div>
      )}
      
      <div className="text-center">
        {/* Show different UI based on mobile vs desktop and Phantom availability */}
        {isMobile && !isPhantomAvailable && !isInPhantomApp && (
          <div className="alert alert-info mb-3" role="alert">
            <div className="mb-2">
              <strong>Mobile users:</strong> Open this page in the Phantom app to connect your wallet.
            </div>
            <a 
              href={phantomDeepLink} 
              className="btn btn-sm btn-primary"
              onClick={() => {
                // Store the attempt for when they return
                sessionStorage.setItem('phantomConnectAttempt', 'true');
              }}
            >
              Open in Phantom App
            </a>
          </div>
        )}

        {connecting ? (
          <div>
            <div className="spinner-border text-primary mb-2" role="status">
              <span className="visually-hidden">Connecting...</span>
            </div>
            <p className="text-muted">Connecting to wallet...</p>
          </div>
        ) : (
          <button 
            onClick={connect} 
            className="primary-btn btn-lg"
            disabled={isMobile && !isPhantomAvailable && !isInPhantomApp}
            style={{
              background: '#A9E000',
              color: 'black',
              border: 'none',
              padding: '12px 24px',
              borderRadius: '8px',
              opacity: (isMobile && !isPhantomAvailable && !isInPhantomApp) ? 0.6 : 1
            }}
          >
            {isPhantomAvailable || isInPhantomApp ? 'Connect Wallet' : 'Install Phantom Wallet'}
          </button>
        )}

        {/* Wallets supported section */}
        <div className="mt-4">
          <small className="text-muted d-block mb-2">Wallets supported</small>
          <a
            href="https://phantom.app/download"
            target="_blank"
            rel="noopener noreferrer"
            className="d-inline-flex align-items-center text-decoration-none"
            aria-label="Download Phantom Wallet"
          >
            <span style={{ fontSize: '1.25rem', marginRight: '8px' }}>
              <img src="/phantom.png" alt="Phantom Logo" style={{ height: '24px', width: '24px' }} />
            </span>
            <span>Download Phantom</span>
          </a>
        </div>
      </div>
    </div>
  );
};

export default WalletGuard;