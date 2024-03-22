"use client";

import { useEffect, useState } from "react";
import { Card, CardBody } from "@nextui-org/card";
import { Image } from "@nextui-org/image";
import apiClient from "@/services/api-common";
import Laoding from "@/components/laoding";
import ErrorPage from "@/components/error";
import {
  Link,
  Table,
  TableBody,
  TableCell,
  TableColumn,
  TableHeader,
  TableRow,
} from "@nextui-org/react";
import { timeConversion } from "@/utils/time";

function Profile() {
  const [player, setPlayer] = useState<PlayerInfo | null>(null);
  const [recentlyPlayed, setRecentlyPlayed] = useState<GameInfo[]>([]);
  const [isError, setIsError] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [url, setUrl] = useState("");
  const IMAGE_URL_BASE =
    "http://media.steampowered.com/steamcommunity/public/images/apps";

  useEffect(() => {
    const fetchPlayerInfo = async () => {
      const token = localStorage.getItem("token");
      const userId = localStorage.getItem("user");
      try {
        const [profileResponse, recentlyPlayedResponse] = await Promise.all([
          apiClient.get(`steam/player-profile`, {
            headers: {
              "axum-accountId": userId,
              Authorization: token,
            },
          }),
          apiClient.get<RecentlyPlayedResponse>(`steam/recently-played`, {
            headers: {
              "axum-accountId": userId,
              Authorization: token,
            },
          }),
        ]);

        const playerData = profileResponse.data["response"][0];
        setRecentlyPlayed(recentlyPlayedResponse["data"]["response"]["games"]);
        setPlayer(playerData);
        console.log(recentlyPlayed);
        setUrl(`https://steamcommunity.com/profiles/${player?.steamid}/`);
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
                      <Link isExternal href={url} showAnchorIcon>
                        {player?.personaname}
                      </Link>
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
        <div className="mt-8">
          <Table aria-label="Example table with custom cells">
            <TableHeader>
              <TableColumn key="icon">Icon</TableColumn>
              <TableColumn key="name">Name</TableColumn>
              <TableColumn key="playtime_2weeks">Recently Played</TableColumn>
              <TableColumn key="playtime_forever">Total Time</TableColumn>
            </TableHeader>
            <TableBody items={recentlyPlayed}>
              {(game) => (
                <TableRow key={game.appid}>
                  <TableCell>
                    <img
                      src={`${IMAGE_URL_BASE}/${game.appid}/${game.img_icon_url}.jpg`}
                    />
                  </TableCell>
                  <TableCell>{game.name}</TableCell>
                  <TableCell>
                    {game.playtime_2weeks
                      ? `${timeConversion(game.playtime_2weeks)} hrs`
                      : "0 hrs"}
                  </TableCell>
                  <TableCell>
                    {game.playtime_forever
                      ? `${timeConversion(game.playtime_forever)} hrs`
                      : "0 hrs"}
                  </TableCell>
                </TableRow>
              )}
            </TableBody>
          </Table>
        </div>
      </div>
    </div>
  );
}

export default Profile;
