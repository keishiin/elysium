'use client';
import ErrorComponent from "../components/ErrorComponent";
import Loading from "../components/Loading";
import Navbar from "../components/Navbar";
import {
    Avatar,
    Table,
    TableHeader,
    TableColumn,
    TableBody,
    TableRow,
    TableCell,
    Pagination,
    Spinner,
} from "@nextui-org/react";
import { useEffect, useState } from "react";
import timeConversion from "../utils/time";
import IMAGE_URL_BASE from "../constants";
import apiClient from "../services/api-common";

function Steam() {
    const [page, setPage] = useState(0);
    const [data, setData] = useState<Steam>({
        cursor: 0,
        data: [],
        game_count: 0,
    });
    const [isLoading, setIsLoading] = useState(true);
    const [isError, setIsError] = useState(false);
    const [pages, setPages] = useState(1);

    const token = localStorage.getItem("token");
    const userId = localStorage.getItem("user");

    useEffect(() => {
        const fetchData = async () => {
            setIsLoading(true);
            setIsError(false);
            try {
                let url = `steam/games`;
                if (page > 0) {
                    url = `steam/games?cursor=${page}`;
                }
                const response = await apiClient.get(url, {
                    headers: {
                        "axum-accountId": userId,
                        Authorization: token,
                    },
                });
                setData(response.data);
                setPages(Math.ceil(response.data.game_count / 10))
            } catch (error) {
                setIsError(true);
            }
            setIsLoading(false);
        };

        fetchData();
    }, [page, token, userId]);

    return (
        <>
            <Navbar />
            <div className="flex justify-center items-center min-h-screen w-full bg-gray-700 text-white">
                {isLoading && <Loading />}
                {isError && <ErrorComponent />}
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
                        </TableHeader>
                        <TableBody
                            items={data.data}
                            loadingContent={<Spinner />}
                            loadingState={isLoading ? "loading" : "idle"}
                        >
                            {(game) => (
                                <TableRow key={game?.appid}>
                                    <TableCell>
                                        <Avatar
                                            isBordered
                                            className="opacity-100 z-100"
                                            src={`${IMAGE_URL_BASE}/${game.appid}/${game.img_icon_url}.jpg`}
                                        ></Avatar>
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
                )}
            </div>
        </>
    );
}

export default Steam;
