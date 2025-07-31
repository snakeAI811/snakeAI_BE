
function RoadmapPage() {
    return (
        <div className="w-100 border border-5 border-black w-100 p-3" style={{ minHeight: "84vh" }}>
            <div className="text-uppercase text-black fs-1 fs-lg-4 fs-xl-4 fw-bolder pt-3 pb-2 px-3">ROADMAP</div>
            <div className="d-flex justify-content-center align-items-center pt-lg-5">
                <div className="roadmap-container grid-4 gap-2 gap-lg-3">
                    <div className="phase-container position-relative mb-5">
                        <div className="w-90 d-flex justify-content-center align-items-center position-absolute top-0 start-50 translate-middle">
                            <div className="phase-title">PHASE 1</div>
                        </div>
                        <div className="phase-box text-center border border-5 border-black">
                            Token Creation & Simplified Mining<br />
                            → Tweet to mine. No minting yet. we track balances off-chain.
                        </div>
                    </div>
                    <div className="phase-container position-relative mb-5">
                        <div className="w-90 d-flex justify-content-center align-items-center position-absolute top-0 start-50 translate-middle">
                            <div className="phase-title">PHASE 2</div>
                        </div>
                        <div className="phase-box text-center border border-5 border-black">
                            Token Claim Event (TCE)
                            → We hit 1M tweets. Users claim tokens and choose their role: Seller, Staker, or Patron.
                        </div>
                    </div>
                    <div className="phase-container position-relative mb-5">
                        <div className="w-90 d-flex justify-content-center align-items-center position-absolute top-0 start-50 translate-middle">
                            <div className="phase-title">PHASE 3</div>
                        </div>
                        <div className="phase-box text-center border border-5 border-black">
                            Staking & Phase 2 Mining
                            → Stakers earn yield. Patrons mine with shorter cooldowns and qualify for DAO roles.
                        </div>
                    </div>
                    <div className="phase-container position-relative mb-5">
                        <div className="w-90 d-flex justify-content-center align-items-center position-absolute top-0 start-50 translate-middle">
                            <div className="phase-title">PHASE 4</div>
                        </div>
                        <div className="phase-box text-center border border-5 border-black">
                            Peer-to-Peer Price Discovery
                            → OTC-style trades between Sellers, Stakers & Patrons. Liquidity stays in the system.
                        </div>
                    </div>
                    <div className="phase-container position-relative mb-5">
                        <div className="w-90 d-flex justify-content-center align-items-center position-absolute top-0 start-50 translate-middle">
                            <div className="phase-title">PHASE 5</div>
                        </div>
                        <div className="phase-box text-center border border-5 border-black">
                            DAO Governance & Game Studio
                            → Patrons take DAO seats. The community shapes future games & mechanics.
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default RoadmapPage;
