import React from "react";
import Svg, { Path, type SvgProps } from "react-native-svg";
import type { StyleProp, ViewStyle } from "react-native";

export interface StudentAffairsProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function StudentAffairs({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: StudentAffairsProps) {
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
      <Path d="M2.75 7.5 12 3l9.25 4.5L12 12 2.75 7.5Z" />
      <Path d="M6.5 9.5v3.75c3.3 2.2 7.7 2.2 11 0V9.5m3.75-2v5" />
      <Path d="M4.5 16c2.65-.75 5.15-.35 7.5 1.1 2.35-1.45 4.85-1.85 7.5-1.1v4.25c-2.65-.7-5.15-.35-7.5 1-2.35-1.35-4.85-1.7-7.5-1V16Zm7.5 1.1v4.15" />
    </Svg>
  );
}

export default StudentAffairs;
