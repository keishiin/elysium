"use client";

import Loading from "@/components/laoding";
import apiClient from "@/services/api-common";
import timeConversion from "@/utils/time";
import {
    Table,
    TableHeader,
    TableColumn,
    TableBody,
    TableRow,
    TableCell,
} from "@nextui-org/table";
import { useState, useEffect } from "react";
import { Pagination } from "@nextui-org/pagination";
import { Button } from "@nextui-org/button";
import {
    useDisclosure,
    Modal,
    ModalContent,
    ModalHeader,
    ModalBody,
    ModalFooter,
} from "@nextui-org/react";

export default function DocsPage() {
    const [page, setPage] = useState(0);
    const [isLoading, setIsLoading] = useState(true);
    const [isError, setIsError] = useState(false);
    const [modelError, setModelError] = useState(false);
    const [error, setError] = useState("");
    const [pages, setPages] = useState(1);
    const [gameSchema, setGameSchema] = useState<GameStatsResponse>();
    const [playerAchievments, setPlayerAchievements] = useState<GameAchievementsResponse>();
    const [achievements, setAchievements] = useState<Achievement[]>();
    const [data, setData] = useState<Steam>({
        cursor: 0,
        data: [],
        game_count: 0,
    });
    
    const { isOpen, onOpen, onOpenChange } = useDisclosure();

    const token = localStorage.getItem("token");
    const userId = localStorage.getItem("user");
    const IMAGE_URL_BASE =
        "http://media.steampowered.com/steamcommunity/public/images/apps";

    useEffect(() => {
        const fetchData = async () => {
            setIsLoading(true);
            setIsError(false);
            try {
                const offset = (page - 1) * 10;
                let url = `steam/games`;
                if (page > 0) {
                    url = `steam/games?cursor=${offset}`;
                }
                const response = await apiClient.get(url, {
                    headers: {
                        "axum-accountId": userId,
                        Authorization: token,
                    },
                });
                
                setData(response.data);
                setPages(Math.ceil(response.data.game_count / 10));
            } catch (error) {
                setIsError(true);
            }
            setIsLoading(false);
        };

        fetchData();
    }, [page, token, userId]);

    const handleMoreInfo = async (gameId: string) => {
        try {

            const [gameSchemaResponse, playerGameAchievmentsResponse] = await Promise.all([

              apiClient.get("steam/game-schema", {
                headers: {
                    "axum-accountId": userId,
                    "axum-appid": gameId,
                    Authorization: token,
                },
            }),
              apiClient.get(`steam/game-achievements`, {
                headers: {
                    "axum-accountId": userId,
                    "axum-appid": gameId,
                    Authorization: token,
                },
              }),
            ]);
            setGameSchema(gameSchemaResponse.data.game);
            setPlayerAchievements(playerGameAchievmentsResponse.data);
            setAchievements(gameSchemaResponse.data.game.availableGameStats.achievements);
            console.log(achievements)
            setModelError(false)
        } catch (error) {
            setError("a error occured");
            setModelError(true)
            console.log(error)
        }
      };

    return (
        <div>
            <div className="flex flex-col justify-center items-center min-h-screen w-full">
                {!isLoading && !isError && (
                    <Table
                        aria-label="Example table with client async pagination"
                        bottomContent={
                            pages > 0 ? (
                                <div className="flex w-full justify-center">
                                    <Pagination
                                        isCompact
                                        showControls
                                        showShadow
                                        size="lg"
                                        page={page}
                                        total={pages}
                                        onChange={(page) => setPage(page)}
                                    />
                                </div>
                            ) : null
                        }
                    >
                        <TableHeader>
                            <TableColumn key="icon">Icon</TableColumn>
                            <TableColumn key="name">Name</TableColumn>
                            <TableColumn key="playtime_2weeks">Recently Played</TableColumn>
                            <TableColumn key="playtime_forever">Total Time</TableColumn>
                            <TableColumn key="More Info">Total Time</TableColumn>
                        </TableHeader>
                        <TableBody
                            items={data.data}
                            loadingContent={<Loading />}
                            loadingState={isLoading ? "loading" : "idle"}
                        >
                            {(game) => (
                                <TableRow
                                    key={game?.appid}
                                >
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
                                    <TableCell>
                                        <Button onClick={() => handleMoreInfo(game.appid.toString())} onPress={onOpen}>More Info</Button>
                                    </TableCell>
                                </TableRow>
                            )}
                        </TableBody>
                    </Table>
                )}
            </div>
            <Modal size="full" isOpen={isOpen} onOpenChange={onOpenChange}>
                <ModalContent>
                    {(onClose) => (
                        <>
                            <ModalHeader className="flex flex-col gap-1">
                                {playerAchievments?.gameName ? playerAchievments?.gameName : gameSchema?.gameName}
                            </ModalHeader>
                            <ModalBody>
                                {error && (
                                    <h1 color="danger" className="text-neutral-600 text-sm max-w-sm mt-2 dark:text-neutral-300">
                                        {error}
                                    </h1>
                                )}
                                {gameSchema?.availableGameStats.achievements.map((achi) => {
                                   return <p>{achi.displayName}</p>
                                })}
                            </ModalBody> 
                            <ModalFooter>
                                <Button color="danger" variant="light" onPress={onClose}>
                                    Close
                                </Button>
                            </ModalFooter>
                        </>
                    )}
                </ModalContent>
            </Modal>
        </div>
    );
}
