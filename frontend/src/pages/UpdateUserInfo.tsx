import { useNavigate } from "react-router-dom";
import { useState } from "react";
import { useMutation } from "react-query";
import apiClient from "../services/api-common";
import Navbar from "../components/Navbar";

function UpdateUserInfo() {
  const [platform, setPlatform] = useState("");
  const [newId, setNewId] = useState("");
  const [error, setError] = useState<CustomError | null>(null);
  const nav = useNavigate();

  const { mutate: postUser } = useMutation(
    async () => {
      return await apiClient.put(
        `users/${platform}`,
        {
          xbox_id: newId,
          steam_id: newId,
          psn_id: newId,
        },
        {
          headers: {
            "axum-accountId": localStorage.getItem("user"),
            Authorization: localStorage.getItem("token"),
          },
        },
      );
    },
    {
      onSuccess: () => {
        nav("/userProfile");
      },
      onError: (err) => {
        setError((err as any).response.data as CustomError);
      },
    },
  );

  const postData = (event: any) => {
    event.preventDefault();

    try {
      postUser();
    } catch (err) {
      cleanUp(event);
      console.log("error");
    }
  };

  const cleanUp = (event: any) => {
    event.target.reset();
    setPlatform("");
    setNewId("");
  };

  return (
    <div>
      <Navbar />
      <div className="bg-gray-100 flex justify-center items-center h-screen">
        <div className="lg:p-36 md:p-52 sm:20 p-8 w-full lg:w-1/2">
          {error && (
            <div className="text-red-500 mt-2 text-center">{error.error}</div>
          )}
          <h1 className="text-2xl font-semibold mb-4">Update</h1>
          <form onSubmit={postData}>
            <div className="mb-4">
              <label className="block text-gray-600">New Id</label>
              <input
                type="input"
                id="newId"
                name="newId"
                className="w-full border border-gray-300 rounded-md py-2 px-3 focus:outline-none focus:border-blue-500"
                onChange={(event) => setNewId(event.target.value)}
              ></input>
            </div>
            <div className="main flex overflow-hidden m-4 select-none">
              <label className="flex radio p-2 cursor-pointer">
                <input
                  className="my-auto transform scale-125"
                  type="radio"
                  name="sfg"
                  value="steam_id"
                  onChange={(event) => setPlatform(event.target.value)}
                />
                <div className="title px-2">STEAM</div>
              </label>

              <label className="flex radio p-2 cursor-pointer">
                <input
                  className="my-auto transform scale-125"
                  type="radio"
                  name="sfg"
                  value="psn_id"
                  onChange={(event) => setPlatform(event.target.value)}
                />
                <div className="title px-2">PSN</div>
              </label>

              <label className="flex radio p-2 cursor-pointer">
                <input
                  className="my-auto transform scale-125"
                  type="radio"
                  name="sfg"
                  value="xbox_id"
                  onChange={(event) => setPlatform(event.target.value)}
                />
                <div className="title px-2">XBOX</div>
              </label>
            </div>
            <button
              type="submit"
              className="bg-blue-500 hover:bg-blue-600 text-white font-semibold rounded-md py-2 px-4 w-full"
            >
              Update
            </button>
          </form>
        </div>
      </div>
    </div>
  );
}

export default UpdateUserInfo;
