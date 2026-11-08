import React from "react";
import { Circle, G, Path, Polygon, Rect } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function Grade10Math({ size = 112 }: Props) {
  return (
    <Base size={size}>
      <Polygon points="23,83 51,27 77,83" fill="#F4C45F" {...outline} />
      <Polygon
        points="34,73 51,39 67,73"
        fill="#FFF5D9"
        stroke={C.ink}
        strokeWidth="1.8"
      />
      <G transform="rotate(9 80 59)">
        <Rect
          x="67"
          y="33"
          width="31"
          height="53"
          rx="4"
          fill="#9B8BCB"
          {...outline}
        />
        <Rect
          x="72"
          y="39"
          width="21"
          height="9"
          rx="1.5"
          fill="#DDF0C7"
          stroke={C.ink}
          strokeWidth="1.5"
        />
        {[0, 1, 2, 3, 4, 5].map((i) => (
          <Circle
            key={i}
            cx={75 + (i % 3) * 7}
            cy={56 + Math.floor(i / 3) * 10}
            r="2.3"
            fill={i % 2 ? C.yellow : C.paper}
            stroke={C.ink}
            strokeWidth="1"
          />
        ))}
      </G>
      <Path
        d="M24 91h79"
        stroke={C.ink}
        strokeWidth="2.5"
        strokeLinecap="round"
      />
    </Base>
  );
}
