import React from "react";
import Svg, { Path, type SvgProps } from "react-native-svg";
import type { StyleProp, ViewStyle } from "react-native";

export interface CalculatorProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Calculator({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: CalculatorProps) {
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
      <Path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M15.75 15.75h.008v.008h-.008v-.008Zm0 2.25h.008v.008h-.008v-.008Zm-3-2.25h.008v.008h-.008v-.008Zm0 2.25h.008v.008h-.008v-.008Zm-3-2.25h.008v.008h-.008v-.008Zm0 2.25h.008v.008h-.008v-.008Zm-1.5-6.75h9m-9-3h9M4.5 19.5V4.5A2.25 2.25 0 0 1 6.75 2.25h10.5A2.25 2.25 0 0 1 19.5 4.5v15A2.25 2.25 0 0 1 17.25 21.75H6.75A2.25 2.25 0 0 1 4.5 19.5Z"
      />
    </Svg>
  );
}

export default Calculator;
