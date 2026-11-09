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

export interface FaceIdProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function FaceId({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: FaceIdProps) {
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
        d="M7 3H5C3.89543 3 3 3.89543 3 5V7"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M17 3H19C20.1046 3 21 3.89543 21 5V7"
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
      <Path
        d="M7 21H5C3.89543 21 3 20.1046 3 19V17"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M17 21H19C20.1046 21 21 20.1046 21 19V17"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
    </Svg>
  );
}

export default FaceId;
