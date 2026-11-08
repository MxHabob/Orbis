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

export interface LineSpaceProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function LineSpace({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: LineSpaceProps) {
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
      <Path d="M11 6H21" strokeLinecap="round" strokeLinejoin="round"></Path>
      <Path d="M11 12H21" strokeLinecap="round" strokeLinejoin="round"></Path>
      <Path d="M11 18H21" strokeLinecap="round" strokeLinejoin="round"></Path>
      <Path
        d="M5 19V5M5 19L3 16.5M5 19L7 16.5M5 5L3 7M5 5L7 7"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
    </Svg>
  );
}

export default LineSpace;
