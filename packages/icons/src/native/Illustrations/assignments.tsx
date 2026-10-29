import React from "react";
import { G, Path, Rect } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function Assignments({ size = 112 }: Props) {
  return (
    <Base size={size}>
      <G transform="rotate(-5 54 59)">
        <Rect
          x="27"
          y="24"
          width="61"
          height="75"
          rx="4"
          fill="#FFFDFB"
          {...outline}
        />
        {[0, 1, 2, 3].map((i) => (
          <G key={i}>
            <Path
              d={`M37 ${39 + i * 13}h7`}
              stroke={C.purple}
              strokeWidth="2"
              strokeLinecap="round"
            />
            <Path
              d={`M49 ${39 + i * 13}h28`}
              stroke={C.ink}
              strokeWidth="1.8"
              strokeLinecap="round"
            />
          </G>
        ))}
        <Path d="M23 31v61M19 36v51" stroke={C.purple} strokeWidth="2" />
      </G>
      <G transform="rotate(24 87 70)">
        <Path d="M83 42h9v44h-9z" fill={C.yellow} {...outline} />
        <Path d="M83 42h9v9h-9z" fill={C.green} {...outline} />
        <Path d="M83 42l4-10 5 10z" fill="#F0C5A6" {...outline} />
        <Path d="M83 81h9" stroke={C.ink} strokeWidth="2" />
      </G>
    </Base>
  );
}
