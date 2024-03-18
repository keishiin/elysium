"use client";

import { BottomGradient } from "@/components/ui/button-gradient";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { LabelInputContainer } from "@/components/ui/label-input-container";
import apiClient from "@/services/api-common";
import { isValidPassword, isValidEmail } from "@/utils/regex_utils";
import { IconBrandGithub, IconBrandGoogle } from "@tabler/icons-react";
import { useRouter } from "next/navigation";
import { useRef, useState } from "react";

export default function RegisterPage() {
  const usernameRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);
  const passwordCheckRef = useRef<HTMLInputElement>(null);
  const emailRef = useRef<HTMLInputElement>(null);
  const [error, setError] = useState<CustomError | null>(null);
  const router = useRouter();

  const handleSubmit = async (event: any) => {
    event.preventDefault();
    const usernameValue = usernameRef.current?.value || "";
    const passwordValue = passwordRef.current?.value || "";
    const passwordCheckValue = passwordCheckRef.current?.value || "";
    const emailValue = emailRef.current?.value || "";

    if (passwordValue !== passwordCheckValue) {
      setError({ error: "Passwords to dot match" });
      return;
    }

    if (!isValidPassword(passwordValue)) {
      setError({
        error:
          "Invalid password. Password must contain at least one uppercase letter, one lowercase letter, one digit, one special character, and be at least 8 characters long.",
      });
      return;
    }

    if (!isValidEmail(emailValue)) {
      setError({ error: "Invalid email address. Please enter a valid email." });
      return;
    }

    try {
      const response = await apiClient.post("auth/signup", {
        username: usernameValue,
        password: passwordValue,
        email: emailValue,
      });
      const customHeader = response.headers["authorization"];
      const customHeader2 = response.headers["axum-accountid"];

      localStorage.setItem("token", customHeader);
      localStorage.setItem("user", customHeader2);

      router.push("/profile");
    } catch (err: any) {
      console.log(err);
      setError(err as CustomError);
    }
  };

  return (
    <div className="flex justify-center items-center h-screen">
      <div className="max-w-md w-full mx-auto rounded-none md:rounded-2xl p-4 md:p-8 shadow-input bg-white dark:bg-black">
        <h2 className="font-bold text-xl text-neutral-800 dark:text-neutral-200">
          Signup into Elysium
        </h2>
        {error && (
          <p className="text-neutral-600 text-sm max-w-sm mt-2 dark:text-neutral-300">
            {error.error}
          </p>
        )}
        <form className="my-8" onSubmit={handleSubmit}>
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
            <Label htmlFor="email">Email</Label>
            <Input id="email" placeholder="email" type="text" ref={emailRef} />
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
          <LabelInputContainer className="mb-4">
            <Label htmlFor="password2">Confirm Password</Label>
            <Input
              id="password2"
              placeholder="••••••••"
              type="password"
              ref={passwordCheckRef}
            />
          </LabelInputContainer>

          <button
            className="bg-gradient-to-br relative group/btn from-black dark:from-zinc-900 dark:to-zinc-900 to-neutral-600 block dark:bg-zinc-800 w-full text-white rounded-md h-10 font-medium shadow-[0px_1px_0px_0px_#ffffff40_inset,0px_-1px_0px_0px_#ffffff40_inset] dark:shadow-[0px_1px_0px_0px_var(--zinc-800)_inset,0px_-1px_0px_0px_var(--zinc-800)_inset]"
            type="submit"
          >
            Signup &rarr;
            <BottomGradient />
          </button>

          <div className="bg-gradient-to-r from-transparent via-neutral-300 dark:via-neutral-700 to-transparent my-8 h-[1px] w-full" />

          <div className="flex flex-col space-y-4">
            <button
              className=" relative group/btn flex space-x-2 items-center justify-start px-4 w-full text-black rounded-md h-10 font-medium shadow-input bg-gray-50 dark:bg-zinc-900 dark:shadow-[0px_0px_1px_1px_var(--neutral-800)]"
              type="button"
              onClick={() => router.push("/login")}
            >
              <IconBrandGithub className="h-4 w-4 text-neutral-800 dark:text-neutral-300" />
              <span className="text-neutral-700 dark:text-neutral-300 text-sm">
                Login
              </span>
              <BottomGradient />
            </button>
            <button
              className=" relative group/btn flex space-x-2 items-center justify-start px-4 w-full text-black rounded-md h-10 font-medium shadow-input bg-gray-50 dark:bg-zinc-900 dark:shadow-[0px_0px_1px_1px_var(--neutral-800)]"
              type="button"
              onClick={() => router.push("/")}
            >
              <IconBrandGoogle className="h-4 w-4 text-neutral-800 dark:text-neutral-300" />
              <span className="text-neutral-700 dark:text-neutral-300 text-sm">
                Home
              </span>
              <BottomGradient />
            </button>
          </div>
        </form>
      </div>
      <div className="w-1/2 h-screen hidden lg:block">
        <img
          src="../../static/form.jpg"
          alt="Placeholder Image"
          className="object-cover w-full h-full"
        ></img>
      </div>
    </div>
  );
}
