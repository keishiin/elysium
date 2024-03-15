import Navbar from "../components/Navbar";
import { Card, CardBody, Image } from "@nextui-org/react";
import { get_player_info } from "../services/steam";
import Loading from "../components/Loading";
import ErrorComponent from "../components/ErrorComponent";

function UserProfile() {
  const playerInfo = get_player_info();

  if (playerInfo.isLoading) return <Loading />;

  if (playerInfo.isError) return <ErrorComponent />;

  const player = playerInfo.data["response"][0];
  console.log(player);

  return (
    <>
      <Navbar />
      <div className="h-screen bg-black flex flex-col items-center">
        <div className="max-w-[610px] w-full">
          <Card className="text-white border-none bg-background/60 dark:bg-default-100/50">
            <CardBody>
              <div className="grid grid-cols-6 md:grid-cols-12 gap-6 md:gap-4 items-center justify-center">
                <div className="relative col-span-6 md:col-span-4">
                  <Image
                    alt="player avatar"
                    className="object-cover opacity-100"
                    height={200}
                    shadow="md"
                    src={player.avatarfull}
                    width="100%"
                  />
                </div>
                <div className="flex flex-col col-span-6 md:col-span-8">
                  <div className="flex justify-between items-start">
                    <div className="flex flex-col gap-0">
                      <h1 className="text-large font-medium mt-4">
                        {player.personaname}
                      </h1>
                      <p className="text-small text-foreground/80">
                        Account Created:{" "}
                        {player.timecreated
                          ? new Date(player.timecreated * 1000).toLocaleString()
                          : "Unknown"}
                      </p>
                      <p className="text-small text-foreground/80">
                        Last Online:{" "}
                        {player.lastlogoff
                          ? new Date(player.lastlogoff * 1000).toLocaleString()
                          : "Unknown"}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </CardBody>
          </Card>
          <Card className="text-white border-none bg-background/60 dark:bg-default-100/50">
            <CardBody>
              <div className="grid grid-cols-6 md:grid-cols-12 gap-6 md:gap-4 items-center justify-center">
                <div className="relative col-span-6 md:col-span-4">
                  <Image
                    alt="player avatar"
                    className="object-cover opacity-100"
                    height={200}
                    shadow="md"
                    src={player.avatarfull}
                    width="100%"
                  />
                </div>
                <div className="flex flex-col col-span-6 md:col-span-8">
                  <div className="flex justify-between items-start">
                    <div className="flex flex-col gap-0">
                      <h1 className="text-large font-medium mt-4">
                        {player.personaname}
                      </h1>
                      <p className="text-small text-foreground/80">
                        Account Created:{" "}
                        {player.timecreated
                          ? new Date(player.timecreated * 1000).toLocaleString()
                          : "Unknown"}
                      </p>
                      <p className="text-small text-foreground/80">
                        Last Online:{" "}
                        {player.lastlogoff
                          ? new Date(player.lastlogoff * 1000).toLocaleString()
                          : "Unknown"}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </CardBody>
          </Card>
          <Card className="text-white border-none bg-background/60 dark:bg-default-100/50">
            <CardBody>
              <div className="grid grid-cols-6 md:grid-cols-12 gap-6 md:gap-4 items-center justify-center">
                <div className="relative col-span-6 md:col-span-4">
                  <Image
                    alt="player avatar"
                    className="object-cover opacity-100"
                    height={200}
                    shadow="md"
                    src={player.avatarfull}
                    width="100%"
                  />
                </div>
                <div className="flex flex-col col-span-6 md:col-span-8">
                  <div className="flex justify-between items-start">
                    <div className="flex flex-col gap-0">
                      <h1 className="text-large font-medium mt-4">
                        {player.personaname}
                      </h1>
                      <p className="text-small text-foreground/80">
                        Account Created:{" "}
                        {player.timecreated
                          ? new Date(player.timecreated * 1000).toLocaleString()
                          : "Unknown"}
                      </p>
                      <p className="text-small text-foreground/80">
                        Last Online:{" "}
                        {player.lastlogoff
                          ? new Date(player.lastlogoff * 1000).toLocaleString()
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
    </>
  );
}

export default UserProfile;
