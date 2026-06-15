import { cn } from "@orbis/ui/cn";
import { Zap } from "lucide-react";
import { Link } from "react-router-dom";
import { useState } from "react";
import { MainMenu } from "./main-menu";

export function Sidebar() {
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <aside
      className={cn(
        "h-screen flex-shrink-0 flex-col desktop:overflow-hidden desktop:rounded-tl-[10px] desktop:rounded-bl-[10px] justify-between fixed top-0 pb-4 items-center flex z-50 transition-all duration-200 ease-&lsqb;cubic-bezier(0.4,0,0.2,1)&rsqb;",
        "bg-background border-e border-border start-0",
        isExpanded ? "w-[240px]" : "w-[70px]",
      )}
      onMouseEnter={() => setIsExpanded(true)}
      onMouseLeave={() => setIsExpanded(false)}
    >
      <div
        className={cn(
          "absolute top-0 start-0 h-[70px] flex items-center justify-center bg-background border-b border-border transition-all duration-200 ease-&lsqb;cubic-bezier(0.4,0,0.2,1)&rsqb;",
          isExpanded ? "w-full" : "w-[69px]",
        )}
      >
        <Link to="/" className="absolute start-[22px] transition-none">
          <Zap />
        </Link>
      </div>

      <div className="flex flex-col w-full pt-[70px] flex-1 border-b border-border mb-3">
        <MainMenu isExpanded={isExpanded} />
      </div>

      {/* <TeamDropdown isExpanded={isExpanded} /> */}
    </aside>
  );
}
