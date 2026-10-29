import React from "react";
import Svg, { Path, Circle, type SvgProps } from "react-native-svg";
import type { StyleProp, ViewStyle } from "react-native";

export interface LanguageProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Language({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: LanguageProps) {
  return (
    <Svg
      viewBox="0 0 24 24"
      width={size}
      height={size}
      fill="none"
      stroke={color}
      strokeWidth={strokeWidth}
      style={style}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden={true}
      {...props}
    >
      <Path d="m5 8 6 6m-4-6 2 6m-4-4h7M4 5h10M9 3v2m2 8 2.5-5 2.5 5m-4.25-1.5h3.5" />
      <Circle cx="12" cy="12" r="9.25" />
    </Svg>
  );
}

export default Language;
