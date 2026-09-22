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

export interface ChevronUpDownProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function ChevronUpDown({
  size = 20,
  color = 'currentColor',
  strokeWidth = strokeWidth,
  style,
  ...props
}: ChevronUpDownProps) {
  return (
    <Svg
      viewBox="0 0 24 24"
      width={size}
      height={size}
      fill="none"
      stroke={color}
      strokeWidth={strokeWidth}
      style={style}
      aria-hidden={true}
      {...props}
    >
      <Path
		strokeLinecap="round"
		strokeLinejoin="round"
		d="M8.25 15 12 18.75 15.75 15m-7.5-6L12 5.25 15.75 9"
	/>
    </Svg>
  );
}

export default ChevronUpDown;
