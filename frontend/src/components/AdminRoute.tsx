import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../contexts/AuthContext';
import { userApi } from '../pages/patron/services/apiService';
import Cookies from 'js-cookie';

interface AdminRouteProps {
  children: React.ReactNode;
}

const AdminRoute: React.FC<AdminRouteProps> = ({ children }) => {
  const { isAuthenticated } = useAuth();
  const navigate = useNavigate();
  const [isAdmin, setIsAdmin] = useState<boolean | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const checkAdminStatus = async () => {
      const sessionToken = Cookies.get('SID');
      
      // First check if user is authenticated
      if (!sessionToken && !isAuthenticated) {
        navigate('/get-started', { replace: true });
        return;
      }

      try {
        // Check user role/permissions
        const response = await userApi.getMe();
        if (response.success && response.data) {
          // Check if user has admin role or permissions
          // This assumes the API returns role information
          const userRole = response.data.role || response.data.user_role;
          const isUserAdmin = userRole === 'admin' || response.data.is_admin === true;
          
          setIsAdmin(isUserAdmin);
          
          if (!isUserAdmin) {
            // Redirect non-admin users to home page
            navigate('/home', { replace: true });
          }
        } else {
          setIsAdmin(false);
          navigate('/home', { replace: true });
        }
      } catch (error) {
        console.error('Error checking admin status:', error);
        setIsAdmin(false);
        navigate('/home', { replace: true });
      } finally {
        setLoading(false);
      }
    };

    checkAdminStatus();
  }, [isAuthenticated, navigate]);

  // Show loading spinner while checking admin status
  if (loading) {
    return (
      <div className="container-fluid">
        <div className="row">
          <div className="col-12">
            <div className="card">
              <div className="card-body text-center py-5">
                <div className="spinner-border text-primary" role="status">
                  <span className="visually-hidden">Loading...</span>
                </div>
                <p className="mt-3 text-muted">Verifying admin access...</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  // Show access denied if not admin
  if (isAdmin === false) {
    return (
      <div className="container-fluid">
        <div className="row">
          <div className="col-12">
            <div className="card">
              <div className="card-body text-center py-5">
                <div className="mb-4">
                  <span style={{ fontSize: '4rem' }}>🚫</span>
                </div>
                <h3 className="text-danger">Access Denied</h3>
                <p className="text-muted">You don't have permission to access this admin area.</p>
                <button 
                  className="btn btn-primary"
                  onClick={() => navigate('/home')}
                >
                  Return to Home
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  // Render admin content if user is admin
  return <>{children}</>;
};

export default AdminRoute;