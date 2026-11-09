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

export interface SparklesProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Sparkles({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: SparklesProps) {
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
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M9 4 10.8 9.2 16 11l-5.2 1.8L9 18l-1.8-5.2L2 11l5.2-1.8L9 4Zm9-1.5 1.1 3.5L22.5 7l-3.4 1-1.1 3.5-1.1-3.5L13.5 7l3.4-1L18 2.5Zm-1 13 0.8 3.2 3.2 0.8-3.2 0.8L17 24l-0.8-3.2L13 20l3.2-0.8L17 15.5Z"
      />
    </Svg>
  );
}

export default Sparkles;
