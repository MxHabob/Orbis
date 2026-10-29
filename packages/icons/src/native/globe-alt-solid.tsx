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

export interface GlobeAltSolidProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function GlobeAltSolid({
  size = 16,
  color = "currentColor",
  strokeWidth = 1.75,
  style,
  ...props
}: GlobeAltSolidProps) {
  return (
    <Svg
      viewBox="0 0 24 24"
      width={size}
      height={size}
      fill={color}
      stroke={color}
      strokeWidth={strokeWidth}
      style={style}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden={true}
      {...props}
    >
      <Path
        fillRule="evenodd"
        stroke="none"
        d="M12 2.75a9.25 9.25 0 1 0 0 18.5a9.25 9.25 0 0 0 0-18.5ZM12 4.5c.98 1.25 1.48 3.45 1.45 6.25h-2.9C10.52 7.95 11.02 5.75 12 4.5ZM10.55 13.25h2.9c.03 2.8-.47 5-1.45 6.25c-.98-1.25-1.48-3.45-1.45-6.25ZM3.4 11c1.9.7 4.1 1 6.4 1v1.5c-2.3 0-4.5-.3-6.4-1V11Zm11 1c2.3 0 4.5-.3 6.2-1v1.5c-1.7.7-3.9 1-6.2 1V12Z"
      />
    </Svg>
  );
}

export default GlobeAltSolid;
