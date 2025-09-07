import React from 'react';
import { useWalletContext } from '../contexts/WalletContext';

interface WalletGuardProps {
  children: React.ReactNode;
  showMessage?: boolean;
}

const WalletGuard: React.FC<WalletGuardProps> = ({ children, showMessage = true }) => {
  const { connected, connecting, connect } = useWalletContext();

  if (connected) {
    return <>{children}</>;
  }

  const isMobile = /Android|iPhone|iPad|iPod|Mobile/i.test(navigator.userAgent);
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
        {isMobile && !((window as any)?.phantom?.solana || (window as any)?.solana) && (
          <div className="alert alert-info" role="alert">
            Open this page in Phantom for mobile to connect your wallet.
            <div className="mt-2">
              <a href={phantomDeepLink} className="btn btn-sm btn-primary">Open in Phantom</a>
            </div>
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
            style={{
              background: '#A9E000',
              color: 'black',
              border: 'none',
              padding: '12px 24px',
              borderRadius: '8px'
            }}
          >
            Connect Wallet
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
            <span>Phantom download</span>
          </a>
        </div>
      </div>
    </div>
  );
};

export default WalletGuard;
