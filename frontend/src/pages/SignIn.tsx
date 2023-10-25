import { useState } from "react";
import { Link } from "react-router-dom";

function SignIn() {
  const [username, setUserName] = useState("");
  const [password, setPassword] = useState("");

  const handleSubmit = (event: any) => {
    console.log(username);
    console.log(password);

    event.target.reset();

    setUserName("");
    setPassword("");
  };

  return (
    <div className="bg-gray-100 flex justify-center items-center h-screen">
      {/* <!-- Left: Image --> */}
      <div className="w-1/2 h-screen hidden lg:block">
        <img
          src="../../images/photo-1698207873249-640dab81d84e.jpg"
          alt="Placeholder Image"
          className="object-cover w-full h-full"
        ></img>
      </div>
      {/* Right: Login Form  */}
      <div className="lg:p-36 md:p-52 sm:20 p-8 w-full lg:w-1/2">
        <h1 className="text-2xl font-semibold mb-4">Login</h1>
        <form onSubmit={handleSubmit}>
          {/* <!-- Username Input --> */}
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
          {/* <!-- Password Input --> */}
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
          {/* <!-- Remember Me Checkbox --> */}
          <div className="mb-4 flex items-center">
            <input
              type="checkbox"
              id="remember"
              name="remember"
              className="text-blue-500"
            ></input>
            <label className="text-gray-600 ml-2">Remember Me</label>
          </div>
          {/* <!-- Forgot Password Link --> */}
          <div className="mb-6 text-blue-500">
            <Link to="/" className="hover:underline">
              Forgot Password?
            </Link>
          </div>
          {/* <!-- Login Button --> */}
          <button
            type="submit"
            className="bg-blue-500 hover:bg-blue-600 text-white font-semibold rounded-md py-2 px-4 w-full"
          >
            Login
          </button>
        </form>
        {/* <!-- Sign up  Link --> */}
        <div className="mt-6 text-blue-500 text-center">
          <Link to="/signup" className="hover:underline">
            Sign up Here
          </Link>
        </div>
      </div>
    </div>
  );
}

export default SignIn;
