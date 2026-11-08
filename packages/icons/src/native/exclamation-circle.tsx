import React from "react";
import Svg, { Path, Circle, type SvgProps } from "react-native-svg";
import type { StyleProp, ViewStyle } from "react-native";

export interface ExclamationCircleProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function ExclamationCircle({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: ExclamationCircleProps) {
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
      <Circle cx="12" cy="12" r="9.25" />
      <Path d="M12 7.75v5.5m0 3.5h.01" />
    </Svg>
  );
}

export default ExclamationCircle;
