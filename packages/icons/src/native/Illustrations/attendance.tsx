import React from "react";
import { G, Path, Rect } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function Attendance({ size = 112 }: Props) {
  return (
    <Base size={size}>
      <G transform="rotate(-6 54 60)">
        <Rect
          x="25"
          y="29"
          width="66"
          height="63"
          rx="5"
          fill="#FFFDFB"
          {...outline}
        />
        <Rect
          x="25"
          y="29"
          width="66"
          height="16"
          rx="4"
          fill="#B6A6DD"
          stroke={C.ink}
          strokeWidth="2.4"
        />
        {[35, 49, 63, 77].map((x) => (
          <Path
            key={x}
            d={`M${x} 22v14`}
            stroke={C.ink}
            strokeWidth="3"
            strokeLinecap="round"
          />
        ))}
        {[0, 1, 2].map((r) =>
          [0, 1, 2, 3].map((c) => (
            <Rect
              key={`${r}-${c}`}
              x={32 + c * 13}
              y={52 + r * 10}
              width="8"
              height="7"
              rx="1"
              fill="#FFFDFB"
              stroke="#A89BD0"
              strokeWidth="1.2"
            />
          )),
        )}
      </G>
      <Path
        d="M56 69l8 8 18-21"
        stroke={C.greenDark}
        strokeWidth="5"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <Sparkles />
    </Base>
  );
}
