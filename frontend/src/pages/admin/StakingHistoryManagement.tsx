import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { adminApi } from '../patron/services/apiService';

interface StakingRecord {
  id: string;
  user_pubkey: string;
  username?: string;
  staked_amount: number;
  stake_start_date: string;
  stake_end_date: string | null;
  duration_months: number;
  is_active: boolean;
  validation_status: 'pending' | 'validated' | 'invalid';
  created_at: string;
  updated_at: string;
}

interface StakingHistoryData {
  staking_records: StakingRecord[];
  total_count: number;
  page: number;
  per_page: number;
}

interface UserStakingHistory {
  user_pubkey: string;
  username?: string;
  staking_records: StakingRecord[];
  total_staked: number;
  longest_stake_months: number;
  validation_summary: {
    total_records: number;
    validated: number;
    pending: number;
    invalid: number;
  };
}

function StakingHistoryManagement() {
  const [stakingHistory, setStakingHistory] = useState<StakingHistoryData | null>(null);
  const [selectedUser, setSelectedUser] = useState<UserStakingHistory | null>(null);
  const [loading, setLoading] = useState(false);
  const [userLoading, setUserLoading] = useState(false);
  const [processing, setProcessing] = useState<string | null>(null);
  const [currentPage, setCurrentPage] = useState(1);
  const [searchPubkey, setSearchPubkey] = useState('');

  useEffect(() => {
    fetchStakingHistory();
  }, [currentPage]);

  const fetchStakingHistory = async () => {
    setLoading(true);
    try {
      const response = await adminApi.getAllStakingHistory(currentPage, 50);
      if (response.success && response.data) {
        setStakingHistory(response.data as StakingHistoryData);
      }
    } catch (error) {
      console.error('Failed to fetch staking history:', error);
    } finally {
      setLoading(false);
    }
  };

  const fetchUserStakingHistory = async (userPubkey: string) => {
    setUserLoading(true);
    try {
      const response = await adminApi.getUserStakingHistory(userPubkey);
      if (response.success && response.data) {
        setSelectedUser(response.data as UserStakingHistory);
      }
    } catch (error) {
      console.error('Failed to fetch user staking history:', error);
      alert('Failed to fetch user staking history: ' + error);
    } finally {
      setUserLoading(false);
    }
  };

  const handleValidateRecord = async (userPubkey: string, recordId: string) => {
    setProcessing(recordId);
    try {
      const result = await adminApi.validateStakingHistory(userPubkey, recordId);

      if (result.success) {
        console.log('Staking validation transaction:', result.data);
        // Refresh the data
        fetchStakingHistory();
        if (selectedUser) {
          fetchUserStakingHistory(selectedUser.user_pubkey);
        }
        alert('Staking record validated successfully!');
      } else {
        throw new Error(result.error || 'Failed to validate staking record');
      }
    } catch (error) {
      console.error('Error validating staking record:', error);
      alert('Error validating staking record: ' + error);
    } finally {
      setProcessing(null);
    }
  };

  const handleInvalidateRecord = async (userPubkey: string, recordId: string) => {
    if (!window.confirm('Are you sure you want to invalidate this staking record?')) {
      return;
    }

    setProcessing(recordId);
    try {
      const result = await adminApi.invalidateStakingHistory(userPubkey, recordId);

      if (result.success) {
        console.log('Staking invalidation transaction:', result.data);
        // Refresh the data
        fetchStakingHistory();
        if (selectedUser) {
          fetchUserStakingHistory(selectedUser.user_pubkey);
        }
        alert('Staking record invalidated successfully!');
      } else {
        throw new Error(result.error || 'Failed to invalidate staking record');
      }
    } catch (error) {
      console.error('Error invalidating staking record:', error);
      alert('Error invalidating staking record: ' + error);
    } finally {
      setProcessing(null);
    }
  };

  const getValidationStatusBadge = (status: string) => {
    switch (status) {
      case 'validated': return 'bg-success';
      case 'invalid': return 'bg-danger';
      case 'pending': return 'bg-warning';
      default: return 'bg-secondary';
    }
  };

  const getValidationStatusText = (status: string) => {
    switch (status) {
      case 'validated': return 'Validated';
      case 'invalid': return 'Invalid';
      case 'pending': return 'Pending';
      default: return 'Unknown';
    }
  };

  const handleSearchUser = () => {
    if (searchPubkey.trim()) {
      fetchUserStakingHistory(searchPubkey.trim());
    }
  };

  return (
    <div className="container-fluid">
      {/* Breadcrumb */}
      <nav aria-label="breadcrumb" className="mb-3">
        <ol className="breadcrumb">
          <li className="breadcrumb-item"><Link to="/home">Home</Link></li>
          <li className="breadcrumb-item"><Link to="/admin">Admin Dashboard</Link></li>
          <li className="breadcrumb-item active" aria-current="page">Staking History</li>
        </ol>
      </nav>

      <div className="row">
        <div className="col-12">
          <div className="card">
            <div className="card-header d-flex justify-content-between align-items-center">
              <h4 className="mb-0">📊 Staking History Management</h4>
              <button 
                className="btn btn-primary btn-sm"
                onClick={fetchStakingHistory}
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
              {/* User Search Section */}
              <div className="row mb-4">
                <div className="col-md-8">
                  <div className="input-group">
                    <input
                      type="text"
                      className="form-control"
                      placeholder="Enter user public key to view detailed staking history..."
                      value={searchPubkey}
                      onChange={(e) => setSearchPubkey(e.target.value)}
                      onKeyPress={(e) => e.key === 'Enter' && handleSearchUser()}
                    />
                    <button
                      className="btn btn-outline-primary"
                      onClick={handleSearchUser}
                      disabled={userLoading || !searchPubkey.trim()}
                    >
                      {userLoading ? (
                        <span className="spinner-border spinner-border-sm" />
                      ) : (
                        'Search User'
                      )}
                    </button>
                  </div>
                </div>
              </div>

              {/* Selected User Details */}
              {selectedUser && (
                <div className="card border-primary mb-4">
                  <div className="card-header bg-primary text-white">
                    <h5 className="mb-0">User Staking Details</h5>
                  </div>
                  <div className="card-body">
                    <div className="row">
                      <div className="col-md-6">
                        <p><strong>Public Key:</strong> <code>{selectedUser.user_pubkey}</code></p>
                        {selectedUser.username && (
                          <p><strong>Username:</strong> {selectedUser.username}</p>
                        )}
                        <p><strong>Total Staked:</strong> {(selectedUser.total_staked / 1_000_000_000).toLocaleString()} SNAKE</p>
                        <p><strong>Longest Stake:</strong> {selectedUser.longest_stake_months} months</p>
                      </div>
                      <div className="col-md-6">
                        <h6>Validation Summary:</h6>
                        <div className="d-flex gap-2 mb-2">
                          <span className="badge bg-success">Validated: {selectedUser.validation_summary.validated}</span>
                          <span className="badge bg-warning">Pending: {selectedUser.validation_summary.pending}</span>
                          <span className="badge bg-danger">Invalid: {selectedUser.validation_summary.invalid}</span>
                        </div>
                        <p><strong>Total Records:</strong> {selectedUser.validation_summary.total_records}</p>
                      </div>
                    </div>

                    <div className="table-responsive mt-3">
                      <table className="table table-sm">
                        <thead>
                          <tr>
                            <th>Amount</th>
                            <th>Start Date</th>
                            <th>Duration</th>
                            <th>Status</th>
                            <th>Actions</th>
                          </tr>
                        </thead>
                        <tbody>
                          {selectedUser.staking_records.map((record) => (
                            <tr key={record.id}>
                              <td>{(record.staked_amount / 1_000_000_000).toLocaleString()} SNAKE</td>
                              <td>{new Date(record.stake_start_date).toLocaleDateString()}</td>
                              <td>{record.duration_months} months</td>
                              <td>
                                <span className={`badge ${getValidationStatusBadge(record.validation_status)}`}>
                                  {getValidationStatusText(record.validation_status)}
                                </span>
                              </td>
                              <td>
                                {record.validation_status === 'pending' && (
                                  <div className="btn-group btn-group-sm">
                                    <button
                                      className="btn btn-success btn-sm"
                                      onClick={() => handleValidateRecord(selectedUser.user_pubkey, record.id)}
                                      disabled={processing === record.id}
                                    >
                                      {processing === record.id ? (
                                        <span className="spinner-border spinner-border-sm" />
                                      ) : (
                                        '✓'
                                      )}
                                    </button>
                                    <button
                                      className="btn btn-danger btn-sm"
                                      onClick={() => handleInvalidateRecord(selectedUser.user_pubkey, record.id)}
                                      disabled={processing === record.id}
                                    >
                                      {processing === record.id ? (
                                        <span className="spinner-border spinner-border-sm" />
                                      ) : (
                                        '✗'
                                      )}
                                    </button>
                                  </div>
                                )}
                              </td>
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  </div>
                </div>
              )}

              {/* All Staking Records Table */}
              {loading ? (
                <div className="text-center py-4">
                  <span className="spinner-border" />
                  <p className="mt-2">Loading staking history...</p>
                </div>
              ) : stakingHistory && stakingHistory.staking_records.length > 0 ? (
                <div className="table-responsive">
                  <table className="table table-striped table-hover">
                    <thead>
                      <tr>
                        <th>User</th>
                        <th>Staked Amount</th>
                        <th>Start Date</th>
                        <th>Duration</th>
                        <th>Status</th>
                        <th>Validation</th>
                        <th>Actions</th>
                      </tr>
                    </thead>
                    <tbody>
                      {stakingHistory.staking_records.map((record) => (
                        <tr key={record.id}>
                          <td>
                            <div>
                              <small>
                                <code>{record.user_pubkey.slice(0, 8)}...{record.user_pubkey.slice(-8)}</code>
                              </small>
                              {record.username && (
                                <div className="text-muted small">{record.username}</div>
                              )}
                            </div>
                          </td>
                          <td>{(record.staked_amount / 1_000_000_000).toLocaleString()} SNAKE</td>
                          <td>
                            {new Date(record.stake_start_date).toLocaleDateString()}
                            <br />
                            <small className="text-muted">
                              {new Date(record.stake_start_date).toLocaleTimeString()}
                            </small>
                          </td>
                          <td>
                            <span className="badge bg-info">
                              {record.duration_months} months
                            </span>
                            <br />
                            <small className={`text-${record.is_active ? 'success' : 'muted'}`}>
                              {record.is_active ? 'Active' : 'Completed'}
                            </small>
                          </td>
                          <td>
                            <span className={`badge ${record.is_active ? 'bg-success' : 'bg-secondary'}`}>
                              {record.is_active ? 'Active' : 'Completed'}
                            </span>
                          </td>
                          <td>
                            <span className={`badge ${getValidationStatusBadge(record.validation_status)}`}>
                              {getValidationStatusText(record.validation_status)}
                            </span>
                          </td>
                          <td>
                            <div className="btn-group-vertical btn-group-sm">
                              <button
                                className="btn btn-outline-primary btn-sm"
                                onClick={() => fetchUserStakingHistory(record.user_pubkey)}
                                disabled={userLoading}
                              >
                                View Details
                              </button>
                              {record.validation_status === 'pending' && (
                                <>
                                  <button
                                    className="btn btn-success btn-sm"
                                    onClick={() => handleValidateRecord(record.user_pubkey, record.id)}
                                    disabled={processing === record.id}
                                  >
                                    {processing === record.id ? (
                                      <span className="spinner-border spinner-border-sm" />
                                    ) : (
                                      '✓ Validate'
                                    )}
                                  </button>
                                  <button
                                    className="btn btn-danger btn-sm"
                                    onClick={() => handleInvalidateRecord(record.user_pubkey, record.id)}
                                    disabled={processing === record.id}
                                  >
                                    {processing === record.id ? (
                                      <span className="spinner-border spinner-border-sm" />
                                    ) : (
                                      '✗ Invalidate'
                                    )}
                                  </button>
                                </>
                              )}
                            </div>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>

                  {/* Pagination */}
                  {stakingHistory.total_count > stakingHistory.per_page && (
                    <nav>
                      <ul className="pagination justify-content-center">
                        <li className={`page-item ${currentPage === 1 ? 'disabled' : ''}`}>
                          <button
                            className="page-link"
                            onClick={() => setCurrentPage(currentPage - 1)}
                            disabled={currentPage === 1}
                          >
                            Previous
                          </button>
                        </li>
                        <li className="page-item active">
                          <span className="page-link">
                            Page {currentPage} of {Math.ceil(stakingHistory.total_count / stakingHistory.per_page)}
                          </span>
                        </li>
                        <li className={`page-item ${currentPage >= Math.ceil(stakingHistory.total_count / stakingHistory.per_page) ? 'disabled' : ''}`}>
                          <button
                            className="page-link"
                            onClick={() => setCurrentPage(currentPage + 1)}
                            disabled={currentPage >= Math.ceil(stakingHistory.total_count / stakingHistory.per_page)}
                          >
                            Next
                          </button>
                        </li>
                      </ul>
                    </nav>
                  )}
                </div>
              ) : (
                <div className="alert alert-info text-center">
                  <h5>No Staking Records Found</h5>
                  <p className="mb-0">No staking history records are available.</p>
                </div>
              )}
            </div>

            <div className="card-footer">
              <div className="row">
                <div className="col-md-6">
                  <h6>Validation Guidelines:</h6>
                  <ul className="small mb-0">
                    <li>Validate records with proper on-chain proof</li>
                    <li>Check staking duration and amounts carefully</li>
                    <li>Invalidate suspicious or incorrect records</li>
                  </ul>
                </div>
                <div className="col-md-6">
                  <h6>Status Meanings:</h6>
                  <ul className="small mb-0">
                    <li><span className="badge bg-success">Validated</span> - Verified and approved</li>
                    <li><span className="badge bg-warning">Pending</span> - Awaiting validation</li>
                    <li><span className="badge bg-danger">Invalid</span> - Rejected or incorrect</li>
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

export default StakingHistoryManagement;