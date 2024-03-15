import ErrorComponent from "../components/ErrorComponent";
import Loading from "../components/Loading";
import Navbar from "../components/Navbar";
import { get_onwed_games, get_player_info } from "../services/steam";
import GameList from "../components/GameList";

function Steam() {
    const ownedGames = get_onwed_games(104);

    if (ownedGames.isLoading) return <Loading />;

    if (ownedGames.isError) return <ErrorComponent />;

    return (
        <>
            <Navbar />
            <section>
                <GameList games={ownedGames.data["data"]} />
                <div className="px-5 py-5 bg-white border-t flex flex-col xs:flex-row items-center xs:justify-between          ">
                    <span className="text-xs xs:text-sm text-gray-900">
                        Showing 1 to 4 of 50 Entries
                    </span>
                    <div className="inline-flex mt-2 xs:mt-0">
                        <button className="text-sm text-indigo-50 transition duration-150 hover:bg-indigo-500 bg-indigo-600 font-semibold py-2 px-4 rounded-l">
                            Prev
                        </button>
                        &nbsp; &nbsp;
                        <button className="text-sm text-indigo-50 transition duration-150 hover:bg-indigo-500 bg-indigo-600 font-semibold py-2 px-4 rounded-r">
                            Next
                        </button>
                    </div>
                </div>
            </section>
        </>
    );
}

export default Steam;
