"use client";

import { useEffect, useState } from "react";
import { Card, CardBody } from "@nextui-org/card";
import { Image } from "@nextui-org/image";
import apiClient from "@/services/api-common";
import Laoding from "@/components/laoding";
import ErrorPage from "@/components/error";

function Profile() {
  const [player, setPlayer] = useState<PlayerInfo | null>(null);
  const [isError, setIsError] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const fetchPlayerInfo = async () => {
      const token = localStorage.getItem("token");
      const userId = localStorage.getItem("user");
      try {
        const response = await apiClient.get(`steam/player-profile`, {
          headers: {
            "axum-accountId": userId,
            Authorization: token,
          },
        });
        const playerData = response.data["response"][0];
        setPlayer(playerData);
        setIsLoading(false);
        setIsError(false);
      } catch (error: any) {
        setIsError(true);
        setIsLoading(false);
        if (error.message == "Request failed with status code 401") {
          localStorage.removeItem("user");
          localStorage.removeItem("token");
        }
        console.error("Error fetching player info:", error);
      }
    };

    fetchPlayerInfo();
  }, [isLoading]);

  if (isLoading) {
    return <Laoding />;
  }

  if (isError) {
    return <ErrorPage />;
  }

  return (
    <div className="h-screen flex flex-col items-center">
      <div className="max-w-[610px] w-full">
        <Card className="border-none bg-background/60 dark:bg-default-100/50">
          <CardBody>
            <div className="grid grid-cols-6 md:grid-cols-12 gap-6 md:gap-4 items-center justify-center">
              <div className="relative col-span-6 md:col-span-4">
                <Image
                  alt="player avatar"
                  className="object-cover opacity-100"
                  height={200}
                  shadow="md"
                  src={player?.avatarfull}
                  width="100%"
                />
              </div>
              <div className="flex flex-col col-span-6 md:col-span-8">
                <div className="flex justify-between items-start">
                  <div className="flex flex-col gap-0">
                    <h1 className="text-large font-medium mt-4">
                      {player?.personaname}
                    </h1>
                    <p className="text-small text-foreground/80">
                      Account Created:{" "}
                      {player?.timecreated
                        ? new Date(player?.timecreated * 1000).toLocaleString()
                        : "Unknown"}
                    </p>
                    <p className="text-small text-foreground/80">
                      Last Online:{" "}
                      {player?.lastlogoff
                        ? new Date(player?.lastlogoff * 1000).toLocaleString()
                        : "Unknown"}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </CardBody>
        </Card>
      </div>
    </div>
  );
}

export default Profile;
