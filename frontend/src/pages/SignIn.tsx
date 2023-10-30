import { Link, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
import { useMutation } from "react-query";
import apiClient from "../services/api-common";

function SignIn() {
  const [username, setUserName] = useState("");
  const [password, setPassword] = useState("");
  const nav = useNavigate();

  const { isLoading: isPostingUser, mutate: postUser } = useMutation(
    async () => {
      return await apiClient.post("auth/signin", {
        username: username,
        password: password,
      });
    },
    {
      onSuccess: (res) => {
        if (localStorage.getItem("user") != null) {
          localStorage.removeItem("user");
        }
        localStorage.setItem("user", JSON.stringify(res.data));
        console.log("headers => ", res.headers["set-cookie"]);
      },
      onError: (err) => {
        console.log(err);
      },
    },
  );

  useEffect(() => {
    if (isPostingUser) {
      console.log(isPostingUser);
    }
  }, [isPostingUser]);

  const postData = (event: any) => {
    event.preventDefault();

    try {
      postUser();
      nav("/userProfile");
    } catch (err) {
      cleanUp(event);
      console.log("error");
    }
  };

  const cleanUp = (event: any) => {
    event.target.reset();
    setUserName("");
    setPassword("");
  };

  return (
    <div className="bg-gray-100 flex justify-center items-center h-screen">
      <div className="w-1/2 h-screen hidden lg:block">
        <img
          src="../../images/photo-1698207873249-640dab81d84e.jpg"
          alt="Placeholder Image"
          className="object-cover w-full h-full"
        ></img>
      </div>
      <div className="lg:p-36 md:p-52 sm:20 p-8 w-full lg:w-1/2">
        <h1 className="text-2xl font-semibold mb-4">Login</h1>
        <form onSubmit={postData}>
          <div className="mb-4">
            <label className="block text-gray-600">Username</label>
            <input
              type="text"
              id="username"
              name="username"
              className="w-full border border-gray-300 rounded-md py-2 px-3 focus:outline-none focus:border-blue-500"
              onChange={(event) => setUserName(event.target.value)}
            ></input>
          </div>
          <div className="mb-4">
            <label className="block text-gray-600">Password</label>
            <input
              type="password"
              id="password"
              name="password"
              className="w-full border border-gray-300 rounded-md py-2 px-3 focus:outline-none focus:border-blue-500"
              onChange={(event) => setPassword(event.target.value)}
            ></input>
          </div>
          <div className="mb-4 flex items-center">
            <input
              type="checkbox"
              id="remember"
              name="remember"
              className="text-blue-500"
            ></input>
            <label className="text-gray-600 ml-2">Remember Me</label>
          </div>
          <div className="mb-6 text-blue-500">
            <Link to="/" className="hover:underline">
              Forgot Password?
            </Link>
          </div>
          <button
            type="submit"
            className="bg-blue-500 hover:bg-blue-600 text-white font-semibold rounded-md py-2 px-4 w-full"
          >
            Login
          </button>
        </form>
        <div className="mt-6 text-blue-500 text-center">
          <Link to="/signup" className="hover:underline">
            Sign up
          </Link>
        </div>
        <div className="mt-6 text-blue-500 text-center">
          <Link to="/" className="hover:underline">
            Home
          </Link>
        </div>
      </div>
    </div>
  );
}

export default SignIn;
