import React from "react";
import Svg, { Path, Circle, type SvgProps } from "react-native-svg";
import type { StyleProp, ViewStyle } from "react-native";

export interface AiCenterProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function AiCenter({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: AiCenterProps) {
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
      <Path d="M7 3.75h10A3.25 3.25 0 0 1 20.25 7v8A3.25 3.25 0 0 1 17 18.25h-5L7.5 21v-2.75A3.25 3.25 0 0 1 3.75 15V7A3.25 3.25 0 0 1 7 3.75Z" />
      <Path d="m12 7.25.95 2.8 2.8.95-2.8.95-.95 2.8-.95-2.8-2.8-.95 2.8-.95.95-2.8Z" />
      <Circle cx="6.75" cy="7.25" r=".45" fill={color} stroke="none" />
      <Circle cx="17.25" cy="14.75" r=".45" fill={color} stroke="none" />
    </Svg>
  );
}

export default AiCenter;
