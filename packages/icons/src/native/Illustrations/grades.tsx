import React from "react";
import { Path, Rect } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function Grades({ size = 112 }: Props) {
  return (
    <Base size={size}>
      <Path d="M20 77l58-7 15 11-61 9z" fill="#A7D28D" {...outline} />
      <Path d="M20 67l58-7 15 11-61 9z" fill="#E8D7AC" {...outline} />
      <Path d="M20 57l58-7 15 11-61 9z" fill="#B8A8DF" {...outline} />
      <Path
        d="M30 53l-3-22q0-3 4-4l35-5q4 0 4 4l3 21z"
        fill="#FFFDFB"
        {...outline}
      />
      <Path
        d="M37 33l27-4M38 41l14-2"
        stroke={C.purple}
        strokeWidth="2"
        strokeLinecap="round"
      />
      <Path
        d="M38 49l8-1"
        stroke={C.purple}
        strokeWidth="2"
        strokeLinecap="round"
      />
      <Path d="M75 60l8-1v19l-8 1z" fill={C.green} {...outline} />
      <Path d="M86 53l8-1v25l-8 1z" fill={C.yellow} {...outline} />
      <Path d="M97 43l8-1v34l-8 1z" fill="#8CB9D7" {...outline} />
      <Rect
        x="69"
        y="78"
        width="37"
        height="22"
        rx="2"
        fill="#FFFDFB"
        {...outline}
      />
      <Path
        d="M76 91l5-7 5 4 8-8"
        stroke={C.greenDark}
        strokeWidth="2.5"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </Base>
  );
}
