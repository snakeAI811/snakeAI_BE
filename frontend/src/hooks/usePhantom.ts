// hooks/usePhantom.ts
// Improved Phantom provider detection with mobile support and events
import { useEffect, useMemo, useState } from "react";

// Detect mobile (basic)
const isMobile = () => /Android|iPhone|iPad|iPod|Mobile/i.test(navigator.userAgent);

// Get Phantom provider from window in a robust way
function getPhantomProvider(): any | null {
  const w = window as any;
  if (w?.phantom?.solana) return w.phantom.solana; // Phantom in-app browser/mobile
  if (w?.solana) return w.solana; // Desktop extension or injected provider
  return null;
}

export const usePhantom = () => {
  const [walletAvailable, setWalletAvailable] = useState(false);
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [connected, setConnected] = useState(false);

  const provider = useMemo(() => (typeof window !== 'undefined' ? getPhantomProvider() : null), []);

  useEffect(() => {
    if (provider?.isPhantom) {
      setWalletAvailable(true);

      // Attach connect/disconnect events
      const onConnect = (pk: any) => {
        try {
          const key = typeof pk?.toString === 'function' ? pk.toString() : pk?.publicKey?.toString?.();
          if (key) setPublicKey(key);
        } catch {}
        setConnected(true);
      };
      const onDisconnect = () => {
        setConnected(false);
        setPublicKey(null);
      };

      provider.on?.('connect', onConnect);
      provider.on?.('disconnect', onDisconnect);

      // Attempt trusted connect silently (no popup) where supported
      provider.connect?.({ onlyIfTrusted: true }).catch(() => {});

      return () => {
        provider.removeListener?.('connect', onConnect);
        provider.removeListener?.('disconnect', onDisconnect);
      };
    } else {
      setWalletAvailable(false);
    }
  }, [provider]);

  const connect = async () => {
    try {
      const p = provider ?? getPhantomProvider();
      if (!p || !p.isPhantom) {
        // On mobile outside the Phantom dapp browser, deep link to open current site in Phantom
        if (isMobile()) {
          const url = typeof window !== 'undefined' ? window.location.href : '';
          const deeplink = `https://phantom.app/ul/browse/${encodeURIComponent(url)}`;
          window.location.href = deeplink;
        }
        throw new Error("Phantom not found");
      }

      // Ensure user gesture initiated
      const res = await p.connect();
      const key = res?.publicKey?.toString?.();
      if (key) setPublicKey(key);
      setConnected(true);
      return key ?? null;
    } catch (error) {
      console.error("Phantom connection error:", error);
      return null;
    }
  };

  const disconnect = async () => {
    try {
      const p = provider ?? getPhantomProvider();
      await p?.disconnect?.();
    } catch (e) {
      console.warn('Phantom disconnect warning:', e);
    } finally {
      setConnected(false);
      setPublicKey(null);
    }
  };

  return {
    walletAvailable,
    connected,
    publicKey,
    connect,
    disconnect,
    provider,
  };
};