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

export interface LineSpaceSmallerProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function LineSpaceSmaller({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: LineSpaceSmallerProps) {
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
        d="M5 19V5M5 10L3 8M5 10L7 8M5 14L3 16M5 14L7 16"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
    </Svg>
  );
}

export default LineSpaceSmaller;
