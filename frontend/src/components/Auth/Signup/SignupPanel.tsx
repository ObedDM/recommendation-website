import { useState } from "react";
import CountryDropdown from "./CountryDropdown";
import { CountryOption } from "react-select-country-list";
import GradientButton from "../../Other/customButtons/GradientButton";
import { useNavigate } from "react-router-dom";

interface SignupPanelProps {
    buttonSignUp: string[]
    onSwitch: () => void;
}

export default function SignupPanel({ buttonSignUp, onSwitch }: SignupPanelProps) {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [email, setEmail] = useState('');
    const [country, setCountry] = useState<CountryOption | null>(null);
    const navigate = useNavigate();

    async function handleSignup() {

        if (country === null) {
            console.error("country must be set")
            return
        }

        try {
            const response = await fetch("http://127.0.0.1:5050/auth/signup", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    "username": username,
                    "email": email,
                    "password": password,
                    "country": country.value
                }),
            });

            const result = await response.json();

            if (response.ok) {
                console.log(result.message);

                try {
                    const response = await fetch("http://localhost:5050/auth/login", {
                        method: "POST",
                        credentials: "include",
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
                        navigate("/home")

                    } else {
                        console.warn(result.message)
                    }
                    
                } catch (err) {
                    console.error("Error logging in:", err)
                }                
                
            } else {
                console.warn(result.message);
            }

        } catch (err) {
            console.error("Error signing up:", err)
        }
    }

    function getUsername(e: React.ChangeEvent<HTMLInputElement>) {
        setUsername(e.target.value);
    }

    function getPassword(e: React.ChangeEvent<HTMLInputElement>) {
        setPassword(e.target.value);
    }

    function getEmail(e: React.ChangeEvent<HTMLInputElement>) {
        setEmail(e.target.value)
    }

    function getCountry(country: CountryOption | null) {
        setCountry(country)
    }

    return (
                <div className="absolute top-1/2 right-1/8 transform -translate-y-1/2">
                    <div className="bg-white w-[510px] h-[620px] px-15 py-11 rounded-[20px] shadow-[0_5px_20px_0_rgba(0,0,0,0.25)]">
                        <h1 className="font-medium text-black text-[50px] mb-4"> Sign up </h1>
                        
                        <label className="text-[20px]"> Username </label>
                        <input
                            type="text"
                            className="text-[20px] border rounded-[5px] px-1 pb-1 mt-1 mb-4 w-full focus:outline-none focus:ring-[1px] focus:ring-black shadow-[0_2px_4px_0_rgba(0,0,0,.15)]"
                            onChange={getUsername}/>
        
                        <label className="text-[20px]"> Password </label>
                        <input
                            type="password"
                            className="text-[20px] border rounded-[5px] px-1 pb-1 mt-1 mb-4 w-full focus:outline-none focus:ring-[1px] focus:ring-black shadow-[0_2px_4px_0_rgba(0,0,0,.15)]"
                            onChange={getPassword}/>

                        <label className="text-[20px]"> Email address </label>
                        <input
                            type="text"
                            className="text-[20px] border rounded-[5px] px-1 pb-1 mt-1 mb-4 w-full focus:outline-none focus:ring-[1px] focus:ring-black shadow-[0_2px_4px_0_rgba(0,0,0,.15)]"
                            onChange={getEmail}/>       

                        <label className="text-[20px]"> Country </label>                        
                        <div className="text-[19px] w-full mt-1 mb-12 shadow-[0_2px_4px_0_rgba(0,0,0,.15)]">
                            <CountryDropdown 
                                onCountryChange={getCountry}/>
                        </div>
                        
                        <div className="flex items-center">
                            <span className="text-[17px] text-[#A0A0A0] mr-1"> Already have an account? </span>
                            <a onClick={onSwitch}
                                className="text-[17px] text-[#5500FF] hover:font-semibold hover:underline grow cursor-pointer">
                                    Log in
                            </a>
                    
                            {/* Sign up button */}
                            <GradientButton
                                text={buttonSignUp[0]}
                                gradientClass={buttonSignUp[1]}
                                onClick={handleSignup}
                                route={null}/>
                        </div>
                    </div>
                </div>
    )
}