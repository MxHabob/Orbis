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

export interface ArrowPathProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function ArrowPath({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: ArrowPathProps) {
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
      <Path d="M20 11a8 8 0 0 0-13.66-5.66L4 7.68M4 4.5v3.18h3.18M4 13a8 8 0 0 0 13.66 5.66L20 16.32M20 19.5v-3.18h-3.18" />
    </Svg>
  );
}

export default ArrowPath;
