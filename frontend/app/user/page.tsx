"use client";

import { title } from "@/components/primitives";
import { LabelInputContainer } from "@/components/ui/label-input-container";
import apiClient from "@/services/api-common";
import { Button } from "@nextui-org/button";
import { Card, CardBody } from "@nextui-org/card";
import { Modal, ModalContent, ModalHeader, ModalBody, Input, ModalFooter, useDisclosure } from "@nextui-org/react";
import { Label } from "@radix-ui/react-label";
import { useRouter } from "next/navigation";
import { useEffect, useRef, useState } from "react";

export default function UserPage() {
	const { isOpen, onOpen, onOpenChange } = useDisclosure();
	const [axumUser, setAxumUser] = useState<AxumUser>();
	const steamIdRef = useRef<HTMLInputElement>(null);
	const [error, setError] = useState("");
	const router = useRouter();


  useEffect(() => {
    const fetchPlayerInfo = async () => {
      const token = localStorage.getItem("token");
      const userId = localStorage.getItem("user");
      try {
        const [userInfo] =
          await Promise.all([
            apiClient.get(`users`, {
              headers: {
                "axum-accountId": userId,
                Authorization: token,
              },
            }),
          ]);

          setAxumUser(userInfo.data)
        } catch (error: any) {
          if (error.message == "Request failed with status code 401") {
            localStorage.removeItem("user");
            localStorage.removeItem("token");
          }
          console.error("Error fetching player info:", error);
        }
      };
  
      fetchPlayerInfo();
    }, []);


	const updateInfo = async (event: any) => {
		const steamIdValue = steamIdRef.current?.value || "";
	
		if (steamIdValue.length < 1) {
		  setError("steam id cant be empty");
		  return;
		}
	
		try {
		  await apiClient.put(
			`users/steam_id`,
			{
			  steam_id: steamIdValue,
			},
			{
			  headers: {
				"axum-accountId": localStorage.getItem("user"),
				Authorization: localStorage.getItem("token"),
			  },
			},
		  );
		} catch (err) {
		  console.log(err);
		  setError("Unable to sign in");
		}
	  };

	const signout = () => {
		localStorage.removeItem("user");
		localStorage.removeItem("token");
	
		router.push("/");
	  };
	return (
		<div>
			<h1 className={title()}>User</h1>
			<Card className="mt-10 border-none bg-background/60 dark:bg-default-100/50">
          <CardBody>
            <div className="grid grid-cols-6 md:grid-cols-12 gap-6 md:gap-4 items-center justify-center">
              <div className="flex flex-col col-span-6 md:col-span-8">
                <div className="flex justify-between items-start">
                  <div className="flex flex-col gap-0">
                    <h1 className="text-large font-medium mt-4">
                      Username: {axumUser?.username}
                    </h1>
                    <p className="text-small text-foreground/80"></p>
                    <p className="text-small text-foreground/80">
                      Email: {axumUser?.email}
                    </p>
                    <p className="text-small text-foreground/80">
                      SteamId: {axumUser?.steam_id}
                    </p>
                  </div>
                  <div className="flex flex-row">
                    <Button className="mx-4" color="primary" variant="light" onPress={onOpen}>
                      Update SteamId
                    </Button>
                    <Button className="mx-4" color="danger" variant="light" onPress={signout}>
                      Signout
                    </Button>
                  </div>
                </div>
              </div>
            </div>
          </CardBody>
        </Card>

		<Modal size="lg" isOpen={isOpen} onOpenChange={onOpenChange}>
        {error && (
          <p className="text-neutral-600 text-sm max-w-sm mt-2 dark:text-neutral-300">
            {error}
          </p>
        )}
        <ModalContent>
          {(onClose) => (
            <>
              <ModalHeader className="flex flex-col gap-1">
                Update Steam Id
              </ModalHeader>
              <ModalBody>
                <form className="my-8" onSubmit={updateInfo}>
                  <LabelInputContainer className="mb-4">
                    <Label htmlFor="username">steam Id</Label>
                    <Input
                      id="steamId"
                      placeholder="steamId"
                      type="text"
                      ref={steamIdRef}
                    />
                  </LabelInputContainer>
                </form>
              </ModalBody>
              <ModalFooter>
                <Button color="danger" variant="light" onPress={onClose}>
                  Close
                </Button>
                <Button color="primary" onPress={updateInfo}>
                  Update
                </Button>
              </ModalFooter>
            </>
          )}
        </ModalContent>
      </Modal>
		</div>
	);
}
