function SteamPlayer({ player }: { player: PlayerInfo }) {
  return (
    <div className="flex rounded list-none  w-full ">
      <img
        className="w-30 h-20 rounded-full border-2"
        src={player.avatarfull}
        alt="Player Avatar"
      />

      <ul className="flex flex-row">
        <li className="mx-2">{player.personaname}</li>
        <li className="mx-2">{player.steamid}</li>
        <li className="mx-2">
          Account Created:{" "}
          {player.timecreated
            ? new Date(player.timecreated * 1000).toLocaleString()
            : "Unknown"}
        </li>
        <li className="mx-2">
          Last Online:{" "}
          {player.lastlogoff
            ? new Date(player.lastlogoff * 1000).toLocaleString()
            : "Unknown"}
        </li>
      </ul>
    </div>
  );
}

export default SteamPlayer;
