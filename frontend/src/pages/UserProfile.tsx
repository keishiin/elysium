import { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import apiClient from "../services/api-common";
import { useMutation, useQuery } from "react-query";

interface SignInResponse {
  id: string;
  username: string;
  email: string;
  steam_id?: string;
  psn_id?: string;
  xbox_id?: string;
}

function UserProfile() {
  const [data, setData] = useState<SignInResponse | undefined>(undefined);
  const [token] = useState(localStorage.getItem("token"));
  const [userId] = useState(localStorage.getItem("user"));
  const nav = useNavigate();

  const { refetch: getUser } = useQuery(
    "get-user",
    async () => {
      return await apiClient.get("users", {
        headers: {
          "axum-accountId": userId,
          Authorization: token,
        },
      });
    },
    {
      onSuccess: (res) => {
        setData(res?.data);
      },
      onError: () => {
        nav("/");
        console.log(`Error: not authenticated`);
      },
    },
  );

  const { mutate: removeUser } = useMutation(
    "remove-user",
    async () => {
      return await apiClient.post(
        "auth/signout",
        {},
        {
          headers: {
            "axum-accountId": userId,
            Authorization: token,
          },
        },
      );
    },
    {
      onSuccess: () => {
        setData(undefined);
        localStorage.removeItem("user");
        localStorage.removeItem("token");

        nav("/");
      },
      onError: (err) => {
        console.log(`Error: ${err}`);
      },
    },
  );

  useEffect(() => {
    try {
      getUser();
    } catch (err) {
      nav("/");
      console.log(`Error: not authenticated`);
    }
  }, []);

  const logout = () => {
    try {
      removeUser();
    } catch (err) {
      console.log(`Error: ${err}`);
    }
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
                    {data?.psn_id !== null ? data?.psn_id : "Null"}
                  </td>
                </tr>
                <tr>
                  <td className="px-2 py-2 text-gray-500 font-semibold">
                    XboxId
                  </td>
                  <td className="px-2 py-2">
                    {" "}
                    {data?.xbox_id !== null ? data?.xbox_id : "Null"}
                  </td>
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
