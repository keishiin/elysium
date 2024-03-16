import ErrorComponent from "../components/ErrorComponent";
import Loading from "../components/Loading";
import Navbar from "../components/Navbar";
import { get_onwed_games } from "../services/steam";
import { Avatar, Table, TableHeader, TableColumn, TableBody, TableRow, TableCell, Pagination, Spinner } from "@nextui-org/react";
import { useEffect, useState } from "react";
import timeConversion from "../utils/time";
import IMAGE_URL_BASE from "../constants";

function Steam() {
    const [page, setPage] = useState(104);
    const ownedGames = get_onwed_games(page);
    const [data, setData] = useState<Game[]>([]);
    const [isLoading, setIsLoading] = useState(true);
    const [isError, setIsError] = useState(false);

    useEffect(() => {
        if (ownedGames.isLoading) {
            setIsLoading(true);
            setIsError(false);
        } else if (ownedGames.isError) {
            setIsLoading(false);
            setIsError(true);
        } else {
            setIsLoading(false);
            setIsError(false);
            setData(ownedGames.data?.data || []);
        }
    }, [ownedGames]);

    const pages = 24;

    useEffect(() => {
        console.log(page)
    }, [page])

    return (
        <>
            <Navbar />
            <div className="flex justify-center items-center min-h-screen w-full bg-black text-white">
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
                            items={data}
                            loadingContent={<Spinner />}
                            loadingState={isLoading ? "loading" : "idle"}
                        >
                            {(game) => (
                                <TableRow key={game?.appid}>
                                    <TableCell><Avatar isBordered className="opacity-100 z-100" src={`${IMAGE_URL_BASE}/${game.appid}/${game.img_icon_url}.jpg`}></Avatar></TableCell>
                                    <TableCell>{game.name}</TableCell>
                                    <TableCell>
                                        {game.playtime_2weeks ? `${timeConversion(game.playtime_2weeks)} hrs` : "0 hrs"}
                                    </TableCell>
                                    <TableCell>
                                        {game.playtime_forever ? `${timeConversion(game.playtime_forever)} hrs` : "0 hrs"}
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
