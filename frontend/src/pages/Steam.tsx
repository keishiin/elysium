import { useState } from "react";
import ErrorComponent from "../components/ErrorComponent";
import Loading from "../components/Loading";
import Navbar from "../components/Navbar";
import { get_onwed_games, get_player_info } from "../services/steam";
import GameList from "../components/GameList";

function Steam() {
  const [cursor, setCursor] = useState(0);
  const playerInfo = get_player_info();
  const ownedGames = get_onwed_games(cursor);
  
  if (playerInfo.isLoading || ownedGames.isLoading) return <Loading />;
  
  if (playerInfo.isError || ownedGames.isError) return <ErrorComponent />;

  
  return (
    <>
        <Navbar />
        <h1>Steam</h1>
        <section >
          <div>
              {playerInfo.data["response"].map((player: PlayerInfo) => (
                  <div key={player.steamid}>
                    <img src={player.avatarfull}></img>
                    <li>{player.personaname}</li>
                    <li>{player.steamid}</li>
                    <li>Account Created: {player.timecreated ? new Date(player.timecreated * 1000).toLocaleString() : "Unknown"}</li>
                    <li>Last Online: {player.lastlogoff ? new Date(player.lastlogoff * 1000).toLocaleString() : "Unknown"}</li>
                  </div>
              ))}
          </div>
        </section>
        <input type="number" onChange={(event) => setCursor(parseInt(event.target.value))} />

        <section>
            <GameList games={ownedGames.data["data"]}/>
        </section>


    </>
);
}


export default Steam;
