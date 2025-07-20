import React, { useState } from 'react';
import { UserRole } from '../index';
import { roleApi } from '../services/apiService';

interface RoleSelectionProps {
  userRole: UserRole;
  onRoleChange: (role: UserRole) => void;
}

function RoleSelection({ userRole, onRoleChange }: RoleSelectionProps) {
  const [selectedRole, setSelectedRole] = useState<'None' | 'Staker' | 'Patron'>(userRole.role);
  const [loading, setLoading] = useState(false);

  const roleDescriptions = {
    None: {
      title: 'No Role',
      description: 'Basic access to mining and standard features',
      benefits: ['Basic tweet mining', 'Standard rewards', 'Community access'],
      icon: '👤',
      color: 'secondary'
    },
    Staker: {
      title: 'Staker',
      description: 'Lock tokens for enhanced rewards and yield generation',
      benefits: ['5% APY staking rewards', 'Enhanced mining multiplier', 'Priority support', '3-month lock period'],
      icon: '🏦',
      color: 'primary'
    },
    Patron: {
      title: 'Patron',
      description: 'Premium tier with exclusive features and governance rights',
      benefits: ['All Staker benefits', 'DAO governance rights', 'OTC trading rebates', 'Exclusive features', '6-month lock period'],
      icon: '👑',
      color: 'warning'
    }
  };

  const handleRoleSelect = async () => {
    if (selectedRole === userRole.role) return;
    
    setLoading(true);
    try {
      const result = await roleApi.selectRole(selectedRole);

      if (result.success) {
        onRoleChange({ role: selectedRole });
        console.log('Role selection transaction:', result.data);
      } else {
        throw new Error(result.error || 'Failed to select role');
      }
    } catch (error) {
      console.error('Error selecting role:', error);
      // Reset selection on error
      setSelectedRole(userRole.role);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="w-100">
      <h3 className="mb-4">🎭 Choose Your Role</h3>
      
      <div className="row g-4">
        {Object.entries(roleDescriptions).map(([role, info]) => (
          <div key={role} className="col-lg-4">
            <div 
              className={`card h-100 border-3 ${
                selectedRole === role ? 'border-dark bg-light' : 'border-secondary'
              }`}
              style={{ cursor: 'pointer' }}
              onClick={() => setSelectedRole(role as 'None' | 'Staker' | 'Patron')}
            >
              <div className="card-body text-center">
                <div className="fs-1 mb-3">{info.icon}</div>
                <h5 className="card-title">{info.title}</h5>
                <p className="card-text text-muted">{info.description}</p>
                
                <div className="mt-3">
                  <h6>Benefits:</h6>
                  <ul className="list-unstyled">
                    {info.benefits.map((benefit, index) => (
                      <li key={index} className="mb-1">
                        <small>✓ {benefit}</small>
                      </li>
                    ))}
                  </ul>
                </div>
                
                {selectedRole === role && (
                  <div className="mt-3">
                    <span className="badge bg-success">Selected</span>
                  </div>
                )}
                
                {userRole.role === role && (
                  <div className="mt-2">
                    <span className="badge bg-info">Current Role</span>
                  </div>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Role Selection Actions */}
      <div className="mt-4 text-center">
        {selectedRole !== userRole.role && (
          <div className="alert alert-warning">
            <strong>Warning:</strong> Changing roles may require token locking and have associated costs.
            {selectedRole !== 'None' && (
              <div className="mt-2">
                <small>This role requires a commitment period where tokens will be locked.</small>
              </div>
            )}
          </div>
        )}
        
        <button
          className={`btn btn-${roleDescriptions[selectedRole].color} btn-lg px-5`}
          onClick={handleRoleSelect}
          disabled={loading || selectedRole === userRole.role}
        >
          {loading ? (
            <>
              <span className="spinner-border spinner-border-sm me-2" role="status" />
              Processing...
            </>
          ) : selectedRole === userRole.role ? (
            'Current Role'
          ) : (
            `Select ${roleDescriptions[selectedRole].title} Role`
          )}
        </button>
      </div>

      {/* Additional Information */}
      <div className="mt-4">
        <div className="card border-info">
          <div className="card-body">
            <h6 className="card-title">📋 Important Information</h6>
            <ul className="mb-0">
              <li>Role changes may require blockchain transactions and gas fees</li>
              <li>Staker and Patron roles require token locking for specified periods</li>
              <li>Locked tokens cannot be transferred or sold during the lock period</li>
              <li>Higher roles unlock additional features and better rewards</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}

export default RoleSelection;
