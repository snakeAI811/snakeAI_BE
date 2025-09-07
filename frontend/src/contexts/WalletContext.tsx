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

// Helpers shared with usePhantom
const isMobile = () => /Android|iPhone|iPad|iPod|Mobile/i.test(navigator.userAgent);
const getPhantomProvider = (): any | null => {
  const w = window as any;
  if (w?.phantom?.solana) return w.phantom.solana;
  if (w?.solana) return w.solana;
  return null;
};

export const WalletContextProvider: React.FC<WalletContextProviderProps> = ({ children }) => {
  const [connected, setConnected] = useState(false);
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [connecting, setConnecting] = useState(false);

  const connect = async () => {
    setConnecting(true);
    try {
      if (typeof window === 'undefined') throw new Error('Window unavailable');
      const provider = getPhantomProvider();

      if (!provider || !provider.isPhantom) {
        // If mobile and outside Phantom, deep link to open current page in Phantom app
        if (isMobile()) {
          const url = window.location.href;
          const deeplink = `https://phantom.app/ul/browse/${encodeURIComponent(url)}`;
          window.location.href = deeplink;
        } else {
          alert('Phantom wallet not found! Please install it.');
        }
        return;
      }

      // Try silent connect first
      await provider.connect?.({ onlyIfTrusted: true }).catch(() => {});

      // Connect (user gesture)
      const response = await provider.connect();
      const walletPublicKey = response?.publicKey?.toString?.();

      // Try to sign a message if available (for authentication)
      let signatureOk = false;
      if (provider.signMessage && walletPublicKey) {
        try {
          const message = new TextEncoder().encode(
            `Sign this message to authenticate with our app.\n\nWallet: ${walletPublicKey}\nTimestamp: ${Date.now()}`
          );
          const signedMessage = await provider.signMessage(message, 'utf8');
          signatureOk = Boolean(signedMessage?.signature);
        } catch (e) {
          // User may reject; continue with connected state if we don't require signature strictly
          signatureOk = false;
        }
      }

      // Set connected regardless of signature success to improve UX on mobile
      if (walletPublicKey) {
        setPublicKey(walletPublicKey);
        setConnected(true);
        console.log('Wallet connected', signatureOk ? 'and authenticated' : '(no signature)');
      }
    } catch (err) {
      const error = err as Error & { code?: number };
      console.error('Failed to connect wallet:', error);

      if ((error as any)?.code === 4001) {
        alert('Connection cancelled by user.');
      } else if (error.message && error.message.includes('User rejected')) {
        alert('Signature request was rejected. You can try again.');
      } else {
        alert('Failed to connect wallet. Please try again.');
      }

      setConnected(false);
      setPublicKey(null);
    } finally {
      setConnecting(false);
    }
  };

  const disconnect = () => {
    try {
      const provider = getPhantomProvider();
      provider?.disconnect?.();
    } catch (error) {
      console.error('Error disconnecting wallet:', error);
    } finally {
      setConnected(false);
      setPublicKey(null);
    }
  };

  const signMessage = async (message: Uint8Array): Promise<Uint8Array> => {
    if (!connected || typeof window === 'undefined') {
      throw new Error('Wallet not connected');
    }
    const provider = getPhantomProvider();
    if (!provider?.signMessage) throw new Error('signMessage not supported by wallet');

    try {
      const signedMessage = await provider.signMessage(message, 'utf8');
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