import { Link } from "react-router-dom";

export default function Unauthorized() {
    return (
        <div className="bg-white">
            <h1 className="text-pink-500"> You are not authorized to see this page. Please log in </h1>
            <Link to={"/auth?login=true"}>
                <button className="border rounded-md p-1 text-blue-500"> Log in </button>
            </Link>
        </div>
    )
}