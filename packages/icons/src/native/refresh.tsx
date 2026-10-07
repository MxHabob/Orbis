import React from 'react';
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
} from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface RefreshProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Refresh({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: RefreshProps) {
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
      {...props}>
      <Path
		d="M21.8883 13.5C21.1645 18.3113 17.013 22 12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C16.1006 2 19.6248 4.46819 21.1679 8"
		strokeLinecap="round"
		strokeLinejoin="round"
	></Path><Path
		d="M17 8H21.4C21.7314 8 22 7.73137 22 7.4V3"
		strokeLinecap="round"
		strokeLinejoin="round"
	></Path>
    </Svg>
  );
}

export default Refresh;
