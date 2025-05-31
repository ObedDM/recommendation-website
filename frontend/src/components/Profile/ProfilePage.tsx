import { useState } from "react";
import DeleteAccount from "./DeleteAccount";

interface ProfilePageProps {
    username: string
}

export default function ProfilePage({ username }: ProfilePageProps) {
    const [showDelete, setShowDelete] = useState(false);

    function showDeletePanel() {
        setShowDelete(true);
    }

    return (
        <div className="min-h-screen w-screen bg-white flex">
            <div className="bg-[#D9D9D9] w-[260px] p-4 space-y-2 text-sm text-gray-600">
                <input
                    type="text"
                    placeholder="Search"
                    className="w-full p-1 mb-4 border rounded text-sm" />
          

                <button onClick={showDeletePanel}>
                    Delete Account
                </button>
            </div>

            <div className="flex-1 flex flex-col p-8">
                <div className="h-[80px] bg-gradient-to-r from-blue-400 to-purple-500 rounded-md mb-8"></div>

                <h1 className="text-4xl font-bold"> My Profile </h1>

                <div className="relative bg-[#D9D9D9] w-[1595px] flex-1 rounded-lg mt-10 p-8">
                    <div className="mb-4">
                        <label className="block text-[17px] font-semibold mb-1"> Username </label>
                        <div className="flex items-center gap-2">
                            <input
                                type="text"
                                className="border rounded px-2 py-1 text-sm"
                                placeholder={username}
                            />
                            <button className="px-3 py-1 text-white rounded text-sm bg-gradient-to-r from-blue-500 to-purple-500">
                                Edit
                            </button>
                        </div>
                    </div>

                    <img
                        src="bills.jpg"
                        className="absolute -top-20 right-8 w-78 h-78 rounded-full border-10 border-[#D9D9D9] object-cover"
                        alt="Profile"
                    />
                </div>
            </div>

            {showDelete ? <DeleteAccount /> : null}
        </div>
    );
}
