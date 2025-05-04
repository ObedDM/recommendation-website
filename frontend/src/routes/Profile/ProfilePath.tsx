import ProfilePage from "../../components/Profile/ProfilePage";
import Unauthorized from "../../components/Unauthorized/Unauthorized";
import useAuth from "../../hooks/useAuth";
import User from "../../types/user";

export default function ProfilePath() {
    const { data: user, error, loading } = useAuth<User>("/getprofile", {
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
                <ProfilePage
                    username={user.username}/>
                :
                <Unauthorized />
            }
        </>
    );
}
