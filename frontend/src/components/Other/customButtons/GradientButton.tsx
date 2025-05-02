import { Link } from "react-router-dom";

interface GradientButtonProps {
    text: string;
    gradientClass: string;
    route: string | null;
    onClick: () => void;
}

export default function GradientButton({ text, gradientClass, route, onClick, }: GradientButtonProps) {
    const button = (
        <button
            className={`${gradientClass} glow-on-hover text-[18px] text-white tracking-wide pb-[6px] pt-1 px-6 rounded-[20px] shadow-[0_2px_6px_0_rgba(0,0,0,.25)] transition-all duration-300 font-bold w-auto cursor-pointer`}
            onClick={onClick}>
                {text}
        </button>
    );


    
    return (
        <div className="flex justify-end">
            {route ? <Link to={`${route}`}> {button} </Link> : button}
        </div>
    )
}