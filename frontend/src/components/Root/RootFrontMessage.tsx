import GradientButton from "../Other/customButtons/GradientButton";

export default function RootFrontMessage() {
    return (
        <div className="absolute top-1/2 right-1/4 transform -translate-y-1/2">
            <div className="flex justify-center mb-15">
                <h1 className="text-7xl text-white">
                    Your <span className="custom-login-gradient"> all-in-one </span> <br />
                    recommendation engine!
                </h1>
            </div>
            
            <div className="flex justify-center mt-15">
                <h2 className="text-2xl text-white">
                    Discover your next favorite <span className="custom-login-gradient"> game </span> , tailored <br />
                    to your taste!
                </h2>
            </div>

            <div className="flex justify-center mt-10">
                <GradientButton
                    text="Get started!"
                    gradientClass="custom-gradient-none"
                    route={"/auth?login=false"}
                    onClick={() => {}}/>
            </div>
        </div>
    )
}