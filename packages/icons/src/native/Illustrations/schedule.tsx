import React from "react";
import { Circle, Path, Rect } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function Schedule({ size = 112 }: Props) {
  return (
    <Base size={size}>
      <Rect
        x="22"
        y="30"
        width="66"
        height="63"
        rx="6"
        fill="#F9F5FF"
        {...outline}
      />
      <Rect
        x="22"
        y="30"
        width="66"
        height="16"
        rx="5"
        fill="#B9A9E3"
        stroke={C.ink}
        strokeWidth="2.5"
      />
      {[32, 48, 64, 78].map((x) => (
        <Path
          key={x}
          d={`M${x} 24v13`}
          stroke={C.ink}
          strokeWidth="3"
          strokeLinecap="round"
        />
      ))}
      {[0, 1, 2].map((r) =>
        [0, 1, 2, 3].map((c) => (
          <Rect
            key={`${r}-${c}`}
            x={31 + c * 13}
            y={53 + r * 11}
            width="8"
            height="7"
            rx="1.5"
            fill="#FFFDFB"
            stroke="#9E91C7"
            strokeWidth="1.2"
          />
        )),
      )}
      <Circle cx="85" cy="80" r="19" fill={C.paper} {...outline} />
      <Path
        d="M85 68v13l8 5"
        stroke={C.purple}
        strokeWidth="3"
        strokeLinecap="round"
      />
      <Circle cx="85" cy="80" r="2" fill={C.purple} />
    </Base>
  );
}
