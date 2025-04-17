interface GradientButtonProps {
    text: string;
    gradientClass: string
}

export default function GradientButton({ text, gradientClass }: GradientButtonProps) {
    return (
        <div className="flex justify-end">
            <button className={ `${gradientClass} glow-on-hover text-[18px] text-white tracking-wide pb-[6px] pt-1 px-6 rounded-[20px] shadow-[0_2px_6px_0_rgba(0,0,0,.25)] transition-all duration-300 font-bold w-auto cursor-pointer` }>
                {text}
            </button>
        </div>
    )
}