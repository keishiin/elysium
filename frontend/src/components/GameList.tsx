// game_count the total number of games the user owns (including free games they've played, if include_played_free_games was passed)
// A games array, with the following contents (note that if "include_appinfo" was not passed in the request, only appid, playtime_2weeks,
// and playtime_forever will be returned):
// appid Unique identifier for the game
// name The name of the game
// playtime_2weeks The total number of minutes played in the last 2 weeks
// playtime_forever The total number of minutes played "on record", since Steam began tracking total playtime in early 2009.
// img_icon_url, img_logo_url - these are the filenames of various images for the game. To construct the URL to the image,
// use this format: http://media.steampowered.com/steamcommunity/public/images/apps/{appid}/{hash}.jpg. For example, the TF2 logo
// is returned as "07385eb55b5ba974aebbe74d3c99626bda7920b8", which maps to the URL: [1]
// has_community_visible_stats indicates there is a stats page with achievements or other game stats available for this game.
// The uniform URL for accessing this data is http://steamcommunity.com/profiles/{steamid}/stats/{appid}. For example, Robin's
// TF2 stats can be found at: http://steamcommunity.com/profiles/76561197960435530/stats/440. You may notice that clicking this
//  link will actually redirect to a vanity URL like /id/robinwalker/stats/TF2

import IMAGE_URL_BASE from "../constants";
import timeConversion from "../utils/time";

function GameList({ games }: { games: Array<Game> }) {
    return (
        <>
            <div className="bg-white p-8 rounded-md w-full">
                {/** ill use thos when i have a search query set up */}
                {/* <div className=" flex items-center justify-between pb-6">
                    <div>
                        <h1 className="text-gray-600 font-semibold">Steam Game List</h1>
                    </div>
                    <div className="flex items-center justify-between">
                        <div className="flex bg-gray-50 items-center p-2 rounded-md"></div>
                        <div className="flex bg-gray-50 items-center p-2 rounded-md">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                className="h-5 w-5 text-gray-400"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                            >
                                <path
                                    fillRule="evenodd"
                                    d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
                                    clipRule="evenodd"
                                />
                            </svg>
                            <input
                                className="bg-gray-50 outline-none ml-1 block "
                                type="text"
                                name=""
                                id=""
                                placeholder="search..."
                            ></input>
                        </div>
                    </div>
                </div> */}
                <div>
                    <div className="-mx-4 sm:-mx-8 px-4 sm:px-8 py-4 overflow-x-auto">
                        <div className="inline-block min-w-full shadow rounded-lg overflow-hidden">
                            <table className="min-w-full leading-normal">
                                <thead>
                                    <tr>
                                        <th className="px-5 py-3 border-b-2 border-blue-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                                            Name
                                        </th>
                                        <th className="px-5 py-3 border-b-2 border-blue-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                                            App Id
                                        </th>
                                        <th className="px-5 py-3 border-b-2 border-blue-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                                            Recently Played
                                        </th>
                                        <th className="px-5 py-3 border-b-2 border-blue-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                                            Total Playe Time
                                        </th>
                                        <th className="px-5 py-3 border-b-2 border-blue-200 bg-gray-100 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider"></th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {games.map((game) => (
                                        <tr
                                            className="hover:bg-blue-500 cursor-pointer"
                                            key={game.appid}
                                        >
                                            <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm">
                                                <div className="flex items-center">
                                                    <div className="flex-shrink-0 w-10 h-10">
                                                        <img
                                                            className="w-full h-full rounded-full"
                                                            src={`${IMAGE_URL_BASE}/${game.appid}/${game.img_icon_url}.jpg`}
                                                            alt=""
                                                        />
                                                    </div>
                                                    <div className="ml-3">
                                                        <p className="text-gray-900 whitespace-no-wrap">
                                                            {game.name}
                                                        </p>
                                                    </div>
                                                </div>
                                            </td>
                                            <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm tex.">
                                                <p className="text-gray-900 whitespace-no-wrap">
                                                    {game.appid}
                                                </p>
                                            </td>
                                            <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm">
                                                <p className="text-gray-900 whitespace-no-wrap">
                                                    {game.playtime_2weeks
                                                        ? `${timeConversion(game.playtime_2weeks)} hrs`
                                                        : "0 hrs"}
                                                </p>
                                            </td>
                                            <td className="px-5 py-5 border-b border-gray-200 bg-white text-sm">
                                                <p className="text-gray-900 whitespace-no-wrap">
                                                    {game.playtime_forever
                                                        ? `${timeConversion(game.playtime_forever)} hrs`
                                                        : "0 hrs"}
                                                </p>
                                            </td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}

export default GameList;
