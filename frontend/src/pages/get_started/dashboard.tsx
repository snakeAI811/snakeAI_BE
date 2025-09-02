import React from 'react';
import { ReactComponent as IconLeftLogo } from "../../svgs/logo-left.svg";
import { ReactComponent as ScrollDownIcon } from '../../svgs/scroll-down.svg';
// import './DashboardPage.css'; // ⬅️ Import the CSS file with animation

function DashboardPage() {
    return (
        <div className="border border-5 border-black p-3 d-flex justify-content-center align-items-center" style={{ minHeight: "84vh" }}>
            <div className="w-100">
                <div className="d-flex flex-column flex-md-row justify-content-center align-items-center gap-2 mb-4 w-100 mb-2">
                    <IconLeftLogo />
                    <span className="fs-1 fs-lg-2 fs-xl-1 fw-bold">Snake AI</span>
                </div>
                <div className="d-flex flex-column flex-md-row justify-content-center align-items-center gap-4 mb-2">
                    <p className="fs-5 fs-lg-8 fs-xl-9 fw-bold text-center w-75" style={{ lineHeight: 'normal' }}>
                        Meme-powered. Nostalgia-driven. Deflationary.
                    </p>
                </div>
                <div className="d-flex flex-column flex-md-row justify-content-center align-items-center gap-4 mb-2">
                    <p className="fs-6 fs-lg-8 fs-xl-9 text-center w-75" style={{ lineHeight: 'normal' }}>
                        Welcome to the future of crypto engagement—where meme culture meets blockchain innovation.
                    </p>
                </div>
                <div className="d-flex flex-column flex-md-row justify-content-center align-items-center gap-2 mb-2">
                    <ScrollDownIcon className="scroll-down-icon" style={{ marginTop: '16px', width: '65px' }} />
                    <span>Scroll Down</span>
                </div>
            </div>
        </div>
    );
}

export default DashboardPage;
