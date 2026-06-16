import { cn } from "@orbis/ui/cn";
import { Zap } from "lucide-react";
import { useState } from "react";
import { Link } from "react-router-dom";
import { MainMenu } from "./main-menu";

export function Sidebar() {
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <aside
      className={cn(
        "h-screen shrink-0 flex-col desktop:overflow-hidden desktop:rounded-tl-[10px] desktop:rounded-bl-[10px] justify-between fixed top-0 pb-4 items-center flex z-50 transition-all duration-200 ease-&lsqb;cubic-bezier(0.4,0,0.2,1)&rsqb;",
        "bg-background border-e border-border inset-s-0",
        isExpanded ? "w-60" : "w-17.5",
      )}
      onMouseEnter={() => setIsExpanded(true)}
      onMouseLeave={() => setIsExpanded(false)}
    >
      <div
        className={cn(
          "absolute top-0 inset-s-0 h-17.5 flex items-center justify-center bg-background border-b border-border transition-all duration-200 ease-&lsqb;cubic-bezier(0.4,0,0.2,1)&rsqb;",
          isExpanded ? "w-full" : "w-17.25",
        )}
      >
        <Link to="/" className="absolute inset-s-5.5 transition-none">
          <Zap />
        </Link>
      </div>

      <div className="flex flex-col w-full pt-17.5 flex-1 border-b border-border mb-3">
        <MainMenu isExpanded={isExpanded} />
      </div>

      {/* <TeamDropdown isExpanded={isExpanded} /> */}
    </aside>
  );
}
