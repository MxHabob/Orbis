export type PositionSide = "top" | "right" | "bottom" | "left" | "inline-start" | "inline-end";
export type PhysicalSide = "top" | "right" | "bottom" | "left";

function resolveSide(side: PositionSide, direction: "ltr" | "rtl"): PhysicalSide {
  if (side === "inline-start") return direction === "rtl" ? "right" : "left";
  if (side === "inline-end") return direction === "rtl" ? "left" : "right";
  return side;
}

export function useDirectionAwareSide(side: () => PositionSide): () => PhysicalSide {
  const getDirection = () =>
    typeof document !== "undefined" && document.documentElement.dir === "rtl" ? "rtl" : "ltr";

  let physicalSide = $state<PhysicalSide>(resolveSide(side(), getDirection()));

  $effect(() => {
    if (typeof document === "undefined") return;

    const updateSide = () => {
      physicalSide = resolveSide(side(), getDirection());
    };
    const observer = new MutationObserver(updateSide);

    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["dir"],
    });
    updateSide();

    return () => observer.disconnect();
  });

  return () => physicalSide;
}
