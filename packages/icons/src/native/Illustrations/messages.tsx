import React from "react";
import { Circle, Path } from "react-native-svg";
import { Base, C, Props, Sparkles, outline } from "./shared";

export function Messages({ size = 112 }: Props) {
  return (
    <Base size={size}>
      <Path
        d="M16 30q0-9 10-9h42q10 0 10 10v23q0 10-10 10H42L29 75V64q-13-1-13-12z"
        fill="#B5A4E1"
        {...outline}
      />
      {[0, 1, 2].map((i) => (
        <Circle key={i} cx={32 + i * 14} cy="42" r="4" fill="#FFFDFB" />
      ))}
      <Path
        d="M55 62h35q12 0 12 11v13q0 10-12 10H79l-12 8V96q-12-1-12-10z"
        fill="#A8D9A0"
        {...outline}
      />
      {[0, 1, 2].map((i) => (
        <Circle key={i} cx={70 + i * 11} cy="77" r="3" fill="#FFFDFB" />
      ))}
      <Sparkles />
    </Base>
  );
}
