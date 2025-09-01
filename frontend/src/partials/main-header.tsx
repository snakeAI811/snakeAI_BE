import { ReactComponent as IconSmallLogo } from "../svgs/logo-small.svg";
import { useAuth } from "../contexts/AuthContext";
import { useNavigate } from "react-router-dom";
import {API_BASE_URL} from "../config/program";

function MainHeader() {
    const { isAuthenticated, user, logout } = useAuth();
    const navigate = useNavigate();

    return (
        <>
            <div style={{ height: '10vh' }}></div>
            <div className="main-header d-flex fixed-top justify-content-between align-items-center border-bottom border-5 border-black bg-black w-100 pt-2 px-2 px-md-4 px-lg-4 shadow-bottom" style={{ height: '10vh' }}>
                <div className="d-flex justify-content-start align-items-center gap-2">
                    <IconSmallLogo className="fs-1" />
                    <span className="fs-1 fs-lg-7 fs-xl-7 text-white main-header-title">SNAKE.AI</span>
                </div>
                <div className="d-flex justify-content-end align-items-center gap-3 h-100 px-2">
                    {isAuthenticated ? (
                        <div className="d-flex align-items-center gap-2">
                            {/* <span className="text-white">Welcome, {user?.twitter_username || 'User'}!</span> */}
                            <button
                                onClick={() => navigate('/home')}
                                className="fs-5 fw-bold primary-btn text-decoration-none text-center" >
                                DashBoard
                            </button>
                            <button
                                onClick={async () => {
                                    await logout();
                                    navigate('/');
                                }}
                                className="fs-5 fw-bold second-btn text-decoration-none text-center">
                                LOGOUT
                            </button>
                        </div>
                    ) : (
                        <div className="d-flex gap-3">
                            <a href={`${API_BASE_URL}/login`} className="border border-0 fs-5 fw-bold py-2 px-3 text-decoration-none text-center" style={{ backgroundColor: "#A9E000", color: "black" }}>Log In</a>
                        </div>
                    )}
                </div>
            </div>
        </>
    );
}

export default MainHeader;
