import { useState } from "react";
import GradientButton from "../customButtons/GradientButton";
import CountryDropdown from "./CountryDropdown";
import { CountryOption } from "react-select-country-list";

interface SignupPanelProps {
    buttonSignUp: string[]
}

export default function SignupPanel({ buttonSignUp }: SignupPanelProps) {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [email, setEmail] = useState('');
    const [country, setCountry] = useState<CountryOption | null>(null);

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
                console.log(result.message)
            } else {
                console.warn(result.message)
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
                            <a className="text-[17px] text-[#5500FF] hover:font-semibold hover:underline grow cursor-pointer"> Log in </a>
                    
                            {/* Sign up in button */}
                            <GradientButton
                                text={buttonSignUp[0]}
                                gradientClass={buttonSignUp[1]}
                                onClick={handleSignup}/>
                        </div>

                        {/*
                        <div className="flex items-center mt-4">
                            <hr className="grow border-[#A0A0A0]" />
                                <span className="font-medium text-[#A0A0A0] p-4"> New here? </span>
                            <hr className="grow border-[#A0A0A0]" />
                        </div>
                        */}
        
                        {/* Create Account button
                        <div className="flex justify-center mt-4">
                            <GradientButton
                                text={buttonCreateAccount[0]}
                                gradientClass={buttonCreateAccount[1]}
                                onClick={() => {}}/>
                        </div>
                        */}
                    </div>
                </div>
    )
}