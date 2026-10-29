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

export interface DatabaseSettingsProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function DatabaseSettings({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: DatabaseSettingsProps) {
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
        d="M4 6V12C4 12 4 15 11 15C18 15 18 12 18 12V6"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M11 3C18 3 18 6 18 6C18 6 18 9 11 9C4 9 4 6 4 6C4 6 4 3 11 3Z"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M11 21C4 21 4 18 4 18V12"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M19 21C20.1046 21 21 20.1046 21 19C21 17.8954 20.1046 17 19 17C18.6357 17 18.2942 17.0974 18 17.2676C17.4022 17.6134 17 18.2597 17 19C17 19.7403 17.4022 20.3866 18 20.7324C18.2942 20.9026 18.6357 21 19 21Z"
        strokeLinecap="round"
        strokeLinejoin="round"
      ></Path>
      <Path
        d="M19 22C20.6569 22 22 20.6569 22 19C22 17.3431 20.6569 16 19 16C17.3431 16 16 17.3431 16 19C16 20.6569 17.3431 22 19 22Z"
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeDasharray="0.3 2"
      ></Path>
    </Svg>
  );
}

export default DatabaseSettings;
