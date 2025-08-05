import React, { useState, useEffect } from 'react';
import { UserRole } from '../index';
import { patronApi, roleApi } from '../services/apiService';

interface PatronApplicationProps {
  userRole: UserRole;
}

interface Application {
  id: string;
  status: 'none' | 'pending' | 'approved' | 'rejected';
  submitted_at: string;
  qualification_score: number;
}

interface EligibilityCheck {
  eligible: boolean;
  requirements: {
    token_amount: { required: number; current: number; met: boolean };
    wallet_age: { required_days: number; current_days: number; met: boolean };
    mining_history: { required: number; current: number; met: boolean };
    staking_history: { required_months: number; met: boolean; note: string };
  };
  errors: string[];
}

function PatronApplication({ userRole }: PatronApplicationProps) {
  const [application, setApplication] = useState<Application | null>(null);
  const [isCalculating, setIsCalculating] = useState(false);
  const [loading, setLoading] = useState(false);
  const [pendingApplications, setPendingApplications] = useState<Application[]>([]);
  const [eligibilityCheck, setEligibilityCheck] = useState<EligibilityCheck | null>(null);
  const [checkingEligibility, setCheckingEligibility] = useState(false);

  useEffect(() => {
    fetchApplicationStatus();
    if (userRole.role === 'patron') {
      fetchPendingApplications();
    }
  }, [userRole]);

  const fetchApplicationStatus = async () => {
    try {
      const response = await patronApi.getApplicationStatus();
      if (response.success && response.data) {
        // If status is 'none', treat as no application
        if (response.data.status === 'none') {
          setApplication(null);
        } else {
          setApplication(response.data);
        }
      } else {
        // No application found - user hasn't applied yet
        setApplication(null);
      }
    } catch (error) {
      console.error('Failed to fetch application status:', error);
      setApplication(null);
    }
  };

  const fetchPendingApplications = async () => {
    try {
      const response = await patronApi.getPendingApplications();
      if (response.success && response.data) {
        // Map the API response to match our interface
        const mappedApplications = response.data.map(app => ({
          ...app,
          status: app.status as 'none' | 'pending' | 'approved' | 'rejected'
        }));
        setPendingApplications(mappedApplications);
      } else {
        setPendingApplications([]);
      }
    } catch (error) {
      console.error('Failed to fetch pending applications:', error);
      setPendingApplications([]);
    }
  };

  const checkPatronEligibility = async () => {
    setCheckingEligibility(true);
    try {
      // Mock values - in real app, these would come from user data and wallet
      const tokenAmount = 250000 * 1_000_000_000; // 250k tokens in smallest units
      const walletAgeDays = 35; // Mock wallet age
      const totalMinedPhase1 = 1000 * 1_000_000_000; // Mock mined amount
      
      const response = await roleApi.checkPatronEligibility(tokenAmount, walletAgeDays, totalMinedPhase1);
      if (response.success && response.data) {
        setEligibilityCheck(response.data);
      }
    } catch (error) {
      console.error('Failed to check patron eligibility:', error);
    } finally {
      setCheckingEligibility(false);
    }
  };

  const handleApplyForPatron = async () => {
    setLoading(true);
    setIsCalculating(true);
    
    try {
      // Calculate wallet age (mock calculation - in real app, this would come from wallet connection)
      const walletAgedays = 30; // Placeholder: could be calculated from user.created_at or wallet creation time
      
      // Calculate community score (mock calculation - could be based on tweets, likes, etc.)
      const communityScore = Math.min(20, 10); // Placeholder: based on engagement metrics
      
      const result = await patronApi.applyForPatron(walletAgedays, communityScore);

      if (result.success) {
        console.log('Patron application transaction:', result.data);
        fetchApplicationStatus(); // Refresh application status
      } else {
        throw new Error(result.error || 'Failed to submit patron application');
      }
    } catch (error) {
      console.error('Error applying for patron:', error);
    } finally {
      setLoading(false);
      setIsCalculating(false);
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
      case 'none':
        return <span className="badge bg-secondary">Not Applied</span>;
      default:
        return <span className="badge bg-secondary">Unknown</span>;
    }
  };

  return (
    <div className="w-100">
      <h3 className="mb-4">👑 Patron Features</h3>

      {userRole.role !== 'patron' && !application && (
        /* Apply for Patron */
        <div className="card border-warning mb-4">
          <div className="card-body">
            <h5 className="card-title">🎯 Apply to Become a Patron</h5>
            <p className="card-text">
              Patrons enjoy exclusive benefits including DAO governance rights, OTC trading rebates, 
              and access to premium features.
            </p>

            <div className="mb-3">
              <div className="alert alert-secondary">
                <h6>📊 Automatic Qualification Assessment</h6>
                <p className="mb-1">Your application will be automatically evaluated based on:</p>
                <ul className="mb-0">
                  <li>Wallet age and transaction history</li>
                  <li>Community engagement and tweet count</li>
                  <li>Current staking and token holdings</li>
                  <li>Overall platform participation</li>
                </ul>
                {isCalculating && (
                  <div className="mt-2">
                    <span className="spinner-border spinner-border-sm me-2" />
                    <small>Calculating qualification score...</small>
                  </div>
                )}
              </div>
            </div>

            <div className="alert alert-info">
              <div className="d-flex justify-content-between align-items-center mb-2">
                <strong>Requirements:</strong>
                <button 
                  className="btn btn-sm btn-outline-primary"
                  onClick={checkPatronEligibility}
                  disabled={checkingEligibility}
                >
                  {checkingEligibility ? (
                    <>
                      <span className="spinner-border spinner-border-sm me-2" />
                      Checking...
                    </>
                  ) : (
                    'Check Eligibility'
                  )}
                </button>
              </div>
              
              {eligibilityCheck ? (
                <div className="mt-3">
                  <h6 className={`mb-2 ${eligibilityCheck.eligible ? 'text-success' : 'text-warning'}`}>
                    {eligibilityCheck.eligible ? '✅ Eligible for Patron Role' : '⚠️ Requirements Not Met'}
                  </h6>
                  
                  <div className="row">
                    <div className="col-md-6">
                      <div className="mb-2">
                        <span className={`badge ${eligibilityCheck.requirements.token_amount.met ? 'bg-success' : 'bg-danger'} me-2`}>
                          {eligibilityCheck.requirements.token_amount.met ? '✓' : '✗'}
                        </span>
                        <small>Token Amount: {(eligibilityCheck.requirements.token_amount.current / 1_000_000_000).toLocaleString()} / {(eligibilityCheck.requirements.token_amount.required / 1_000_000_000).toLocaleString()} SNAKE</small>
                      </div>
                      <div className="mb-2">
                        <span className={`badge ${eligibilityCheck.requirements.wallet_age.met ? 'bg-success' : 'bg-danger'} me-2`}>
                          {eligibilityCheck.requirements.wallet_age.met ? '✓' : '✗'}
                        </span>
                        <small>Wallet Age: {eligibilityCheck.requirements.wallet_age.current_days} / {eligibilityCheck.requirements.wallet_age.required_days} days</small>
                      </div>
                    </div>
                    <div className="col-md-6">
                      <div className="mb-2">
                        <span className={`badge ${eligibilityCheck.requirements.mining_history.met ? 'bg-success' : 'bg-danger'} me-2`}>
                          {eligibilityCheck.requirements.mining_history.met ? '✓' : '✗'}
                        </span>
                        <small>Mining History: {(eligibilityCheck.requirements.mining_history.current / 1_000_000_000).toLocaleString()} SNAKE mined</small>
                      </div>
                      <div className="mb-2">
                        <span className="badge bg-info me-2">ℹ</span>
                        <small>Staking History: {eligibilityCheck.requirements.staking_history.required_months} months (validated at token lock)</small>
                      </div>
                    </div>
                  </div>
                  
                  {eligibilityCheck.errors.length > 0 && (
                    <div className="alert alert-warning mt-2 mb-0">
                      <small><strong>Issues to resolve:</strong></small>
                      <ul className="mb-0 mt-1">
                        {eligibilityCheck.errors.map((error, index) => (
                          <li key={index}><small>{error}</small></li>
                        ))}
                      </ul>
                    </div>
                  )}
                </div>
              ) : (
                <ul className="mb-0 mt-2">
                  <li>Must have ≥250k SNAKE tokens</li>
                  <li>Minimum 30 days wallet age</li>
                  <li>Mining history in Phase 1</li>
                  <li>6-month staking commitment</li>
                </ul>
              )}
            </div>

            <button
              className="btn btn-warning btn-lg"
              onClick={handleApplyForPatron}
              disabled={loading}
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
              <strong>Qualification Score:</strong>
              <p className="mt-2">
                <span className="badge bg-info fs-6">{application.qualification_score}</span>
                <small className="text-muted ms-2">
                  (Based on wallet age, community engagement, and other factors)
                </small>
              </p>
            </div>
          </div>
        </div>
      )}

      {userRole.role === 'patron' && (
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

      {userRole.role === 'patron' && pendingApplications.length > 0 && (
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
                    <th>Qualification Score</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {pendingApplications.map((app) => (
                    <tr key={app.id}>
                      <td>#{app.id}</td>
                      <td>{new Date(app.submitted_at).toLocaleDateString()}</td>
                      <td>
                        <span className="badge bg-info fs-6">{app.qualification_score}</span>
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
