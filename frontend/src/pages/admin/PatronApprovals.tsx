import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { patronApi } from '../patron/services/apiService';

interface PendingApplication {
  id: string;
  user_pubkey: string;
  status: 'pending' | 'approved' | 'rejected';
  submitted_at: string;
  qualification_score: number;
  wallet_age_days: number;
  token_amount: number;
  staking_history_valid: boolean;
}

function PatronApprovals() {
  const [pendingApplications, setPendingApplications] = useState<PendingApplication[]>([]);
  const [loading, setLoading] = useState(false);
  const [processingApp, setProcessingApp] = useState<string | null>(null);

  useEffect(() => {
    fetchPendingApplications();
  }, []);

  const fetchPendingApplications = async () => {
    setLoading(true);
    try {
      const response = await patronApi.getPendingApplications();
      if (response.success && response.data) {
        setPendingApplications(response.data as PendingApplication[]);
      }
    } catch (error) {
      console.error('Failed to fetch pending applications:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleApproveApplication = async (application: PendingApplication) => {
    setProcessingApp(application.id);
    try {
      const minScore = 60; // Minimum qualification score
      const result = await patronApi.approvePatronApplication(application.user_pubkey, minScore);

      if (result.success) {
        console.log('Patron approval transaction:', result.data);
        // Refresh the list
        fetchPendingApplications();
      } else {
        throw new Error(result.error || 'Failed to approve patron application');
      }
    } catch (error) {
      console.error('Error approving patron application:', error);
      alert('Error approving application: ' + error);
    } finally {
      setProcessingApp(null);
    }
  };

  const handleRejectApplication = async (application: PendingApplication) => {
    setProcessingApp(application.id);
    try {
      // For rejection, we use the revoke function or a specific reject endpoint
      const result = await patronApi.revokePatronStatus(application.user_pubkey);

      if (result.success) {
        console.log('Patron rejection transaction:', result.data);
        // Refresh the list
        fetchPendingApplications();
      } else {
        throw new Error(result.error || 'Failed to reject patron application');
      }
    } catch (error) {
      console.error('Error rejecting patron application:', error);
      alert('Error rejecting application: ' + error);
    } finally {
      setProcessingApp(null);
    }
  };

  const getScoreColor = (score: number) => {
    if (score >= 80) return 'success';
    if (score >= 60) return 'warning';
    return 'danger';
  };

  return (
    <div className="container-fluid">
      {/* Breadcrumb */}
      <nav aria-label="breadcrumb" className="mb-3">
        <ol className="breadcrumb">
          <li className="breadcrumb-item"><Link to="/home">Home</Link></li>
          <li className="breadcrumb-item"><Link to="/admin">Admin Dashboard</Link></li>
          <li className="breadcrumb-item active" aria-current="page">Patron Approvals</li>
        </ol>
      </nav>

      <div className="row">
        <div className="col-12">
          <div className="card">
            <div className="card-header d-flex justify-content-between align-items-center">
              <h4 className="mb-0">👑 Patron Application Approvals</h4>
              <button 
                className="btn btn-primary btn-sm"
                onClick={fetchPendingApplications}
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
                  <p className="mt-2">Loading pending applications...</p>
                </div>
              ) : pendingApplications.length === 0 ? (
                <div className="alert alert-info text-center">
                  <h5>No Pending Applications</h5>
                  <p className="mb-0">All patron applications have been processed.</p>
                </div>
              ) : (
                <div className="table-responsive">
                  <table className="table table-striped table-hover">
                    <thead>
                      <tr>
                        <th>Application ID</th>
                        <th>User Pubkey</th>
                        <th>Submitted</th>
                        <th>Qualification Score</th>
                        <th>Requirements Status</th>
                        <th>Actions</th>
                      </tr>
                    </thead>
                    <tbody>
                      {pendingApplications.map((app) => (
                        <tr key={app.id}>
                          <td>
                            <code className="text-muted">#{app.id.slice(0, 8)}...</code>
                          </td>
                          <td>
                            <small>
                              <code>{app.user_pubkey.slice(0, 8)}...{app.user_pubkey.slice(-8)}</code>
                            </small>
                          </td>
                          <td>
                            {new Date(app.submitted_at).toLocaleDateString()}
                            <br />
                            <small className="text-muted">
                              {new Date(app.submitted_at).toLocaleTimeString()}
                            </small>
                          </td>
                          <td>
                            <span className={`badge bg-${getScoreColor(app.qualification_score)} fs-6`}>
                              {app.qualification_score}
                            </span>
                            <br />
                            <small className="text-muted">
                              {app.qualification_score >= 60 ? 'Qualified' : 'Below Threshold'}
                            </small>
                          </td>
                          <td>
                            <div className="d-flex flex-column gap-1">
                              <div>
                                <span className="badge bg-success me-2">✓</span>
                                <small>Wallet Age: {app.wallet_age_days || 'N/A'} days</small>
                              </div>
                              <div>
                                <span className="badge bg-success me-2">✓</span>
                                <small>Token Amount: {app.token_amount ? (app.token_amount / 1_000_000_000).toLocaleString() : 'N/A'} SNAKE</small>
                              </div>
                              <div>
                                <span className={`badge ${app.staking_history_valid ? 'bg-success' : 'bg-warning'} me-2`}>
                                  {app.staking_history_valid ? '✓' : '⚠'}
                                </span>
                                <small>6-Month Staking: {app.staking_history_valid ? 'Valid' : 'Pending Validation'}</small>
                              </div>
                            </div>
                          </td>
                          <td>
                            <div className="btn-group-vertical" role="group">
                              <button
                                className="btn btn-success btn-sm"
                                onClick={() => handleApproveApplication(app)}
                                disabled={processingApp === app.id || loading}
                              >
                                {processingApp === app.id ? (
                                  <>
                                    <span className="spinner-border spinner-border-sm me-1" />
                                    Approving...
                                  </>
                                ) : (
                                  '✓ Approve'
                                )}
                              </button>
                              <button
                                className="btn btn-danger btn-sm"
                                onClick={() => handleRejectApplication(app)}
                                disabled={processingApp === app.id || loading}
                              >
                                {processingApp === app.id ? (
                                  <>
                                    <span className="spinner-border spinner-border-sm me-1" />
                                    Rejecting...
                                  </>
                                ) : (
                                  '✗ Reject'
                                )}
                              </button>
                            </div>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </div>

            <div className="card-footer">
              <div className="row">
                <div className="col-md-6">
                  <h6>Approval Guidelines:</h6>
                  <ul className="small mb-0">
                    <li>Qualification score ≥ 60 recommended</li>
                    <li>All requirements must be met</li>
                    <li>6-month staking validated on-chain</li>
                  </ul>
                </div>
                <div className="col-md-6">
                  <h6>Next Steps After Approval:</h6>
                  <ul className="small mb-0">
                    <li>User gains patron status immediately</li>
                    <li>DAO voting rights activated</li>
                    <li>Access to premium features enabled</li>
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

export default PatronApprovals;
