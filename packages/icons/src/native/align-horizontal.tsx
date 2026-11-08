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

export interface AlignHorizontalProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function AlignHorizontal({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: AlignHorizontalProps) {
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
      <Path d="M3 22L3 2" strokeLinecap="round" strokeLinejoin="round"></Path>
      <Path d="M21 22V2" strokeLinecap="round" strokeLinejoin="round"></Path>
      <Path d="M15 16H9C7.89543 16 7 15.1046 7 14V10C7 8.89543 7.89543 8 9 8H15C16.1046 8 17 8.89543 17 10V14C17 15.1046 16.1046 16 15 16Z"></Path>
    </Svg>
  );
}

export default AlignHorizontal;
