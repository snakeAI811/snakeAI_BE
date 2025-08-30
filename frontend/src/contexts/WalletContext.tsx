import React, { createContext, useContext, ReactNode, useState } from 'react';

interface WalletContextState {
  connected: boolean;
  publicKey: string | null;
  connecting: boolean;
  connect: () => Promise<void>;
  disconnect: () => void;
  signMessage?: (message: Uint8Array) => Promise<Uint8Array>;
}

interface WalletContextProviderProps {
  children: ReactNode;
}

const WalletContext = createContext<WalletContextState>({
  connected: false,
  publicKey: null,
  connecting: false,
  connect: async () => {},
  disconnect: () => {},
  signMessage: undefined,
});

export const useWalletContext = () => useContext(WalletContext);

export const WalletContextProvider: React.FC<WalletContextProviderProps> = ({ children }) => {
  const [connected, setConnected] = useState(false);
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [connecting, setConnecting] = useState(false);

  const connect = async () => {
    setConnecting(true);
    try {
      // Check if Phantom wallet is available
      if (typeof window !== 'undefined' && (window as any).solana && (window as any).solana.isPhantom) {
        // First, connect to get the public key
        const response = await (window as any).solana.connect();
        const walletPublicKey = response.publicKey.toString();

        // Create a message for the user to sign (proves ownership)
        const message = new TextEncoder().encode(
          `Sign this message to authenticate with our app.\n\nWallet: ${walletPublicKey}\nTimestamp: ${Date.now()}`
        );

        // Request signature to verify wallet ownership
        const signedMessage = await (window as any).solana.signMessage(message, 'utf8');
        
        if (signedMessage && signedMessage.signature) {
          // Only set connected state after successful signature
          setPublicKey(walletPublicKey);
          setConnected(true);
          console.log('Wallet connected and authenticated successfully');
        } else {
          throw new Error('Failed to get signature from wallet');
        }
      } else {
        alert('Phantom wallet not found! Please install it.');
      }
    } catch (err) {
      const error = err as Error & { code?: number };
      console.error('Failed to connect wallet:', error);

      // Handle specific error cases
      if ((error as any)?.code === 4001) {
        alert('Connection cancelled by user.');
      } else if (error.message && error.message.includes('User rejected')) {
        alert('Signature request was rejected. Please approve to continue.');
      } else {
        alert('Failed to connect wallet. Please try again.');
      }

      // Reset state on error
      setConnected(false);
      setPublicKey(null);
    } finally {
      setConnecting(false);
    }
  };

  const disconnect = () => {
    try {
      // Disconnect from Phantom if available
      if (typeof window !== 'undefined' && (window as any).solana && (window as any).solana.isConnected) {
        (window as any).solana.disconnect();
      }
    } catch (error) {
      console.error('Error disconnecting wallet:', error);
    } finally {
      setConnected(false);
      setPublicKey(null);
    }
  };

  const signMessage = async (message: Uint8Array): Promise<Uint8Array> => {
    if (!connected || typeof window === 'undefined' || !(window as any).solana) {
      throw new Error('Wallet not connected');
    }

    try {
      const signedMessage = await (window as any).solana.signMessage(message, 'utf8');
      return signedMessage.signature;
    } catch (error) {
      console.error('Error signing message:', error);
      throw error;
    }
  };

  const value: WalletContextState = {
    connected,
    publicKey,
    connecting,
    connect,
    disconnect,
    signMessage,
  };

  return (
    <WalletContext.Provider value={value}>
      {children}
    </WalletContext.Provider>
  );
};