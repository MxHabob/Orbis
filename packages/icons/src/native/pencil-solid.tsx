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

export interface PencilSolidProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function PencilSolid({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: PencilSolidProps) {
  return (
    <Svg
      viewBox="0 0 24 24"
      width={size}
      height={size}
      fill={color}
      stroke={color}
      strokeWidth={strokeWidth}
      style={style}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden={true}
      {...props}
    >
      <Path d="m16.3 4.7 3-1.2 1.2 3-12.7 12.7-5.2 1.3 1.3-5.2L16.3 4.7Zm-1.5 1.5 3 3" />
    </Svg>
  );
}

export default PencilSolid;
