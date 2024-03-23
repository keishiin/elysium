"use client";

import { useState, useRef } from "react";
import { IconBrandGithub, IconBrandGoogle } from "@tabler/icons-react";
import { LabelInputContainer } from "@/components/ui/label-input-container";
import { BottomGradient } from "@/components/ui/button-gradient";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import apiClient from "@/services/api-common";
import Link from "next/link";
import { useRouter } from "next/navigation";

export default function LoginPage() {
  const usernameRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);
  const [error, setError] = useState("");
  const router = useRouter();

  const postData = async (event: any) => {
    event.preventDefault();
    const usernameValue = usernameRef.current?.value || "";
    const passwordValue = passwordRef.current?.value || "";

    try {
      await apiClient.post("auth/signin", {
        username: usernameValue,
        password: passwordValue,
      });

      router.push("/user");
    } catch (err) {
      console.log(err);
      setError("Unable to sign in");
    }
  };
  return (
    <div className="flex justify-center items-center min-h-screen">
      <div className="w-full lg:w-1/2 lg:block hidden">
        <img
          src="../../static/form.jpg"
          alt="Placeholder Image"
          className="object-cover w-full h-full"
        />
      </div>
      <div className="max-w-md w-full mx-auto rounded-none md:rounded-2xl p-4 md:p-8 shadow-input bg-white dark:bg-black">
        <h2 className="font-bold text-xl text-neutral-800 dark:text-neutral-200">
          Login into Elysium
        </h2>
        {error && (
          <p className="text-neutral-600 text-sm max-w-sm mt-2 dark:text-neutral-300">
            {error}
          </p>
        )}
        <form className="my-8" onSubmit={postData}>
          <LabelInputContainer className="mb-4">
            <Label htmlFor="username">Username</Label>
            <Input
              id="username"
              placeholder="username"
              type="text"
              ref={usernameRef}
            />
          </LabelInputContainer>
          <LabelInputContainer className="mb-4">
            <Label htmlFor="password">Password</Label>
            <Input
              id="password"
              placeholder="••••••••"
              type="password"
              ref={passwordRef}
            />
          </LabelInputContainer>

          <button
            className="bg-gradient-to-br relative group/btn from-black dark:from-zinc-900 dark:to-zinc-900 to-neutral-600 block dark:bg-zinc-800 w-full text-white rounded-md h-10 font-medium shadow-[0px_1px_0px_0px_#ffffff40_inset,0px_-1px_0px_0px_#ffffff40_inset] dark:shadow-[0px_1px_0px_0px_var(--zinc-800)_inset,0px_-1px_0px_0px_var(--zinc-800)_inset]"
            type="submit"
          >
            Login &rarr;
            <BottomGradient />
          </button>

          <div className="bg-gradient-to-r from-transparent via-neutral-300 dark:via-neutral-700 to-transparent my-8 h-[1px] w-full" />

          <div className="flex flex-col space-y-4">
            <Link href="/register">
              <button
                className=" relative group/btn flex space-x-2 items-center justify-start px-4 w-full text-black rounded-md h-10 font-medium shadow-input bg-gray-50 dark:bg-zinc-900 dark:shadow-[0px_0px_1px_1px_var(--neutral-800)]"
                type="button"
              >
                <IconBrandGithub className="h-4 w-4 text-neutral-800 dark:text-neutral-300" />
                <span className="text-neutral-700 dark:text-neutral-300 text-sm">
                  Signin
                </span>
                <BottomGradient />
              </button>
            </Link>
            <Link href="/">
              <button
                className=" relative group/btn flex space-x-2 items-center justify-start px-4 w-full text-black rounded-md h-10 font-medium shadow-input bg-gray-50 dark:bg-zinc-900 dark:shadow-[0px_0px_1px_1px_var(--neutral-800)]"
                type="button"
              >
                <IconBrandGoogle className="h-4 w-4 text-neutral-800 dark:text-neutral-300" />
                <span className="text-neutral-700 dark:text-neutral-300 text-sm">
                  Home
                </span>
                <BottomGradient />
              </button>
            </Link>
          </div>
        </form>
      </div>
    </div>
  );
}
