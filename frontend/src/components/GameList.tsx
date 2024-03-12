function GameList({ games }: { games: Array<Game> }) {

    return (
        <>
            {games.map((game) => {
                <ul role="list" className="divide-y divide-gray-100" key={game.app_id}>
                <li className="flex justify-between gap-x-6 py-5">
                    <div className="flex min-w-0 gap-x-4">
                    <img className="h-12 w-12 flex-none rounded-full bg-gray-50" src={game.img_icon_url} alt=""></img>
                        <div className="min-w-0 flex-auto">
                            <p className="text-sm font-semibold leading-6 text-gray-900">{game.name}</p>
                        </div>
                    </div>
                    <div className="hidden shrink-0 sm:flex sm:flex-col sm:items-end">
                        <p className="text-sm leading-6 text-gray-900">{game.total_play_time}</p>
                        <p className="mt-1 text-xs leading-5 text-gray-500">Recent Time<time dateTime="2023-01-23T13:23Z">{game.recent_play_time}</time></p>
                    </div>
                </li>
            </ul> 
            })}
        </>

    )
}

export default GameList;