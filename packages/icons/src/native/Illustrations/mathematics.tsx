import React from "react";
import { G, Path, Rect } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function Mathematics({ size = 112 }: Props) {
  return (
    <Base size={size}>
      <Sparkles />
      <G transform="rotate(-8 43 58)">
        <Rect
          x="23"
          y="24"
          width="38"
          height="57"
          rx="5"
          fill="#8F80C5"
          {...outline}
        />
        <Rect
          x="29"
          y="30"
          width="26"
          height="10"
          rx="2"
          fill="#DDF0C7"
          {...outline}
        />
        {[
          [31, 47],
          [42, 47],
          [31, 58],
          [42, 58],
          [31, 69],
          [42, 69],
        ].map(([x, y], i) => (
          <Rect
            key={i}
            x={x}
            y={y}
            width="7"
            height="7"
            rx="1.5"
            fill={i % 2 ? C.yellow : "#FFFDFB"}
            stroke={C.ink}
            strokeWidth="1.4"
          />
        ))}
      </G>
      <G transform="rotate(9 77 54)">
        <Path d="M68 28h25l-3 43-25-3z" fill="#F8D47D" {...outline} />
        <Path
          d="M71 36l16 2M70 44l16 2M69 52l16 2"
          stroke={C.ink}
          strokeWidth="1.6"
          strokeLinecap="round"
        />
      </G>
      <Path d="M23 84l61 4-9 9-44-3z" fill={C.green} {...outline} />
      <Path d="M29 90l42 3" stroke={C.ink} strokeWidth="1.5" />
    </Base>
  );
}
