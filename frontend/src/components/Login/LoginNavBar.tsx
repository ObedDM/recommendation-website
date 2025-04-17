import React from "react";

interface CreateNavBarProps {
    navItems: string[];
}

export default function LoginNavBar({ navItems }: CreateNavBarProps) {
    
    function dropdown(item: string) {
        if ((item === 'PRICING') || (item === 'CONTACT')) {
            return true
        }

        return false
    }

    return (
        <nav className="h-[2.5rem] bg-white flex items-center justify-between px-4">
            <div className="flex items-center gap-2">
                <img src="/Navbar Logo.png" className="w-8 h-auto h-full"/>
                <span className="text-[#888888] text-[1.2rem]">CHECK’D © 2025</span>
            </div>
            
            <ul className="flex items-center space-x-4">
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
        </nav>
    )
}