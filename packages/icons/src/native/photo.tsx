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

export interface PhotoProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Photo({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: PhotoProps) {
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
        d="M21 7.6V20.4C21 20.7314 20.7314 21 20.4 21H7.6C7.26863 21 7 20.7314 7 20.4V7.6C7 7.26863 7.26863 7 7.6 7H20.4C20.7314 7 21 7.26863 21 7.6Z"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M18 4H4.6C4.26863 4 4 4.26863 4 4.6V18"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M7 16.8L12.4444 15L21 18"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M16.5 13C15.6716 13 15 12.3284 15 11.5C15 10.6716 15.6716 10 16.5 10C17.3284 10 18 10.6716 18 11.5C18 12.3284 17.3284 13 16.5 13Z"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
    </Svg>
  );
}

export default Photo;
