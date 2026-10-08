export type PositionSide = "top" | "right" | "bottom" | "left" | "inline-start" | "inline-end";
export type PhysicalSide = "top" | "right" | "bottom" | "left";
export type TextDirection = "ltr" | "rtl";

function getDocumentDirection(): TextDirection {
  return typeof document !== "undefined" && document.documentElement.dir === "rtl"
    ? "rtl"
    : "ltr";
}

export function useDocumentDirection(): () => TextDirection {
  let direction = $state<TextDirection>(getDocumentDirection());

  $effect(() => {
    if (typeof document === "undefined") return;

    const updateDirection = () => {
      direction = getDocumentDirection();
    };
    const observer = new MutationObserver(updateDirection);

    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["dir"],
    });
    updateDirection();

    return () => observer.disconnect();
  });

  return () => direction;
}

function resolveSide(side: PositionSide, direction: TextDirection): PhysicalSide {
  if (side === "inline-start") return direction === "rtl" ? "right" : "left";
  if (side === "inline-end") return direction === "rtl" ? "left" : "right";
  return side;
}

export function useDirectionAwareSide(side: () => PositionSide): () => PhysicalSide {
  const getDirection = useDocumentDirection();
  let physicalSide = $state<PhysicalSide>(resolveSide(side(), getDirection()));

  $effect(() => {
    physicalSide = resolveSide(side(), getDirection());
  });

  return () => physicalSide;
}
