
import { useState, useEffect, useMemo } from "react";

import ResponsiveMenu from "../../components/ResponsiveMenu";
import { usePhantom } from "../../hooks/usePhantom";
import { daoApi } from "../patron/services/apiService";
import StatusBar from "../../components/StatusBar";
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

function Leaderboard() {
    // eslint-disable-next-line no-empty-pattern
    const { } = usePhantom();
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
                    setUsers([]);
                }
            } catch (error) {
                console.error('Error fetching users:', error);
                // Fallback to mock data
                setUsers([]);
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
                    <StatusBar title="Leaderboard" />


                    <div className="custom-border-y custom-content-height d-flex flex-column ">
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
                                            <td className="align-middle ">Miner</td>
                                            <td className="align-middle text-center">{user.score}</td>
                                            <td className="align-middle text-center">{user.roleDuration} days</td>
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

export default Leaderboard;
