import React from "react";
import Svg, {
  Path,
  Circle,
  Rect,
  Line,
  Polyline,
  Polygon,
  G,
  Defs,
  ClipPath,
  Mask,
  Use,
  Symbol,
  LinearGradient,
  RadialGradient,
  Stop,
  type SvgProps,
} from "react-native-svg";
import type { StyleProp, ViewStyle } from "react-native";

export interface ClockRotateRightProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function ClockRotateRight({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: ClockRotateRightProps) {
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
        d="M12 6L12 12L18 12"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M21.8883 10.5C21.1645 5.68874 17.013 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22C16.1006 22 19.6248 19.5318 21.1679 16"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M17 16H21.4C21.7314 16 22 16.2686 22 16.6V21"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
    </Svg>
  );
}

export default ClockRotateRight;
