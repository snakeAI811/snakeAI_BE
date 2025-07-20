import React, { createContext, useContext, ReactNode, useState, useEffect } from 'react';
import Cookies from 'js-cookie';
import { userApi } from '../pages/patron/services/apiService';

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
  login: () => {},
  logout: () => {},
  setUser: () => {},
});

export const useAuth = () => useContext(AuthContext);

export const AuthContextProvider: React.FC<AuthContextProviderProps> = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [user, setUser] = useState<User | null>(null);
  const [sessionId, setSessionId] = useState<string | null>(null);

  useEffect(() => {
    // Check for existing session on mount
    const existingSessionId = Cookies.get('SID');
    if (existingSessionId) {
      setSessionId(existingSessionId);
      setIsAuthenticated(true);
      // Fetch user data if needed
      fetchUserData();
    }
  }, []);

  const fetchUserData = async () => {
    try {
      const sessionToken = Cookies.get('SID');
      if (!sessionToken) return;

      const result = await userApi.getMe();
      if (result.success) {
        setUser(result.data);
      }
    } catch (error) {
      console.error('Failed to fetch user data:', error);
    }
  };

  const login = (newSessionId: string) => {
    Cookies.set('SID', newSessionId, { expires: 7 });
    setSessionId(newSessionId);
    setIsAuthenticated(true);
    fetchUserData();
  };

  const logout = () => {
    Cookies.remove('SID');
    setSessionId(null);
    setIsAuthenticated(false);
    setUser(null);
  };

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
