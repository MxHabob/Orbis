import React from "react";
import { Circle, Ellipse, Path } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function StudentPortrait({ size = 112, girl = false }: Props & { girl?: boolean }) {
  return <Base size={size}>
    <Sparkles/>
    {/* hair behind head */}
    {girl ? (
      <Path d="M28 51q-2-29 27-34 34-5 39 29l-2 27-13 8-40-2-13-14z" fill={C.hair} {...outline}/>
    ) : (
      <Path d="M29 48q-4-26 21-32 24-9 39 9l7 25-12 7-42-3z" fill={C.hair} {...outline}/>
    )}
    {/* neck and uniform */}
    <Path d="M49 77h22v12H49z" fill={C.skin} {...outline}/>
    <Path d="M26 113q2-27 24-31l11 9 12-9q23 6 25 31z" fill={girl ? "#9CC58A" : "#8B79BD"} {...outline}/>
    <Path d="M50 84l11 9 11-9" fill="#FFFDFB" stroke={C.ink} strokeWidth="2"/>
    {/* face */}
    <Ellipse cx="60" cy="53" rx="25" ry="29" fill={C.skin} {...outline}/>
    {/* ears */}
    <Ellipse cx="35" cy="55" rx="5" ry="8" fill={C.skin} stroke={C.ink} strokeWidth="2"/>
    <Ellipse cx="85" cy="55" rx="5" ry="8" fill={C.skin} stroke={C.ink} strokeWidth="2"/>
    {/* fringe */}
    {girl ? (
      <Path d="M35 48q0-27 25-27 19 0 25 19-12-8-21-16-10 17-29 24z" fill={C.hair} {...outline}/>
    ) : (
      <Path d="M35 46q-2-22 19-27 19-7 29 8l-8 1 6 7-11-3-8-7-5 10-8-3z" fill={C.hair} {...outline}/>
    )}
    {/* eyes, brows, smile */}
    <Path d="M43 48l9-2M67 46l9 2" stroke={C.hair} strokeWidth="2.3" strokeLinecap="round"/>
    <Ellipse cx="49" cy="54" rx="3" ry="4" fill={C.ink}/>
    <Ellipse cx="72" cy="54" rx="3" ry="4" fill={C.ink}/>
    <Circle cx="50" cy="53" r="1" fill="#FFFDFB"/>
    <Circle cx="73" cy="53" r="1" fill="#FFFDFB"/>
    <Path d="M55 64q6 5 12-1" stroke="#B65F58" strokeWidth="2" strokeLinecap="round"/>
    <Ellipse cx="42" cy="62" rx="5" ry="2.5" fill="#E99B8D" opacity=".7"/>
    <Ellipse cx="78" cy="62" rx="5" ry="2.5" fill="#E99B8D" opacity=".7"/>
    {girl && <Path d="M83 57q5 20-8 27" stroke={C.hairLight} strokeWidth="4" strokeLinecap="round"/>}
  </Base>;
}
