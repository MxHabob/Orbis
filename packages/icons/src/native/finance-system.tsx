import React from "react";
import Svg, { Path, Circle, type SvgProps } from "react-native-svg";
import type { StyleProp, ViewStyle } from "react-native";

export interface FinanceSystemProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function FinanceSystem({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: FinanceSystemProps) {
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
      <Path d="M4.25 4.25h10.5v15.5H4.25A1.25 1.25 0 0 1 3 18.5V5.5a1.25 1.25 0 0 1 1.25-1.25Z" />
      <Path d="M6.5 8h5.75m-5.75 3.25h4.5m-4.5 3.25h3" />
      <Circle cx="17.25" cy="16.75" r="4.25" />
      <Path d="M16 15.75h2.5m-2.5 2h2.5" />
    </Svg>
  );
}

export default FinanceSystem;
