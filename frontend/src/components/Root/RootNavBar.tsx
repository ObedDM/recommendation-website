import React from "react";
import GradientButton from "../Other/customButtons/GradientButton";

interface RootNavBarProps {
    navItems: string[];
}

export default function RootNavBar({ navItems }: RootNavBarProps) {
    
    function dropdown(item: string) {
        if ((item === 'PRICING') || (item === 'CONTACT')) {
            return true
        }

        return false
    }

    return (
        <nav className="h-[2.5rem] bg-white flex items-center justify-between px-4">
            <ul className="flex items-center space-x-4 grow">
                <img src="/Navbar Logo.png" className="w-8 h-auto h-full"/>

                {navItems.map((item, index) => (
                    <React.Fragment key={index}>
                        {index > 0 && (
                            <li className="text-[#BEBEBE]">|</li>
                        )}
                        
                        <li className="flex items-center gap-2">
                            <span className="text-[#888888] text-[1.2rem]">
                                {item}
                            </span>
                            
                            {dropdown(item) && (
                            <svg
                                width="22"
                                height="22"
                                viewBox="0 0 24 24"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                                stroke="#BEBEBE">
                                <path
                                fillRule="evenodd"
                                clipRule="evenodd"
                                d="M12.7071 14.7071C12.3166 15.0976 11.6834 15.0976 11.2929 14.7071L6.29289 9.70711C5.90237 9.31658 5.90237 8.68342 6.29289 8.29289C6.68342 7.90237 7.31658 7.90237 7.70711 8.29289L12 12.5858L16.2929 8.29289C16.6834 7.90237 17.3166 7.90237 17.7071 8.29289C18.0976 8.68342 18.0976 9.31658 17.7071 9.70711L12.7071 14.7071Z"
                                fill="#BEBEBE"
                                />
                            </svg>
                            )}
                        </li>
                    </React.Fragment>
                ))}
            </ul>

            <div className="flex items-center">
                <svg fill="#888888" width="25px" height="25px" viewBox="-1 0 19 19" xmlns="http://www.w3.org/2000/svg" stroke="#888888" stroke-width="0.152"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"><path d="M16.417 9.57a7.917 7.917 0 1 1-8.144-7.908 1.758 1.758 0 0 1 .451 0 7.913 7.913 0 0 1 7.693 7.907zM5.85 15.838q.254.107.515.193a11.772 11.772 0 0 1-1.572-5.92h-3.08a6.816 6.816 0 0 0 4.137 5.727zM2.226 6.922a6.727 6.727 0 0 0-.511 2.082h3.078a11.83 11.83 0 0 1 1.55-5.89q-.249.083-.493.186a6.834 6.834 0 0 0-3.624 3.622zm8.87 2.082a14.405 14.405 0 0 0-.261-2.31 9.847 9.847 0 0 0-.713-2.26c-.447-.952-1.009-1.573-1.497-1.667a8.468 8.468 0 0 0-.253 0c-.488.094-1.05.715-1.497 1.668a9.847 9.847 0 0 0-.712 2.26 14.404 14.404 0 0 0-.261 2.309zm-.974 5.676a9.844 9.844 0 0 0 .713-2.26 14.413 14.413 0 0 0 .26-2.309H5.903a14.412 14.412 0 0 0 .261 2.31 9.844 9.844 0 0 0 .712 2.259c.487 1.036 1.109 1.68 1.624 1.68s1.137-.644 1.623-1.68zm4.652-2.462a6.737 6.737 0 0 0 .513-2.107h-3.082a11.77 11.77 0 0 1-1.572 5.922q.261-.086.517-.194a6.834 6.834 0 0 0 3.624-3.621zM11.15 3.3a6.82 6.82 0 0 0-.496-.187 11.828 11.828 0 0 1 1.55 5.89h3.081A6.815 6.815 0 0 0 11.15 3.3z"></path></g></svg>
                <svg width="22" height="22" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="#BEBEBE"> <path fillRule="evenodd" clipRule="evenodd" d="M12.7071 14.7071C12.3166 15.0976 11.6834 15.0976 11.2929 14.7071L6.29289 9.70711C5.90237 9.31658 5.90237 8.68342 6.29289 8.29289C6.68342 7.90237 7.31658 7.90237 7.70711 8.29289L12 12.5858L16.2929 8.29289C16.6834 7.90237 17.3166 7.90237 17.7071 8.29289C18.0976 8.68342 18.0976 9.31658 17.7071 9.70711L12.7071 14.7071Z" fill="#BEBEBE"/> </svg>
                <div className="mx-3">
                    <GradientButton
                        text="Sign up"
                        gradientClass="custom-gradient-none"
                        route={"/Auth?login=false"}
                        onClick={() => {}}/>
                </div>
                <GradientButton
                    text="Log in"
                    gradientClass="custom-login-gradient"
                    route={"/Auth?login=true"}
                    onClick={() => {}}/>
            </div>
        </nav>
    )
}