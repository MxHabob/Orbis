import React from "react";
import { G, Path, Rect } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function Grade7Math({ size = 112 }: Props) {
  return <Base size={size}>
    <G transform="rotate(-5 59 58)">
      <Rect x="20" y="27" width="77" height="57" rx="5" fill="#E9B75D" {...outline}/>
      <Rect x="26" y="32" width="65" height="44" rx="2" fill="#4F9B82" stroke={C.ink} strokeWidth="2"/>
      <Path d="M37 43h25M37 51h12" stroke="#FFFDFB" strokeWidth="2" strokeLinecap="round"/>
      <Path d="M48 66l8-11M48 55l8 11M67 55h14M74 49v13" stroke="#FFFDFB" strokeWidth="2.6" strokeLinecap="round"/>
      <Path d="M32 85h52" stroke={C.ink} strokeWidth="3" strokeLinecap="round"/>
    </G>
    <Sparkles/>
  </Base>;
}
