import { useEffect, useState } from "react";
import { SparklesCore } from "../components/ui/sparkles";
import { useNavigate } from "react-router-dom";

export function Home() {
  const [token] = useState(localStorage.getItem("token"));
  const [userId] = useState(localStorage.getItem("user"));
  const nav = useNavigate();

  // I have no idea why this is needed to get this to work properly and I hate it
  useEffect(() => {
    document.documentElement.style.height = "100%";
    document.body.style.height = "100%";

    return () => {
      document.documentElement.style.height = "";
      document.body.style.height = "";
    };
  }, []);

  return (
    <div className="h-screen relative w-full bg-black flex flex-col items-center justify-center overflow-hidden rounded-md">
      <div className="w-full absolute inset-0">
        <SparklesCore
          id="tsparticlesfullpage"
          background="transparent"
          minSize={0.6}
          maxSize={1.4}
          particleDensity={100}
          className="w-full h-full"
          particleColor="#FFFFFF"
        />
      </div>
      <h1 className="md:text-7xl text-3xl lg:text-6xl font-bold text-center text-white relative z-20">
        Ely<text className="text-blue-500 dark:text-blue-500">sium</text>
      </h1>
      <div className="z-20 mt-8 flex flex-col md:flex-row space-y-5 md:space-y-0 space-x-0 md:space-x-4">
        {!userId && !token && (
          <>
            <button
              className="w-40 h-10 rounded-xl bg-black border dark:border-white border-transparent text-white text-sm"
              onClick={() => nav("/login")}
            >
              Login
            </button>
            <button
              className=" mt-8 w-40 h-10 rounded-xl bg-white text-black border border-black text-sm"
              onClick={() => nav("/signup")}
            >
              Signup
            </button>
          </>
        )}
        {(userId || token) && (
          <button
            className="mt-8 w-40 h-10 rounded-xl bg-black border dark:border-white border-transparent text-white text-sm"
            onClick={() => nav("/profile")}
          >
            Profile
          </button>
        )}
      </div>
    </div>
  );
}
