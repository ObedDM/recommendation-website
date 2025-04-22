import { useState } from "react"
import GradientButton from "../../Other/customButtons/GradientButton";

interface LoginPanelProps {
    buttonLogIn: string[]
    buttonCreateAccount: string[]
    onSwitch: () => void;
}

export default function LoginPanel({ buttonLogIn, buttonCreateAccount, onSwitch }: LoginPanelProps) {
    
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');

    async function handleLogin() {
        try {
            const response = await fetch("http://localhost:5050/auth/login", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    "username": username,
                    "password": password
                })
            });

            const result = await response.json();

            if (response.ok) {
                console.log(result.message)
            } else {
                console.warn(result.message)
            }
        } catch (err) {
            console.error("Error logging in:", err)
        }
    }

    function getUsername(e: React.ChangeEvent<HTMLInputElement>) {
        setUsername(e.target.value);
    }

    function getPassword(e: React.ChangeEvent<HTMLInputElement>) {
        setPassword(e.target.value);
    }
    
    return (
        <div className="absolute top-1/2 right-1/8 transform -translate-y-1/2">
            <div className="bg-white w-[510px] h-[540px] px-15 py-11 rounded-[20px] shadow-[0_5px_20px_0_rgba(0,0,0,0.25)]">
                <h1 className="font-medium text-black text-[50px] mb-4"> Log in </h1>
                
                <label className="text-[20px]"> Username </label>
                <input
                    type="text"
                    className="text-[20px] border rounded-[5px] px-1 pb-1 mt-1 mb-4 w-full focus:outline-none focus:ring-[1px] focus:ring-black shadow-[0_2px_4px_0_rgba(0,0,0,.15)]"
                    onChange={getUsername}/>

                <label className="text-[20px]"> Password </label>
                <input
                    type="password"
                    className="text-[20px] border rounded-[5px] px-1 pb-1 mt-1 mb-1 w-full focus:outline-none focus:ring-[1px] focus:ring-black shadow-[0_2px_4px_0_rgba(0,0,0,.15)]"
                    onChange={getPassword}/>
                
                <a className="text-[17px] text-[#A0A0A0] hover:text-[#808080] hover:underline">Forgot your password?</a>
            
                {/* Log in button */}
                <GradientButton
                    text={buttonLogIn[0]}
                    gradientClass={buttonLogIn[1]}
                    onClick={handleLogin}/>

                <div className="flex items-center mt-4">
                    <hr className="grow border-[#A0A0A0]" />
                        <span className="font-medium text-[#A0A0A0] p-4"> New here? </span>
                    <hr className="grow border-[#A0A0A0]" />
                </div>

                {/* Create Account button */}
                <div className="flex justify-center mt-4">
                    <GradientButton
                        text={buttonCreateAccount[0]}
                        gradientClass={buttonCreateAccount[1]}
                        onClick={onSwitch}/>
                </div>
            </div>
        </div>
    )
}