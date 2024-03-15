import { useNavigate } from "react-router-dom";
import { useState, useRef } from "react";
import { useMutation } from "react-query";
import apiClient from "../services/api-common";
import { IconBrandGithub, IconBrandGoogle } from "@tabler/icons-react";
import { Label } from "../components/ui/label";
import { Input } from "../components/ui/input";
import { LabelInputContainer } from "../components/ui/labelInputContainer";
import { BottomGradient } from "../components/ui/bottomGradient";

function SignIn() {
  const usernameRef = useRef<HTMLInputElement>(null);
  const passwordRef = useRef<HTMLInputElement>(null);
  const [error, setError] = useState<CustomError | null>(null);
  const nav = useNavigate();

  const { mutate: postUser } = useMutation(
    async (userData: { username: string; password: string }) => {
      return await apiClient.post("auth/signin", {
        username: userData.username,
        password: userData.password,
      });
    },
    {
      onSuccess: () => {
        nav("/profile");
      },
      onError: (err) => {
        setError((err as any).response.data as CustomError);
      },
    },
  );

  const postData = (event: any) => {
    event.preventDefault();
    const usernameValue = usernameRef.current?.value || "";
    const passwordValue = passwordRef.current?.value || "";

    postUser({ username: usernameValue, password: passwordValue });
  };

  return (
    <div className="flex bg-black justify-center items-center h-screen">
      <div className="w-1/2 h-screen hidden lg:block">
        <img
          src="../../images/photo-1698207873249-640dab81d84e.jpg"
          alt="Placeholder Image"
          className="object-cover w-full h-full"
        ></img>
      </div>
      <div className="max-w-md w-full mx-auto rounded-none md:rounded-2xl p-4 md:p-8 shadow-input bg-white dark:bg-black">
        <h2 className="font-bold text-xl text-neutral-800 dark:text-neutral-200">
          Login into Webapp
        </h2>
        {error && (
          <p className="text-neutral-600 text-sm max-w-sm mt-2 dark:text-neutral-300">
            {error.error}
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
            <button
              className=" relative group/btn flex space-x-2 items-center justify-start px-4 w-full text-black rounded-md h-10 font-medium shadow-input bg-gray-50 dark:bg-zinc-900 dark:shadow-[0px_0px_1px_1px_var(--neutral-800)]"
              type="button"
              onClick={() => nav("/signup")}
            >
              <IconBrandGithub className="h-4 w-4 text-neutral-800 dark:text-neutral-300" />
              <span className="text-neutral-700 dark:text-neutral-300 text-sm">
                Signin
              </span>
              <BottomGradient />
            </button>
            <button
              className=" relative group/btn flex space-x-2 items-center justify-start px-4 w-full text-black rounded-md h-10 font-medium shadow-input bg-gray-50 dark:bg-zinc-900 dark:shadow-[0px_0px_1px_1px_var(--neutral-800)]"
              type="button"
              onClick={() => nav("/")}
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
    </div>
  );
}

export default SignIn;
