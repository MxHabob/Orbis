import { lazy, Suspense } from "react";

const GlobalSheets = lazy(() =>
  import("./global-sheets").then((mod) => ({ default: mod.GlobalSheets })),
);

export function GlobalSheetsProvider() {
  return (
    <Suspense fallback={null}>
      <GlobalSheets />
    </Suspense>
  );
}
