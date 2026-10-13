import React from "react";
import Svg, { Circle, Ellipse, G, Line, Path, Polygon, Rect } from "react-native-svg";

export type Props = { size?: number; color?: string };

export const C = {
  ink: "#25234D",
  purple: "#7864A8",
  purpleLight: "#CFC4EA",
  lavender: "#EAE5F7",
  green: "#9ACB82",
  greenDark: "#4E8D62",
  yellow: "#F4C45F",
  orange: "#E9A64A",
  paper: "#FFFDFB",
  gray: "#D8D8E2",
  skin: "#F4BE9A",
  hair: "#49352F",
  hairLight: "#765047",
  blue: "#7FB7D8",
};

export const outline = {
  stroke: C.ink,
  strokeWidth: 2.6,
  strokeLinecap: "round" as const,
  strokeLinejoin: "round" as const,
};

export function Base({ size = 112, children }: React.PropsWithChildren<Props>) {
  return (
    <Svg width={size} height={size} viewBox="0 0 120 120" fill="none">
      {children}
    </Svg>
  );
}

export function Sparkles() {
  return (
    <G stroke={C.green} strokeWidth={2.4} strokeLinecap="round">
      <Path d="M16 38l-5-4M20 27l-1-6M28 32l5-5M95 37l5-4M98 49l7 1" />
    </G>
  );
}
