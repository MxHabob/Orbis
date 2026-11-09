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

export interface FaceProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Face({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: FaceProps) {
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
        d="M12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2Z"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path d="M16 8L16 10" strokeLinecap="round" strokeLinejoin="round"></Path>

      <Path d="M8 8L8 10" strokeLinecap="round" strokeLinejoin="round"></Path>

      <Path
        d="M9 16C9 16 10 17 12 17C14 17 15 16 15 16"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>

      <Path
        d="M12 8L12 13L11 13"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
    </Svg>
  );
}

export default Face;
