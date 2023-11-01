import { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import apiClient from "../services/api-common";
import { useMutation } from "react-query";

interface SignInResponse {
    id: string;
    username: string;
    email: string;
    steam_id?: string;
    psn_auth_code?: string;
}

function UserProfile() {
    const [data, setData] = useState<SignInResponse | undefined>(undefined);
    const [token, setToken] = useState(localStorage.getItem("token"));
    const nav = useNavigate();

    const { isLoading: isPostingUser, mutate: postUser } = useMutation(
        async () => {
            if (data) {
                // Proceed with the API request
                return await apiClient.post(
                    "users",
                    {
                        user_id: data.id,
                        username: data.username,
                    },
                    {
                        headers: {
                            "axum-accountId": data.id,
                            Authorization: token,
                        },
                    },
                );
            } else {
                console.log("Data is empty or missing required fields", data);
            }
        },
        {
            onSuccess: (res) => {
                localStorage.setItem("user1", JSON.stringify(res?.data));
            },
            onError: (err) => {
                console.log(err);
            },
        },
    );

    useEffect(() => {
        setTimeout(() => {
            const localStorageUser = localStorage.getItem("user");
            const localStorageToken = localStorage.getItem("token");
            if (localStorageUser !== null && localStorageToken !== null) {
                const userData = JSON.parse(localStorageUser);
                setData(userData);
                setToken(localStorageToken);
            } else {
                nav("/");
            }
        }, 5000);

        try {
            postUser();
        } catch (err) {
            console.log(err);
        }
    }, []);

    const logout = () => {
        setData(undefined);
        localStorage.removeItem("user");
        localStorage.removeItem("token");

        nav("/");
    };

    return (
        <div className="flex items-center h-screen w-full justify-center">
            <div className="max-w-xs">
                <div className="bg-white shadow-xl rounded-lg py-3">
                    <div className="photo-wrapper p-2">
                        <img
                            className="w-32 h-32 rounded-full mx-auto"
                            src="../../images/acastro_210113_1777_gamingstock_0002.jpg"
                            alt="TestUser"
                        ></img>
                    </div>
                    <div className="p-2">
                        <h3 className="text-center text-xl text-gray-900 font-medium leading-8">
                            {data?.username !== null ? data?.username : "Null"}
                        </h3>
                        <table className="text-xs my-3">
                            <tbody>
                                <tr>
                                    <td className="px-2 py-2 text-gray-500 font-semibold">
                                        Email
                                    </td>
                                    <td className="px-2 py-2">
                                        {data?.email !== null ? data?.email : "Null"}
                                    </td>
                                </tr>
                                <tr>
                                    <td className="px-2 py-2 text-gray-500 font-semibold">
                                        SteamId
                                    </td>
                                    <td className="px-2 py-2">
                                        {data?.steam_id !== null ? data?.steam_id : "Null"}
                                    </td>
                                </tr>
                                <tr>
                                    <td className="px-2 py-2 text-gray-500 font-semibold">
                                        PsnId
                                    </td>
                                    <td className="px-2 py-2">
                                        {data?.psn_auth_code !== null
                                            ? data?.psn_auth_code
                                            : "Null"}
                                    </td>
                                </tr>
                                <tr>
                                    <td className="px-2 py-2 text-gray-500 font-semibold">
                                        XboxId
                                    </td>
                                    <td className="px-2 py-2">testId2</td>
                                </tr>
                            </tbody>
                        </table>

                        <div className="text-center my-3">
                            <button onClick={logout}>Sign out</button>
                            <Link
                                to="/"
                                className="text-xs text-indigo-500 italic hover:underline hover:text-indigo-600 font-medium"
                            >
                                Home
                            </Link>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default UserProfile;
