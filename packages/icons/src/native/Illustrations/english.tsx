import React from "react";
import { G, Path } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function English({ size = 112 }: Props) {
  return (
    <Base size={size}>
      <Sparkles />
      <Path d="M17 34q20-9 41 4v48q-21-13-41-4z" fill="#FFFDFB" {...outline} />
      <Path d="M58 38q21-13 43-4v48q-22-5-43 4z" fill="#F9F5FF" {...outline} />
      <Path d="M58 39v47" stroke={C.ink} strokeWidth="2.2" />
      {[0, 1, 2, 3].map((i) => (
        <G key={i} stroke="#A99BD5" strokeWidth="1.8" strokeLinecap="round">
          <Path d={`M25 ${46 + i * 8}q12-5 24 0`} />
          <Path d={`M66 ${47 + i * 8}q12-5 25-1`} />
        </G>
      ))}
      <G transform="rotate(24 86 76)">
        <Path d="M82 53l8 3-14 36-8-3z" fill={C.green} {...outline} />
        <Path d="M82 53l8 3 4-8-8-3z" fill={C.yellow} {...outline} />
        <Path d="M86 45l4-7 5 2-1 8" fill="#E88C73" {...outline} />
      </G>
    </Base>
  );
}
