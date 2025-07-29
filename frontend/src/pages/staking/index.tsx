import React, { useMemo } from 'react';
import ResponsiveMenu from "../../components/ResponsiveMenu";
import StakingDashboard from '../dashboard/StakingDashboard';
import WalletGuard from "../../components/WalletGuard";
import { useWallet } from '@solana/wallet-adapter-react';
import { Connection, PublicKey } from '@solana/web3.js';
import { Program, AnchorProvider } from '@project-serum/anchor';
import snakeContractIdl from '../../services/snake_contract.json';

function StakingPage() {
  const { wallet, publicKey, signTransaction, signAllTransactions } = useWallet();
  
  const connection = useMemo(() => 
    new Connection(process.env.REACT_APP_RPC_URL || 'https://api.devnet.solana.com'),
    []
  );
  
  const programId = useMemo(() => 
    new PublicKey(process.env.REACT_APP_PROGRAM_ID || '7atsSKAcDXELnLGjvNX27ke8wdC8XUpsG1bJciCj1pQZ'),
    []
  );

  const { program } = useMemo(() => {
    if (!wallet || !publicKey || !signTransaction || !signAllTransactions) {
      return { provider: null, program: null };
    }

    const walletAdapter = {
      publicKey,
      signTransaction,
      signAllTransactions,
    };

    const provider = new AnchorProvider(connection, walletAdapter as any, { 
      preflightCommitment: "processed" 
    });

    const program = new Program(
      snakeContractIdl as any,
      programId,
      provider
    );

    return { program };
  }, [wallet, publicKey, signTransaction, signAllTransactions, connection, programId]);

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
                {program ? (
                  <StakingDashboard program={program} connection={connection} />
                ) : (
                  <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '50vh' }}>
                    <div className="text-center">
                      <div className="spinner-border text-primary mb-3" role="status">
                        <span className="visually-hidden">Loading...</span>
                      </div>
                      <p>Connecting to Solana network...</p>
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
