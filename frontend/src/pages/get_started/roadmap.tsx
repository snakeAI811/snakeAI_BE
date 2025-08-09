import { ReactComponent as RoadMapIcon } from "../../svgs/roadmap.svg";

function RoadmapPage() {
    return (
        <div className="w-100 border border-5 border-black p-3" style={{ minHeight: "84vh" }}>
            <div className="text-uppercase text-black fs-1 fs-lg-4 fs-xl-4 fw-bolder pt-3 pb-2 px-3">
                ROADMAP
            </div>

            <div className="d-flex justify-content-center align-items-center pt-lg-5">
                <div className="overflow-auto">
                    <RoadMapIcon style={{width: '80vw', minWidth: '1000px', height: '100%'}} />
                </div>
            </div>
        </div>
    );
}

export default RoadmapPage;
