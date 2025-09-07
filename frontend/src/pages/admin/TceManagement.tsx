import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { adminApi } from '../patron/services/apiService';

interface TceStatus {
  tce_enabled: boolean;
  tce_start_date: string | null;
  total_users_affected: number;
}

function TceManagement() {
  const [tceStatus, setTceStatus] = useState<TceStatus | null>(null);
  const [loading, setLoading] = useState(false);
  const [processing, setProcessing] = useState(false);

  useEffect(() => {
    fetchTceStatus();
  }, []);

  const fetchTceStatus = async () => {
    setLoading(true);
    try {
      const response = await adminApi.getTceStatus();
      if (response.success && response.data) {
        setTceStatus(response.data as TceStatus);
      }
    } catch (error) {
      console.error('Failed to fetch TCE status:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleEnableTce = async () => {
    setProcessing(true);
    try {
      const result = await adminApi.enableTce();

      if (result.success) {
        console.log('TCE enable transaction:', result.data);
        // Refresh the status
        fetchTceStatus();
        alert('TCE has been enabled successfully!');
      } else {
        throw new Error(result.error || 'Failed to enable TCE');
      }
    } catch (error) {
      console.error('Error enabling TCE:', error);
      alert('Error enabling TCE: ' + error);
    } finally {
      setProcessing(false);
    }
  };

  const handleDisableTce = async () => {
    if (!window.confirm('Are you sure you want to disable TCE? This will affect all users.')) {
      return;
    }

    setProcessing(true);
    try {
      const result = await adminApi.disableTce();

      if (result.success) {
        console.log('TCE disable transaction:', result.data);
        // Refresh the status
        fetchTceStatus();
        alert('TCE has been disabled successfully!');
      } else {
        throw new Error(result.error || 'Failed to disable TCE');
      }
    } catch (error) {
      console.error('Error disabling TCE:', error);
      alert('Error disabling TCE: ' + error);
    } finally {
      setProcessing(false);
    }
  };

  const getStatusBadge = (enabled: boolean) => {
    return enabled ? 'bg-success' : 'bg-danger';
  };

  const getStatusText = (enabled: boolean) => {
    return enabled ? 'Enabled' : 'Disabled';
  };

  return (
    <div className="container-fluid">
      {/* Breadcrumb */}
      <nav aria-label="breadcrumb" className="mb-3">
        <ol className="breadcrumb">
          <li className="breadcrumb-item"><Link to="/home">Home</Link></li>
          <li className="breadcrumb-item"><Link to="/admin">Admin Dashboard</Link></li>
          <li className="breadcrumb-item active" aria-current="page">TCE Management</li>
        </ol>
      </nav>

      <div className="row">
        <div className="col-12">
          <div className="card">
            <div className="card-header d-flex justify-content-between align-items-center">
              <h4 className="mb-0">🔧 TCE Flag Management</h4>
              <button 
                className="btn btn-primary btn-sm"
                onClick={fetchTceStatus}
                disabled={loading}
              >
                {loading ? (
                  <>
                    <span className="spinner-border spinner-border-sm me-2" />
                    Refreshing...
                  </>
                ) : (
                  'Refresh'
                )}
              </button>
            </div>

            <div className="card-body">
              {loading ? (
                <div className="text-center py-4">
                  <span className="spinner-border" />
                  <p className="mt-2">Loading TCE status...</p>
                </div>
              ) : tceStatus ? (
                <div className="row">
                  <div className="col-md-6">
                    <div className="card border-0 bg-light">
                      <div className="card-body">
                        <h5 className="card-title">Current TCE Status</h5>
                        <div className="d-flex align-items-center mb-3">
                          <span className={`badge ${getStatusBadge(tceStatus.tce_enabled)} fs-6 me-3`}>
                            {getStatusText(tceStatus.tce_enabled)}
                          </span>
                          <span className="text-muted">
                            {tceStatus.tce_enabled ? 'TCE is currently active' : 'TCE is currently inactive'}
                          </span>
                        </div>

                        {tceStatus.tce_start_date && (
                          <div className="mb-3">
                            <strong>Start Date:</strong>
                            <br />
                            <span className="text-muted">
                              {new Date(tceStatus.tce_start_date).toLocaleString()}
                            </span>
                          </div>
                        )}

                        <div className="mb-3">
                          <strong>Users Affected:</strong>
                          <br />
                          <span className="badge bg-info fs-6">
                            {tceStatus.total_users_affected.toLocaleString()}
                          </span>
                        </div>
                      </div>
                    </div>
                  </div>

                  <div className="col-md-6">
                    <div className="card border-0 bg-light">
                      <div className="card-body">
                        <h5 className="card-title">Actions</h5>
                        <div className="d-grid gap-2">
                          {!tceStatus.tce_enabled ? (
                            <button
                              className="btn btn-success"
                              onClick={handleEnableTce}
                              disabled={processing}
                            >
                              {processing ? (
                                <>
                                  <span className="spinner-border spinner-border-sm me-2" />
                                  Enabling TCE...
                                </>
                              ) : (
                                '✓ Enable TCE'
                              )}
                            </button>
                          ) : (
                            <button
                              className="btn btn-danger"
                              onClick={handleDisableTce}
                              disabled={processing}
                            >
                              {processing ? (
                                <>
                                  <span className="spinner-border spinner-border-sm me-2" />
                                  Disabling TCE...
                                </>
                              ) : (
                                '✗ Disable TCE'
                              )}
                            </button>
                          )}
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              ) : (
                <div className="alert alert-warning text-center">
                  <h5>Unable to Load TCE Status</h5>
                  <p className="mb-0">Please check your connection and try refreshing.</p>
                </div>
              )}
            </div>

            <div className="card-footer">
              <div className="row">
                <div className="col-md-6">
                  <h6>About TCE (Token Claim Engine):</h6>
                  <ul className="small mb-0">
                    <li>Controls global token claiming functionality</li>
                    <li>When enabled, users can claim accumulated rewards</li>
                    <li>When disabled, claiming is suspended system-wide</li>
                  </ul>
                </div>
                <div className="col-md-6">
                  <h6>⚠️ Important Notes:</h6>
                  <ul className="small mb-0">
                    <li>Changes affect all users immediately</li>
                    <li>Disabling TCE prevents all reward claims</li>
                    <li>Use with caution during maintenance</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default TceManagement;