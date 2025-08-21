import { ReactComponent as SellerIcon } from "../../svgs/role-seller.svg";
import { ReactComponent as StakerIcon } from "../../svgs/role-staker.svg";
import { ReactComponent as PatronIcon } from "../../svgs/role-patron.svg";

function UserRolePage() {
    return (
        <div className="w-100 border border-5 border-black w-100 p-3" style={{ minHeight: "84vh" }}>
            <div className=" flex-wrap fs-3 fs-lg-5 fs-xl-6 fs-xxl-7 py-2 py-lg-4">
                <div className="text-center" style={{ lineHeight: 'normal' }}>THREE ROLES DEFINE YOUR</div>
                <div className="text-center" style={{ lineHeight: 'normal' }}>JOURNEY AT TCE</div>
            </div>
            <div className="d-flex justify-content-center px-1 px-lg-5 px-xl-5">
                <div className="role-bundle" >
                    <div className="role-card border border-0 box-margin-bottom px-3"  >
                        <div className="border border-5 border-black">
                            <div className="">
                                <div className="px-3 role-icon-bg" >
                                    <SellerIcon />
                                </div>
                                <h2 className="fw-bolder px-3 fs-3 fs-lg-10" style={{ lineHeight: 'normal' }}>SELLERS</h2>
                                <p className="fs-7 fs-lg-12 p-3" style={{ lineHeight: 'normal' }}> Exit at TCE. You earned it.</p>
                            </div>
                        </div>
                    </div>
                    <div className="role-card border border-0 box-margin-bottom px-3">
                        <div className="border border-5 border-black">
                            <div className="">
                                <div className="px-3 role-icon-bg" >
                                    <StakerIcon />
                                </div>
                                <h2 className="fw-bolder px-3 fs-3 fs-lg-10" style={{ lineHeight: 'normal' }}>STAKERES</h2>
                                <p className="fs-7 fs-lg-12 p-3" style={{ lineHeight: 'normal' }}> Lock your tokens for 3 months. Earn APY and joining mining phase 2.</p>
                            </div>
                        </div>
                    </div>
                    <div className="role-card border border-0 box-margin-bottom px-3">
                        <div className="border border-5 border-black">
                            <div className="">
                                <div className="px-3 role-icon-bg" >
                                    <PatronIcon />
                                </div>
                                <h2 className="fw-bolder px-3 fs-3 fs-lg-10" style={{ lineHeight: 'normal' }}>PATRONS</h2>
                                <p className="fs-7 fs-lg-12 p-3" style={{ lineHeight: 'normal' }}> Stake + Govern. Get DAO power and help shape the future.</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default UserRolePage;
