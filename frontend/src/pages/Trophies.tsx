import Navbar from "../components/Navbar";

interface TrophiesProps {
  platform: string;
}

function Trophies({ platform }: TrophiesProps) {
  return (
    <div className="min-h-screen bg-black">
      <Navbar />
      <div>{platform}</div>
    </div>
  );
}

export default Trophies;
