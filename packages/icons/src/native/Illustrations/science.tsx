import React from "react";
import { Circle, G, Line, Path } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function Science({ size = 112 }: Props) {
  return <Base size={size}>
    <Sparkles />
    <Path d="M38 22h18M43 22v26L28 77q-5 10 7 11h31q12-1 6-12L55 48V22" fill="#FFFDFB" {...outline}/>
    <Path d="M35 69q14 5 31-1l8 12q3 6-7 6H36q-9 0-5-7z" fill="#A8D9A0" stroke={C.ink} strokeWidth="2.3"/>
    <Path d="M37 72l7-7 7 5 7-8 7 5" stroke="#4E9C75" strokeWidth="2" strokeLinecap="round"/>
    <G stroke={C.ink} strokeWidth="2.3">
      <Line x1="78" y1="31" x2="91" y2="42"/><Line x1="91" y1="42" x2="82" y2="59"/><Line x1="91" y1="42" x2="104" y2="29"/>
      <Circle cx="78" cy="31" r="5" fill="#B5A7E0"/><Circle cx="91" cy="42" r="5" fill={C.yellow}/>
      <Circle cx="82" cy="59" r="5" fill={C.green}/><Circle cx="104" cy="29" r="5" fill={C.blue}/>
    </G>
  </Base>;
}
