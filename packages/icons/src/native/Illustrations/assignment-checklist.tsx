import React from "react";
import { Circle, G, Path, Rect } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function AssignmentChecklist({ size = 112 }: Props) {
  return (
    <Base size={size}>
      <G transform="rotate(-5 58 59)">
        <Rect
          x="28"
          y="24"
          width="62"
          height="75"
          rx="5"
          fill="#FFFDFB"
          {...outline}
        />
        <Path
          d="M44 20h29q4 0 4 4v7H40v-7q0-4 4-4z"
          fill="#B7A7DF"
          {...outline}
        />
        {[0, 1, 2].map((i) => (
          <G key={i}>
            <Rect
              x="38"
              y={42 + i * 16}
              width="9"
              height="9"
              rx="1.5"
              fill="#FFFDFB"
              stroke={C.ink}
              strokeWidth="1.5"
            />
            <Path
              d={`M53 ${46 + i * 16}h25`}
              stroke={C.purple}
              strokeWidth="2"
              strokeLinecap="round"
            />
          </G>
        ))}
      </G>
      <Circle
        cx="84"
        cy="76"
        r="16"
        fill="#A8D9A0"
        stroke={C.ink}
        strokeWidth="2.5"
      />
      <Path
        d="M76 76l6 6 11-13"
        stroke="#FFFDFB"
        strokeWidth="4"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <Sparkles />
    </Base>
  );
}
