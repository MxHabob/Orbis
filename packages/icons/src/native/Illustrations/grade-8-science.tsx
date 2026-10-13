import React from "react";
import { Circle, Ellipse, G, Path } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function Grade8Science({ size = 112 }: Props) {
  return <Base size={size}>
    <G stroke={C.ink} strokeWidth="2.3" strokeLinecap="round" strokeLinejoin="round">
      <Ellipse cx="50" cy="51" rx="29" ry="12" transform="rotate(55 50 51)" fill="none"/>
      <Ellipse cx="50" cy="51" rx="29" ry="12" transform="rotate(-55 50 51)" fill="none"/>
      <Ellipse cx="50" cy="51" rx="29" ry="12" fill="none"/>
      <Circle cx="50" cy="51" r="5" fill={C.yellow}/>
      <Circle cx="23" cy="39" r="4" fill={C.purpleLight}/>
      <Circle cx="75" cy="62" r="4" fill={C.green}/>
    </G>
    <Path d="M87 23h13M90 23v22L78 71q-4 8 6 9h22q9-1 4-9L98 45V23" fill="#FFFDFB" {...outline}/>
    <Path d="M82 66q11 4 22 0l6 9q2 3-5 3H84q-7 0-4-4z" fill="#A8D9A0" stroke={C.ink} strokeWidth="2"/>
    <Sparkles/>
  </Base>;
}
