import { Link } from "react-router-dom";

function UserProfile() {
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
                            Joh Doe
                        </h3>
                        <table className="text-xs my-3">
                            <tbody>
                                <tr>
                                    <td className="px-2 py-2 text-gray-500 font-semibold">
                                        Email
                                    </td>
                                    <td className="px-2 py-2">TestUser@exmaple.com</td>
                                </tr>
                                <tr>
                                    <td className="px-2 py-2 text-gray-500 font-semibold">
                                        SteamId
                                    </td>
                                    <td className="px-2 py-2">1233245325</td>
                                </tr>
                                <tr>
                                    <td className="px-2 py-2 text-gray-500 font-semibold">
                                        PsnId
                                    </td>
                                    <td className="px-2 py-2">testId</td>
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
