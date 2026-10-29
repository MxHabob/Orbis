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

export interface UsersProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Users({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: UsersProps) {
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
      <Circle cx="9" cy="7.5" r="3.25" />
      <Circle cx="17" cy="7.75" r="2.25" />
      <Path d="M3.5 20a5.5 5.5 0 0 1 11 0m0-4.75a5.5 5.5 0 0 1 6 4.75" />
    </Svg>
  );
}

export default Users;
