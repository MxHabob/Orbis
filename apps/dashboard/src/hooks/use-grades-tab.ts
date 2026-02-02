import { useQueryState } from "nuqs";
import { createLoader, parseAsStringLiteral } from "nuqs/server";

export const gradesTabSchema = {
  tab: parseAsStringLiteral(["monthly", "final"] as const).withDefault("monthly"),
};

export function useGradesTab() {
  const [tab, setTab] = useQueryState(
    "tab",
    parseAsStringLiteral(["monthly", "final"] as const).withDefault("monthly"),
  );

  return { tab, setTab };
}

export const loadGradesTab = createLoader(gradesTabSchema);
