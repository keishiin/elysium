import ErrorComponent from "../components/ErrorComponent";
import Navbar from "../components/Navbar";
import get_player_info from "../services/steam";

function Steam() {
  const { data, isLoading, isError } = get_player_info();

  if (isLoading) return (
    <div className="flex justify-center items-center min-h-screen">
      <div
        className="h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-e-transparent align-[-0.125em] text-surface motion-reduce:animate-[spin_6s_linear_infinite] dark:text-white" role="status">
        <span className="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]">Loading...</span>
      </div>
    </div>
  );

  if (isError) return <ErrorComponent />;;

  return (
    <>
        <Navbar />
        <h1>Steam</h1>
        <div>
            {data["response"].map((player: PlayerInfo,  index: number) => (
                <div key={index}>
                   <img src={player.avatarfull}></img>
                   <li>{player.communityvisibilitystate}</li>
                   <li>{player.profilestate}</li>
                   <li>{player.personaname}</li>
                   <li>{player.steamid}</li>
                   <li>{player.timecreated}</li>
                   <li>{player.lastlogoff}</li>
                </div>
            ))}
        </div>
    </>
);
}


export default Steam;
