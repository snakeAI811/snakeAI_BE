import React, { createContext, useContext, ReactNode, useState, useEffect, useCallback } from 'react';
import Cookies from 'js-cookie';
import { userApi, setAuthErrorHandler } from '../pages/patron/services/apiService';

interface User {
  id: string;
  twitter_username?: string;
  wallet_address?: string;
}

interface AuthContextState {
  isAuthenticated: boolean;
  user: User | null;
  sessionId: string | null;
  login: (sessionId: string) => void;
  logout: () => void;
  setUser: (user: User | null) => void;
}

interface AuthContextProviderProps {
  children: ReactNode;
}

const AuthContext = createContext<AuthContextState>({
  isAuthenticated: false,
  user: null,
  sessionId: null,
  login: () => { },
  logout: () => { },
  setUser: () => { },
});

export const useAuth = () => useContext(AuthContext);

export const AuthContextProvider: React.FC<AuthContextProviderProps> = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [user, setUser] = useState<User | null>(null);
  const [sessionId, setSessionId] = useState<string | null>(null);

  const logout = useCallback(() => {
    Cookies.remove('SID');
    setSessionId(null);
    setIsAuthenticated(false);
    setUser(null);
  }, []);

  const fetchUserData = useCallback(async () => {
    let isMounted = true;
    try {
      const sessionToken = Cookies.get('SID');
      if (!sessionToken) return;

      const result = await userApi.getMe();
      if (!isMounted) return;
      if (result.success) {
        setUser(result.data);
        setIsAuthenticated(true);
      } else {
        setIsAuthenticated(false);
        setSessionId(null);
        setUser(null);
        Cookies.remove('SID');
      }
    } catch (error) {
      if (!isMounted) return;
      setIsAuthenticated(false);
      setSessionId(null);
      setUser(null);
      Cookies.remove('SID');
    }
    return () => { isMounted = false; };
  }, []);

  const login = useCallback((newSessionId: string) => {
    Cookies.set('SID', newSessionId, { expires: 7 });
    setSessionId(newSessionId);
    setIsAuthenticated(true);
    fetchUserData();
    
    // Check if there's a stored redirect URL for claim pages
    const redirectUrl = localStorage.getItem('redirectAfterLogin');
    if (redirectUrl && redirectUrl.startsWith('/claim')) {
      localStorage.removeItem('redirectAfterLogin');
      // Redirect to tweet mining page as requested
      window.location.href = '/tweet-mining';
    }
  }, [fetchUserData]);

  useEffect(() => {
    setAuthErrorHandler(() => {
      if (window.location.pathname !== '/' && !window.location.pathname.includes('get-started')) {
        window.location.href = '/get-started';
      }
      logout();
    });

    const existingSessionId = Cookies.get('SID');
    if (existingSessionId && !isAuthenticated) {
      setSessionId(existingSessionId);
      fetchUserData();
    }
  }, [logout, fetchUserData, isAuthenticated]);

  useEffect(() => {
    const existingSessionId = Cookies.get('SID');
    if (!existingSessionId && isAuthenticated) {
      setIsAuthenticated(false);
      setUser(null);
      setSessionId(null);
    }
  }, [isAuthenticated]);

  const value: AuthContextState = {
    isAuthenticated,
    user,
    sessionId,
    login,
    logout,
    setUser,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
};
