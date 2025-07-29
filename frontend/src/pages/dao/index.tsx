
import { useState, useEffect, useMemo } from "react";

import ResponsiveMenu from "../../components/ResponsiveMenu";
import { useAuth } from "../../contexts/AuthContext";
import { usePhantom } from "../../hooks/usePhantom";
import { daoApi } from "../patron/services/apiService";
import './index.css';

interface ProfileData {
    twitter_username: string;
    wallet_address: string;
    latest_claim_timestamp: string | null;
    reward_balance: number;
    tweets: number;
    likes: number;
    replies: number;
}

interface UserTableData {
    id: string;
    userIcon?: string; // Keep for backwards compatibility
    avatar?: string | null;
    username: string;
    address: string;
    score: number;
    roleDuration: number;
    activity: number;
}

function DAO() {
    // eslint-disable-next-line no-empty-pattern
    const { } = usePhantom();
    const { user } = useAuth();
    const [profileData, setProfileData] = useState<ProfileData | null>(null);
    const [sortBy, setSortBy] = useState<'activity' | 'address' | 'roleDuration' | 'score'>('score');
    const [showDropdown, setShowDropdown] = useState(false);
    const [searchTerm, setSearchTerm] = useState('');
    const [users, setUsers] = useState<UserTableData[]>([]);
    const [loading, setLoading] = useState(true);

    // Fetch DAO users from backend
    useEffect(() => {
        const fetchUsers = async () => {
            setLoading(true);
            try {
                const response = await daoApi.getDaoUsers(searchTerm || undefined, sortBy);
                if (response.success && response.data) {
                    // Transform backend data to match frontend interface
                    const transformedUsers = response.data.map(user => ({
                        id: user.id,
                        avatar: user.avatar || null,
                        username: user.username,
                        address: user.wallet_address,
                        score: user.score,
                        roleDuration: user.role_duration,
                        activity: user.activity
                    }));
                    setUsers(transformedUsers);
                } else {
                    console.error('Failed to fetch users:', response.error);
                    // Fallback to mock data if API fails
                    setUsers([
                        { id: '1', avatar: 'https://avatars.githubusercontent.com/u/1?v=4', username: 'alice_trader', address: '0x1234...5678', score: 950, roleDuration: 180, activity: 85 },
                        { id: '2', avatar: null, username: 'crypto_master', address: '0xabcd...efgh', score: 820, roleDuration: 90, activity: 92 },
                        { id: '3', avatar: 'https://avatars.githubusercontent.com/u/3?v=4', username: 'moon_walker', address: '0x9876...5432', score: 1200, roleDuration: 365, activity: 78 },
                        { id: '4', avatar: null, username: 'dao_voter', address: '0xdef0...1234', score: 650, roleDuration: 60, activity: 95 },
                        { id: '5', avatar: 'https://avatars.githubusercontent.com/u/5?v=4', username: 'yield_farmer', address: '0x5678...abcd', score: 1050, roleDuration: 240, activity: 88 },
                    ]);
                }
            } catch (error) {
                console.error('Error fetching users:', error);
                // Fallback to mock data
                setUsers([
                    { id: '1', avatar: 'https://avatars.githubusercontent.com/u/1?v=4', username: 'alice_trader', address: '0x1234...5678', score: 950, roleDuration: 180, activity: 85 },
                    { id: '2', avatar: null, username: 'crypto_master', address: '0xabcd...efgh', score: 820, roleDuration: 90, activity: 92 },
                    { id: '3', avatar: 'https://avatars.githubusercontent.com/u/3?v=4', username: 'moon_walker', address: '0x9876...5432', score: 1200, roleDuration: 365, activity: 78 },
                    { id: '4', avatar: null, username: 'dao_voter', address: '0xdef0...1234', score: 650, roleDuration: 60, activity: 95 },
                    { id: '5', avatar: 'https://avatars.githubusercontent.com/u/5?v=4', username: 'yield_farmer', address: '0x5678...abcd', score: 1050, roleDuration: 240, activity: 88 },
                ]);
            } finally {
                setLoading(false);
            }
        };

        // Only debounce search term changes, not sorting
        if (searchTerm !== '') {
            const timeoutId = setTimeout(fetchUsers, 300);
            return () => clearTimeout(timeoutId);
        } else {
            fetchUsers();
        }
    }, [searchTerm, sortBy]);

    // Memoize the user list to prevent unnecessary re-renders
    const displayUsers = useMemo(() => users, [users]);

    // Memoize the error handler to prevent re-creating the function on every render
    const handleImageError = useMemo(() => (e: React.SyntheticEvent<HTMLImageElement>) => {
        e.currentTarget.src = '/avatars/logo-colored.svg';
    }, []);

    // Memoize dropdown handlers to prevent re-creation
    const handleSortChange = useMemo(() => ({
        score: () => { setSortBy('score'); setShowDropdown(false); },
        activity: () => { setSortBy('activity'); setShowDropdown(false); },
        address: () => { setSortBy('address'); setShowDropdown(false); },
        roleDuration: () => { setSortBy('roleDuration'); setShowDropdown(false); }
    }), []);
   
    return (
        <div className="w-100 p-3" style={{ height: '100vh', overflow: 'auto' }}>
            <div className="d-flex gap-3" style={{ height: "calc(100vh-60px)", paddingTop: '35px' }}>
                {/* Menu Begin */}
                <ResponsiveMenu />
                {/* Menu End */}

                <div className="custom-content">
                    <div className="w-100 d-flex justify-space-between align-items-end">
                        <div className="fs-1" style={{ lineHeight: 'normal' }}>
                            DAO
                        </div>
                        <div className="fs-6 text-muted">
                            Welcome back, {profileData?.twitter_username || user?.twitter_username || 'User'}!
                        </div>
                    </div>

                    <div className="custom-border-y custom-content-height d-flex flex-column px-3">
                        {/* <div className="d-flex justify-content-between align-items-center mb-3 mt-3">
                            <h4>User Rankings</h4>
                        </div> */}

                        <div className="mb-3 pb-2 custom-border-bottom">
                            <div className="input-group searchbar">
                                <input
                                    type="text"
                                    className="form-control"
                                    placeholder="Search by username or address..."
                                    value={searchTerm}
                                    onChange={(e) => setSearchTerm(e.target.value)}
                                />
                                <span className="input-group-text">🔍</span>
                            </div>
                        </div>

                        <div className="d-flex justify-content-between align-items-center mb-3">
                            <span className="text-muted">
                                {loading ? 'Loading...' : `Showing ${displayUsers.length} users`}
                            </span>
                            <div className="position-relative">
                                <button 
                                    className="btn btn-outline-secondary d-flex align-items-center gap-2"
                                    onClick={() => setShowDropdown(!showDropdown)}
                                >
                                    <span>Sort by: {sortBy}</span>
                                </button>
                                {showDropdown && (
                                    <div className="position-absolute top-100 end-0 mt-1 bg-white border rounded shadow" style={{ zIndex: 1000, minWidth: '150px' }}>
                                        <div 
                                            className="px-3 py-2 cursor-pointer hover-bg-light border-bottom"
                                            onClick={handleSortChange.score}
                                        >
                                            Score
                                        </div>
                                        <div 
                                            className="px-3 py-2 cursor-pointer hover-bg-light border-bottom"
                                            onClick={handleSortChange.activity}
                                        >
                                            Activity
                                        </div>
                                        <div 
                                            className="px-3 py-2 cursor-pointer hover-bg-light border-bottom"
                                            onClick={handleSortChange.address}
                                        >
                                            Address
                                        </div>
                                        <div 
                                            className="px-3 py-2 cursor-pointer hover-bg-light"
                                            onClick={handleSortChange.roleDuration}
                                        >
                                            Role Duration
                                        </div>
                                    </div>
                                )}
                            </div>
                        </div>

                        <div className="table-responsive">
                            <table className="table table-borderless">
                                <thead className="pb-2">
                                    <tr>
                                        <th scope="col"></th>
                                        <th scope="col"><span className="pb-2 custom-border-bottom px-2">Address</span></th>
                                        <th scope="col"><span className="pb-2 custom-border-bottom px-2">Status</span></th>
                                        <th scope="col" className="text-center"><span className="pb-2 custom-border-bottom px-2">Score</span></th>
                                        <th scope="col" className="text-center"><span className="pb-2 custom-border-bottom px-2">Role Duration</span></th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {displayUsers.map((user: UserTableData, index: number) => (
                                        <tr key={user.id}>
                                            <td>
                                                <div className="d-flex align-items-center gap-2">
                                                    <img 
                                                        src={user.avatar || '/avatars/logo-colored.svg'} 
                                                        alt={`${user.username} avatar`}
                                                        className="custom-s-avatar"
                                                        onError={handleImageError}
                                                    />
                                                    {/* <span className="text-muted">#{index + 1}</span> */}
                                                </div>
                                            </td>
                                            <td>
                                                <div className="d-flex flex-column">
                                                    <span className="fw-semibold">{user.username}</span>
                                                    <span className="font-monospace text-muted small">{user.address}</span>
                                                </div>
                                            </td>
                                            <td>Miner</td>
                                            <td className="text-center">{user.score}</td>
                                            <td className="text-center">{user.roleDuration} days</td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default DAO;
