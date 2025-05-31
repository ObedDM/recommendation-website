import { useNavigate } from "react-router-dom";
import GradientButton from "../Other/customButtons/GradientButton";

export default function DeleteAccount() {
    const navigate = useNavigate();

    async function deleteAccount() {
        const response = await fetch("http://localhost:5050/profile", {
            method: "DELETE",
            credentials: "include"
        });

        const result = await response.json();

        if (response.ok) {
            console.log(`success: ${result.message}`)
            navigate("/auth?login=false")
        }

        else {
            console.log(`fail: ${result.message}`)
        }
    }

    return (
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[770px] h-[318px] rounded-[20px] p-6 bg-white shadow-[0_2px_6px_0_rgba(0,0,0,.25)]">
            <div className="flex flex-col items-start gap-4">
                <span className="mb-4">
                    Your account is about to be deleted
                </span>

                <span className="mb-10">
                    All data will be lost and you wont be able to recover it. <br />
                    Are you sure you want to delete your account?
                </span>

                <div className="flex items-center">
                    <GradientButton
                        text="Go Back"
                        gradientClass="custom-login-gradient"
                        route={null}
                        onClick={() => {}}/>
                    
                    <GradientButton
                        text="Delete Account"
                        gradientClass="custom-gradient-delete"
                        route={null}
                        onClick={() => deleteAccount()}/>
                </div>
                {/* Delete & Go back button */}
            </div>
        </div>
    )
}