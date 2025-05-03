import User from "../../types/user";
import Unauthorized from "../../components/Unauthorized/Unauthorized";
import HomePage from "../../components/Home/HomePage";
import useAuth from "../../hooks/useAuth";

export default function HomePath() {
    const { data: user, error, loading } = useAuth<User>("/gethome", {
        method: "GET"
    });

    if (loading) {
        return <div>Loading...</div>
    }

    if (error) {
        console.warn(error);
        return <Unauthorized />
    }

    return (
        <>
            {user ?
                <HomePage 
                    id={user.id}
                    username={user.username}/>
                :
                <Unauthorized />
            }
        </>
    )
}