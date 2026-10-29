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

export interface InfoCircleProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function InfoCircle({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: InfoCircleProps) {
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
      <Circle cx="12" cy="12" r="9.25" />
      <Path d="M12 10.75v5.5m0-8.5h.01" />
    </Svg>
  );
}

export default InfoCircle;
