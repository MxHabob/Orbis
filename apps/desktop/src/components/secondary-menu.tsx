"use client";

import { cn } from "@orbis/ui/cn";
import { Link } from "react-router-dom";
import { useLocation } from "react-router-dom";

type Item = {
  path: string;
  label: string;
};

type Props = {
  items: Item[];
};

export function SecondaryMenu({ items }: Props) {
  const { pathname } = useLocation();

  return (
    <nav className="py-4">
      <ul className="flex gap-6 text-sm overflow-auto scrollbar-hide">
        {items.map((item) => (
          <Link
            prefetch
            key={item.path}
            to={item.path}
            className={cn(
              "text-[#606060]",
              pathname === item.path &&
                "text-primary font-medium underline underline-offset-8",
            )}
          >
            <span>{item.label}</span>
          </Link>
        ))}
      </ul>
    </nav>
  );
}
