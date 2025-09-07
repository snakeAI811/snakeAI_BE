import React, { createContext, useContext, ReactNode, useState, useEffect } from 'react';

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

// Enhanced mobile detection
const isMobile = () => {
  if (typeof window === 'undefined') return false;
  return /Android|iPhone|iPad|iPod|Mobile|BlackBerry|Opera Mini|IEMobile|WPDesktop/i.test(navigator.userAgent);
};

// Enhanced provider detection with better mobile support
const getPhantomProvider = (): any | null => {
  if (typeof window === 'undefined') return null;
  
  const w = window as any;
  
  // Check for Phantom specifically first
  if (w?.phantom?.solana && w.phantom.solana.isPhantom) {
    return w.phantom.solana;
  }
  
  // On mobile, sometimes Phantom injects as window.solana
  if (w?.solana && w.solana.isPhantom) {
    return w.solana;
  }
  
  return null;
};

// Check if we're inside Phantom mobile app
const isInPhantomApp = (): boolean => {
  if (typeof window === 'undefined') return false;
  const userAgent = navigator.userAgent;
  return userAgent.includes('Phantom') || userAgent.includes('phantom');
};

export const WalletContextProvider: React.FC<WalletContextProviderProps> = ({ children }) => {
  const [connected, setConnected] = useState(false);
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [connecting, setConnecting] = useState(false);

  // Check if user previously attempted to connect (for mobile deep link returns)
  useEffect(() => {
    if (typeof window === 'undefined') return;
    
    const hadConnectAttempt = sessionStorage.getItem('phantomConnectAttempt');
    if (hadConnectAttempt) {
      // User returned from Phantom app, check if provider is now available
      const provider = getPhantomProvider();
      if (provider) {
        console.log('User returned from Phantom app, provider now available');
        // Don't auto-connect, just clear the flag
        sessionStorage.removeItem('phantomConnectAttempt');
      }
    }
  }, []);

  const connect = async () => {
    setConnecting(true);
    try {
      if (typeof window === 'undefined') throw new Error('Window unavailable');
      
      const provider = getPhantomProvider();

      if (!provider || !provider.isPhantom) {
        if (isMobile() && !isInPhantomApp()) {
          // Create a more reliable deep link
          const currentUrl = window.location.href;
          const deeplink = `https://phantom.app/ul/browse/${encodeURIComponent(currentUrl)}?ref=wallet-connect`;
          
          // Store connection attempt for when user returns
          sessionStorage.setItem('phantomConnectAttempt', 'true');
          
          // Use window.open with fallback to location.href
          const opened = window.open(deeplink, '_blank');
          if (!opened) {
            window.location.href = deeplink;
          }
          return;
        } else {
          throw new Error('Phantom wallet not found! Please install it from phantom.app');
        }
      }

      // Connect with proper error handling
      const response = await provider.connect();
      const walletPublicKey = response?.publicKey?.toString?.();

      if (!walletPublicKey) {
        throw new Error('Failed to get wallet public key');
      }

      // Optional: Try to sign a message for authentication
      let signatureOk = false;
      if (provider.signMessage) {
        try {
          const message = new TextEncoder().encode(
            `Authenticate with our app\nWallet: ${walletPublicKey}\nTime: ${Date.now()}`
          );
          const signedMessage = await provider.signMessage(message, 'utf8');
          signatureOk = Boolean(signedMessage?.signature);
        } catch (e) {
          console.log('Signature rejected, continuing without auth:', e);
        }
      }

      setPublicKey(walletPublicKey);
      setConnected(true);
      
      // Clear any stored connection attempt
      sessionStorage.removeItem('phantomConnectAttempt');
      
      console.log('Wallet connected successfully', signatureOk ? '(authenticated)' : '(no signature)');

    } catch (err) {
      const error = err as Error & { code?: number };
      console.error('Failed to connect wallet:', error);

      // Handle specific error codes
      if (error.code === 4001 || error.message?.includes('User rejected')) {
        console.log('Connection cancelled by user');
      } else {
        console.error('Connection error:', error.message);
        // Don't show alert on mobile to avoid poor UX
        if (!isMobile()) {
          alert(`Failed to connect wallet: ${error.message}`);
        }
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
      sessionStorage.removeItem('phantomConnectAttempt');
    }
  };

  const signMessage = async (message: Uint8Array): Promise<Uint8Array> => {
    if (!connected || typeof window === 'undefined') {
      throw new Error('Wallet not connected');
    }
    
    const provider = getPhantomProvider();
    if (!provider?.signMessage) {
      throw new Error('signMessage not supported by wallet');
    }

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