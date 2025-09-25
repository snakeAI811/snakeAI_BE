
import { ReactComponent as IconCup } from '../../../svgs/cup.svg';
import { ReactComponent as IconStar } from '../../../svgs/star.svg';
import { ReactComponent as IconUsers } from '../../../svgs/users.svg';

function WhyJoinPage() {
    return (
        <div className="w-100 border border-5 border-black w-100 p-2" style={{ minHeight: "84vh", backgroundColor: '#131200' }}>
            <div className="text-uppercase fs-1 fs-lg-6 fs-xl-5 fw-bolder p-3 mb-5" style={{ lineHeight: 'normal', color: '#A9E000' }}>WHY JOIN Snake AI?</div>
            <div className="row p-3">
                <div className="col-md-4 mb-4">
                    <div className='d-flex justify-content-center'>
                        <IconCup />
                    </div>
                    <div className="px-1 py-1">
                        <hr className="border border-dashed border-white border-1"></hr>
                    </div>
                    <span className="fs-4 fs-lg-9 fs-xl-10 fw-bolder" style={{ lineHeight: 'normal', color: '#A9E000' }}>Mine Early, Earn More:</span>
                    <div className="px-1 py-1">
                        <hr className="border border-dashed border-white border-1"></hr>
                    </div>
                    <span className="fs-5 fs-lg-11 fs-xl-12" style={{ lineHeight: 'normal', color: 'white' }}>Phase 1 miners get the highest rewards — it pays to be early.</span>
                </div>
                <div className="col-md-4 mb-4">
                    <div className='d-flex justify-content-center'>
                        <IconStar />
                    </div>
                    <div className="px-1 py-1">
                        <hr className="border border-dashed border-white border-1"></hr>
                    </div>
                    <span className="fs-4 fs-lg-9 fs-xl-10 fw-bolder" style={{ lineHeight: 'normal', color: '#A9E000' }}>Scarcity Drives Value: </span>
                    <div className="px-1 py-1">
                        <hr className="border border-dashed border-white border-1"></hr>
                    </div>
                    <span className="fs-5 fs-lg-11 fs-xl-12" style={{ lineHeight: 'normal', color: 'white' }}>Deflationary model + fixed supply = growing value over time.</span>
                </div>
                <div className="col-md-4 mb-4">
                    <div className='d-flex justify-content-center'>
                        <IconUsers />
                    </div>
                    <div className="px-1 py-1">
                        <hr className="border border-dashed border-white border-1"></hr>
                    </div>
                    <span className="fs-4 fs-lg-9 fs-xl-10 fw-bolder" style={{ lineHeight: 'normal', color: '#A9E000' }}>Power to the Players: </span>
                    <div className="px-1 py-1">
                        <hr className="border border-dashed border-white border-1"></hr>
                    </div>
                    <span className="fs-5 fs-lg-11 fs-xl-12" style={{ lineHeight: 'normal', color: 'white' }}> Holders shape the future: govern the DAO, fuel the game studio.</span>
                </div>
            </div>
        </div>
    );
}

export default WhyJoinPage;
