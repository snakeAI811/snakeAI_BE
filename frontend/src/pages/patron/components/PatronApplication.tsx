import React, { useState, useEffect } from 'react';
import { UserRole } from '../index';
import { patronApi } from '../services/apiService';

interface PatronApplicationProps {
  userRole: UserRole;
}

interface Application {
  id: string;
  status: 'pending' | 'approved' | 'rejected';
  submitted_at: string;
  reason?: string;
}

function PatronApplication({ userRole }: PatronApplicationProps) {
  const [application, setApplication] = useState<Application | null>(null);
  const [applicationReason, setApplicationReason] = useState('');
  const [loading, setLoading] = useState(false);
  const [pendingApplications, setPendingApplications] = useState<Application[]>([]);

  useEffect(() => {
    fetchApplicationStatus();
    if (userRole.role === 'Patron') {
      fetchPendingApplications();
    }
  }, [userRole]);

  const fetchApplicationStatus = async () => {
    try {
      // TODO: Replace with actual API call
      // Mock data for now
      setApplication({
        id: '1',
        status: 'approved',
        submitted_at: new Date().toISOString(),
        reason: 'Experienced community member with significant contributions'
      });
    } catch (error) {
      console.error('Failed to fetch application status:', error);
    }
  };

  const fetchPendingApplications = async () => {
    try {
      // TODO: Replace with actual API call for admins/patrons
      // Mock data for now
      setPendingApplications([
        {
          id: '2',
          status: 'pending',
          submitted_at: new Date().toISOString(),
          reason: 'Active trader seeking governance participation'
        },
        {
          id: '3',
          status: 'pending',
          submitted_at: new Date().toISOString(),
          reason: 'Long-term holder wanting to contribute to ecosystem'
        }
      ]);
    } catch (error) {
      console.error('Failed to fetch pending applications:', error);
    }
  };

  const handleApplyForPatron = async () => {
    if (!applicationReason.trim()) return;

    setLoading(true);
    try {
      const result = await patronApi.applyForPatron(applicationReason);

      if (result.success) {
        console.log('Patron application transaction:', result.data);
        setApplicationReason('');
        fetchApplicationStatus(); // Refresh application status
      } else {
        throw new Error(result.error || 'Failed to submit patron application');
      }
    } catch (error) {
      console.error('Error applying for patron:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleApproveApplication = async (applicationId: string) => {
    setLoading(true);
    try {
      const result = await patronApi.approvePatron(applicationId, true);

      if (result.success) {
        console.log('Patron approval transaction:', result.data);
        fetchPendingApplications(); // Refresh pending applications
      } else {
        throw new Error(result.error || 'Failed to approve patron application');
      }
    } catch (error) {
      console.error('Error approving patron application:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleRejectApplication = async (applicationId: string) => {
    setLoading(true);
    try {
      const result = await patronApi.approvePatron(applicationId, false);

      if (result.success) {
        console.log('Patron rejection transaction:', result.data);
        fetchPendingApplications(); // Refresh pending applications
      } else {
        throw new Error(result.error || 'Failed to reject patron application');
      }
    } catch (error) {
      console.error('Error rejecting patron application:', error);
    } finally {
      setLoading(false);
    }
  };

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'approved':
        return <span className="badge bg-success">Approved</span>;
      case 'rejected':
        return <span className="badge bg-danger">Rejected</span>;
      case 'pending':
        return <span className="badge bg-warning">Pending</span>;
      default:
        return <span className="badge bg-secondary">Unknown</span>;
    }
  };

  return (
    <div className="w-100">
      <h3 className="mb-4">👑 Patron Features</h3>

      {userRole.role !== 'Patron' && !application && (
        /* Apply for Patron */
        <div className="card border-warning mb-4">
          <div className="card-body">
            <h5 className="card-title">🎯 Apply to Become a Patron</h5>
            <p className="card-text">
              Patrons enjoy exclusive benefits including DAO governance rights, OTC trading rebates, 
              and access to premium features.
            </p>

            <div className="mb-3">
              <label className="form-label">Application Reason</label>
              <textarea
                className="form-control"
                rows={4}
                placeholder="Explain why you want to become a Patron and how you'll contribute to the ecosystem..."
                value={applicationReason}
                onChange={(e) => setApplicationReason(e.target.value)}
                disabled={loading}
              />
            </div>

            <div className="alert alert-info">
              <strong>Requirements:</strong>
              <ul className="mb-0 mt-2">
                <li>Must be a Staker in good standing</li>
                <li>Minimum token holding requirements</li>
                <li>6-month commitment period</li>
                <li>Community approval process</li>
              </ul>
            </div>

            <button
              className="btn btn-warning btn-lg"
              onClick={handleApplyForPatron}
              disabled={loading || !applicationReason.trim()}
            >
              {loading ? (
                <>
                  <span className="spinner-border spinner-border-sm me-2" />
                  Submitting Application...
                </>
              ) : (
                'Submit Patron Application'
              )}
            </button>
          </div>
        </div>
      )}

      {application && (
        /* Application Status */
        <div className="card border-info mb-4">
          <div className="card-body">
            <h5 className="card-title">📋 Your Patron Application</h5>
            <div className="row">
              <div className="col-md-6">
                <strong>Status:</strong> {getStatusBadge(application.status)}
              </div>
              <div className="col-md-6">
                <strong>Submitted:</strong> {new Date(application.submitted_at).toLocaleDateString()}
              </div>
            </div>
            <div className="mt-3">
              <strong>Reason:</strong>
              <p className="mt-2">{application.reason}</p>
            </div>
          </div>
        </div>
      )}

      {userRole.role === 'Patron' && (
        /* Patron Exclusive Features */
        <div className="row g-4 mb-4">
          <div className="col-lg-6">
            <div className="card border-success h-100">
              <div className="card-body">
                <h5 className="card-title">🗳️ DAO Governance</h5>
                <p className="card-text">Participate in ecosystem governance and voting.</p>
                <ul>
                  <li>Vote on protocol proposals</li>
                  <li>Submit governance proposals</li>
                  <li>Access to Patron-only discussions</li>
                </ul>
                <button className="btn btn-success">Access Governance</button>
              </div>
            </div>
          </div>

          <div className="col-lg-6">
            <div className="card border-info h-100">
              <div className="card-body">
                <h5 className="card-title">💎 Premium Features</h5>
                <p className="card-text">Exclusive tools and enhanced functionality.</p>
                <ul>
                  <li>Advanced analytics dashboard</li>
                  <li>Priority customer support</li>
                  <li>Beta feature access</li>
                </ul>
                <button className="btn btn-info">Explore Features</button>
              </div>
            </div>
          </div>
        </div>
      )}

      {userRole.role === 'Patron' && pendingApplications.length > 0 && (
        /* Approve Applications (for Patrons) */
        <div className="card border-warning">
          <div className="card-body">
            <h5 className="card-title">📨 Pending Patron Applications</h5>
            <p className="card-text">Review and approve new Patron applications.</p>

            <div className="table-responsive">
              <table className="table table-striped">
                <thead>
                  <tr>
                    <th>Application ID</th>
                    <th>Submitted</th>
                    <th>Reason</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {pendingApplications.map((app) => (
                    <tr key={app.id}>
                      <td>#{app.id}</td>
                      <td>{new Date(app.submitted_at).toLocaleDateString()}</td>
                      <td>
                        <div style={{ maxWidth: '300px' }}>
                          {app.reason}
                        </div>
                      </td>
                      <td>
                        <div className="btn-group" role="group">
                          <button
                            className="btn btn-success btn-sm"
                            onClick={() => handleApproveApplication(app.id)}
                            disabled={loading}
                          >
                            Approve
                          </button>
                          <button
                            className="btn btn-danger btn-sm"
                            onClick={() => handleRejectApplication(app.id)}
                            disabled={loading}
                          >
                            Reject
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      )}

      {/* Patron Benefits Overview */}
      <div className="mt-4">
        <div className="card border-secondary">
          <div className="card-body">
            <h6 className="card-title">✨ Patron Benefits</h6>
            <div className="row">
              <div className="col-md-6">
                <h6>Financial Benefits:</h6>
                <ul>
                  <li>Enhanced staking rewards</li>
                  <li>OTC trading rebates</li>
                  <li>Priority access to new features</li>
                  <li>Reduced transaction fees</li>
                </ul>
              </div>
              <div className="col-md-6">
                <h6>Governance Rights:</h6>
                <ul>
                  <li>DAO voting power</li>
                  <li>Proposal submission rights</li>
                  <li>Community leadership role</li>
                  <li>Exclusive patron discussions</li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default PatronApplication;
