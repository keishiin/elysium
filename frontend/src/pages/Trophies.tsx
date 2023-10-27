import Navbar from "../components/Navbar";

interface TrophiesProps {
  platform: string;
}

function Trophies({ platform }: TrophiesProps) {
  return (
    <div>
      <Navbar />
      <div>{platform}</div>
    </div>
  );
}

export default Trophies;
