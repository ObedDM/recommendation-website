import { Link } from "react-router-dom";

export default function NotFound() {
    return (
        <div className="bg-white">
            <h1 className="text-pink-500"> This page does not exist! </h1>
            <Link to={"/"}>
                <button className="border rounded-md p-1 text-blue-500"> Go back </button>
            </Link>
        </div>
    );
};