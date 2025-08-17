import React, { useMemo } from 'react';
import ResponsiveMenu from "../../components/ResponsiveMenu";
import SimpleStakingDashboard from './SimpleStakingDashboard';
import WalletGuard from "../../components/WalletGuard";
import { useWalletContext } from '../../contexts/WalletContext';
import { Connection } from '@solana/web3.js';
import { SOLANA_RPC_URL } from '../../config/program';
import { useAuth } from '../../contexts/AuthContext';

function StakingPage() {
  const { connected, publicKey } = useWalletContext();
  const { user, logout } = useAuth();

  const connection = useMemo(() =>
    new Connection(SOLANA_RPC_URL || 'https://api.devnet.solana.com'),
    []
  );

  return (
    <div className="w-100 p-3" style={{ height: "100vh" }}>
      <div className="d-flex gap-4" style={{ height: "calc(100vh-60px)", paddingTop: '55px' }}>
        <ResponsiveMenu />

        <div className="custom-content">
          <div className="w-100">
            <div className="d-flex justify-content-between align-items-center">
              <div className="fs-1" style={{ lineHeight: 'normal' }}>🏦 Staking Dashboard</div>
              <div className="text-end d-flex align-items-center gap-2">
                <div className="fs-6 text-muted">
                  Connected: @{user?.twitter_username || 'Not authenticated'}
                </div>
                <button
                  onClick={async () => {
                    await logout();
                  }}
                  className="fs-6 fw-bold second-btn py-1 px-2 text-decoration-none text-center">
                  LOGOUT
                </button>
              </div>
            </div>
          </div>

          <div className="custom-border-y custom-content-height d-flex flex-column px-3">
            <WalletGuard>
              <div className="item-stretch w-100" style={{ minHeight: '86vh' }}>
                {connected && publicKey ? (
                  <SimpleStakingDashboard connection={connection} />
                ) : (
                  <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '50vh' }}>
                    <div className="text-center">
                      <div className="spinner-border text-primary mb-3" role="status">
                        <span className="visually-hidden">Loading...</span>
                      </div>
                      <p>Please connect your wallet to view staking dashboard...</p>
                    </div>
                  </div>
                )}
              </div>
            </WalletGuard>
          </div>
        </div>
      </div>
    </div>
  );
}

export default StakingPage;
