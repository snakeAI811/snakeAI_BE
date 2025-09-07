import React from 'react';
import { Link } from 'react-router-dom';

function AdminDashboard() {
  const adminFeatures = [
    {
      title: 'Patron Approvals',
      description: 'Review and approve patron applications from users',
      icon: '👑',
      path: '/admin/patron-approvals',
      color: 'primary'
    },
    {
      title: 'TCE Management',
      description: 'Control the Token Claim Engine flag system-wide',
      icon: '🔧',
      path: '/admin/tce-management',
      color: 'success'
    },
    {
      title: 'Staking History',
      description: 'Validate and manage user staking history records',
      icon: '📊',
      path: '/admin/staking-history',
      color: 'info'
    }
  ];

  return (
    <div className="container-fluid">
      {/* Breadcrumb */}
      <nav aria-label="breadcrumb" className="mb-3">
        <ol className="breadcrumb">
          <li className="breadcrumb-item"><Link to="/home">Home</Link></li>
          <li className="breadcrumb-item active" aria-current="page">Admin Dashboard</li>
        </ol>
      </nav>

      <div className="row">
        <div className="col-12">
          <div className="card">
            <div className="card-header">
              <h4 className="mb-0">🛠️ Admin Dashboard</h4>
              <p className="text-muted mb-0">Administrative tools and system management</p>
            </div>

            <div className="card-body">
              <div className="row">
                {adminFeatures.map((feature, index) => (
                  <div key={index} className="col-md-4 mb-4">
                    <div className="card h-100 border-0 shadow-sm">
                      <div className="card-body text-center">
                        <div className="mb-3">
                          <span style={{ fontSize: '3rem' }}>{feature.icon}</span>
                        </div>
                        <h5 className="card-title">{feature.title}</h5>
                        <p className="card-text text-muted">{feature.description}</p>
                        <Link 
                          to={feature.path} 
                          className={`btn btn-${feature.color} btn-lg w-100`}
                        >
                          Access {feature.title}
                        </Link>
                      </div>
                    </div>
                  </div>
                ))}
              </div>

              <div className="row mt-4">
                <div className="col-12">
                  <div className="alert alert-warning">
                    <h6 className="alert-heading">⚠️ Admin Access Notice</h6>
                    <p className="mb-0">
                      These administrative functions have system-wide impact. Please use with caution and ensure 
                      you have proper authorization before making changes. All actions are logged and may require 
                      wallet signatures for blockchain transactions.
                    </p>
                  </div>
                </div>
              </div>
            </div>

            <div className="card-footer">
              <div className="row">
                <div className="col-md-6">
                  <h6>Quick Actions:</h6>
                  <ul className="small mb-0">
                    <li>Review pending patron applications</li>
                    <li>Monitor TCE system status</li>
                    <li>Validate staking records</li>
                  </ul>
                </div>
                <div className="col-md-6">
                  <h6>System Status:</h6>
                  <ul className="small mb-0">
                    <li>All admin functions operational</li>
                    <li>Blockchain connectivity active</li>
                    <li>User authentication verified</li>
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

export default AdminDashboard;