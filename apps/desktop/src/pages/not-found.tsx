import { Link } from "react-router-dom";

export default function NotFoundPage() {
  return (
    <div className="h-screen flex flex-col items-center justify-center text-center text-sm text-[#606060]">
      <img
        src="/appicon.png"
        width={80}
        height={80}
        alt="Orbis"
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
