import React, { useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { useAuth } from '../contexts/AuthContext';
import Cookies from 'js-cookie';

interface ProtectedRouteProps {
  children: React.ReactNode;
}

const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ children }) => {
  const { isAuthenticated } = useAuth();
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    const sessionToken = Cookies.get('SID');
    
    // If no session token and not authenticated, store claim URL and redirect to get-started
    if (!sessionToken && !isAuthenticated) {
      // Store the original URL if it's a claim URL
      if (location.pathname.startsWith('/claim')) {
        localStorage.setItem('redirectAfterLogin', location.pathname);
      }
      navigate('/get-started', { replace: true });
    }
  }, [isAuthenticated, navigate, location.pathname]);

  // If no session token, don't render the protected content
  const sessionToken = Cookies.get('SID');
  if (!sessionToken && !isAuthenticated) {
    return null;
  }

  return <>{children}</>;
};

export default ProtectedRoute;
