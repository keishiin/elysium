
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


function GameList({ games }: { games: Array<Game> }) {
    return (
        <>
            {games.map((game) => (
                <ul role="list" className="divide-y divide-gray-100" key={game.appid}>
                    <li>
                        <div className="p-4">
                            <div className="flex items-center justify-between">
                                <div className="flex items-center gap-x-4">
                                    <img className="h-12 w-12 flex-none rounded-full bg-gray-50" src={`${IMAGE_URL_BASE}/${game.appid}/${game.img_icon_url}.jpg`} alt=""></img>
                                    <p className="text-sm font-semibold leading-6 text-gray-900">{game.name}</p>
                                </div>
                                <div className="flex flex-col items-end">
                                    <p className="text-sm leading-6 text-gray-900">{game.total_play_time?.toString()}</p>
                                    <p className="mt-1 text-xs leading-5 text-gray-500">{game.recent_play_time !== null ? game.recent_play_time?.toString() : "0"}</p>
                                </div>
                            </div>
                        </div>
                    </li>
                </ul>
            ))}
        </>
    );
}

export default GameList;
