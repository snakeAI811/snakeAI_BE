
function HowItWorksPage() {
    return (
        <div className="w-100 border border-5 border-black w-100 p-3" style={{ minHeight: "84vh" }}>
            <div className="d-flex justify-content-center flex-wrap fs-3 fs-lg-4 fs-xl-5 fs-xxl-6 py-2 py-lg-4">
                <div className="fw-bolder text-center" style={{ lineHeight: 'normal' }}>HOW IT WORKS:</div>
                <div className="fw-lighter text-center" style={{ lineHeight: 'normal' }}>TWEET TO MINE</div>
            </div>
            <div className="row px-1 pt-5 pt-lg-5 px-lg-5 px-xl-5">
                <div className="col-sm-6 col-xl-3 border border-0 text-center position-relative box-margin-bottom px-3">
                    <span className="fs-1 fs-lg-2 fs-xl-4 px-4 bg-green-950 fw-bolder position-absolute top-0 start-50 translate-middle" style={{ color: '#A9E000' }}>1</span>
                    <div className="border border-5 border-black pt-3 pt-lg-5 box-height-48vh">
                        <div className="pt-4">
                            <h2 className="fw-bolder fs-3 fs-lg-10" style={{ lineHeight: 'normal' }}>TWEET</h2>

                            <div className="px-3">
                                <hr className="border border-dashed border-black border-3 opacity-100"></hr>
                            </div>
                            <p className="fs-4 fs-lg-10 p-3 px-lg-4" style={{ lineHeight: 'normal' }}>Post on X using #MineTheSnake and mention @playSnakeAI.</p>
                        </div>
                    </div>
                </div>
                <div className="col-sm-6 col-xl-3 border border-0 text-center position-relative box-margin-bottom px-3">
                    <span className="fs-1 fs-lg-2 fs-xl-4 px-4 bg-green-950 fw-bolder position-absolute top-0 start-50 translate-middle" style={{ color: '#A9E000' }}>2</span>
                    <div className="border border-5 border-black pt-3 pt-lg-5 box-height-48vh">
                        <div className="pt-4">
                            <h2 className="fw-bolder fs-3 fs-lg-10" style={{ lineHeight: 'normal' }}>GET LOGGED:</h2>
                            <div className="px-3">
                                <hr className="border border-dashed border-black border-3 opacity-100"></hr>
                            </div>
                            <p className="fs-4 fs-lg-10 p-3 px-lg-4" style={{ lineHeight: 'normal' }}>Our system detects it and logs your activity in the mining ledger.</p>
                        </div>
                    </div>
                </div>
                <div className="col-sm-6 col-xl-3 border border-0 text-center position-relative box-margin-bottom px-3">
                    <span className="fs-1 fs-lg-2 fs-xl-4 px-4 bg-green-950 fw-bolder position-absolute top-0 start-50 translate-middle" style={{ color: '#A9E000' }}>3</span>
                    <div className="border border-5 border-black pt-3 pt-lg-5 box-height-48vh">
                        <div className="pt-4">
                            <h2 className="fw-bolder fs-3 fs-lg-10" style={{ lineHeight: 'normal' }}>NO MINT YET:</h2>
                            <div className="px-3">
                                <hr className="border border-dashed border-black border-3 opacity-100"></hr>
                            </div>
                            <p className="fs-4 fs-lg-10 p-3 px-lg-4" style={{ lineHeight: 'normal' }}>No tokens are minted until TCE (Token Claim Event).</p>
                        </div>
                    </div>
                </div>
                <div className="col-sm-6 col-xl-3 border border-0 text-center position-relative box-margin-bottom px-3">
                    <span className="fs-1 fs-lg-2 fs-xl-4 px-4 bg-green-950 fw-bolder position-absolute top-0 start-50 translate-middle" style={{ color: '#A9E000' }}>4</span>
                    <div className="border border-5 border-black pt-3 pt-lg-5 box-height-48vh">
                        <div className="pt-4">
                            <h2 className="fw-bolder fs-3 fs-lg-10" style={{ lineHeight: 'normal' }}>CLAIM Y CHOOSE:</h2>
                            <div className="px-3">
                                <hr className="border border-dashed border-black border-3 opacity-100"></hr>
                            </div>
                            <p className="fs-4 fs-lg-10 p-3 px-lg-4" style={{ lineHeight: 'normal' }}>At TCE, you choose: Sell, Stake, or become a Patron.</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default HowItWorksPage;
