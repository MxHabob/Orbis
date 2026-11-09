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

export interface UndoActionProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function UndoAction({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: UndoActionProps) {
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
        d="M5 5V8V11"
        stroke="currentColor"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <Path
        d="M8.50005 8C12.5001 8 11.0001 8 15.0001 8C15.0001 8 15.0001 8 15.0001 8C15.0001 8 20.0001 8 20.0001 12.7059C20.0001 18 15.0001 18 15.0001 18C11.5715 18 9.71434 18 6.28577 18"
        stroke="currentColor"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <Path
        d="M11.5 11.5C10.1332 10.1332 9.36683 9.36683 8 8C9.36683 6.63317 10.1332 5.86683 11.5 4.5"
        stroke="currentColor"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </Svg>
  );
}

export default UndoAction;
