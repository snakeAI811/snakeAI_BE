import React, { useMemo } from 'react';
import ResponsiveMenu from "../../components/ResponsiveMenu";
import SimpleStakingDashboard from '../dashboard/SimpleStakingDashboard';
import WalletGuard from "../../components/WalletGuard";
import { useWalletContext } from '../../contexts/WalletContext';
import { Connection } from '@solana/web3.js';

function StakingPage() {
  const { connected, publicKey } = useWalletContext();
  
  const connection = useMemo(() => 
    new Connection(process.env.REACT_APP_RPC_URL || 'https://api.devnet.solana.com'),
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
