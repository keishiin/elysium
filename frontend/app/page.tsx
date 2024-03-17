"use client";

import Link from 'next/link'
import { useEffect, useState } from "react";
import { SparklesCore } from "@/components/ui/sparkles";
import { useRouter } from 'next/navigation';

export default function Home() {
	const [token] = useState(localStorage.getItem("token"));
	const [userId] = useState(localStorage.getItem("user"));
	const router = useRouter();
	
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
		<div className="h-full relative w-ful flex flex-col items-center justify-center overflow-hidden rounded-md">
      <div className="w-full h-full absolute inset-0">
        <SparklesCore
          id="tsparticlesfullpage"
          background="transparent"
          minSize={0.6}
          maxSize={1.4}
          particleDensity={100}
          className="absolute inset-0 z-0"
          particleColor="#FFFFFF"
        />
      </div>
      <h1 className="md:text-7xl text-3xl lg:text-6xl font-bold text-center text-blue-500 dark:text-blue-500 relative z-20">
        Elysium
      </h1>
      <div className="z-20 mt-8 flex flex-col md:flex-row space-y-5 md:space-y-0 space-x-0 md:space-x-4">
        {!userId && !token && (
          <>
            <button
				className="w-40 h-10 rounded-xl bg-black border dark:border-white border-transparent text-white text-sm"
				onClick={() => router.push("/login")}
				>
				Login
            </button>
				<button
				className="w-40 h-10 rounded-xl bg-white text-black border border-black text-sm"
				onClick={() => router.push("/register")}
				>
				Signup
				</button>
          </>
        )}
        {(userId || token) && (
			<button
				className="mt-8 w-40 h-10 rounded-xl bg-black border dark:border-white border-transparent text-white text-sm"
				onClick={() => router.push("/profile")}
			>
            Profile
          </button>
        )}
      </div>
    </div>
	);
}
