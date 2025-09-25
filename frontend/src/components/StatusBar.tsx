import { useAuth } from "../contexts/AuthContext";

interface StatusBarProps {
    title: string;
}

function StatusBar({ title }: StatusBarProps) {
    const { user, logout } = useAuth();

    return (
        <div className="w-100 py-1">
            <div className="d-flex justify-content-between align-items-center">
                <div className="fs-1" style={{ lineHeight: 'normal' }}>{title}</div>
                <div className="text-end d-flex align-items-center gap-2">
                    <div className="fs-6 text-muted loggedin-status">
                        Connected: @{user?.twitter_username || 'Not authenticated'}
                    </div>
                    <button
                        onClick={async () => {
                            await logout();
                        }}
                        className="fs-6 fw-bold second-btn py-1 px-2 text-decoration-none text-center">
                        LOGOUT
                    </button>
                </div>
            </div>
        </div>
    );
}
export default StatusBar;
