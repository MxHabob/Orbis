
import { Link } from "react-router-dom";
import appIcon from "public/appicon.png";

export default function NotFound() {
  return (
    <div className="h-screen flex flex-col items-center justify-center text-center text-sm text-[#606060]">
      <img
        src={appIcon}
        width={80}
        height={80}
        alt="Orbis"
        quality={100}
        className="mb-10"
      />
      <h2 className="text-xl font-semibold mb-2">Not Found</h2>
      <p className="mb-4">Could not find requested resource</p>
      <Link to="/" className="underline">
        Return Home
      </Link>
    </div>
  );
}
