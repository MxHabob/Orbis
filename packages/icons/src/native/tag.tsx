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

export interface TagProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Tag({
  size = 20,
  color = 'currentColor',
  strokeWidth = strokeWidth,
  style,
  ...props
}: TagProps) {
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
      <!-- Tag body with pointed end -->
	<Path d="M4 12 L8 7 H21 V17 H8 L4 12 Z" stroke="currentColor" fill="none" />

	<!-- Tag hole -->
	<Circle cx="10" cy="12" r="0.75" fill="currentColor" stroke="currentColor" />
    </Svg>
  );
}

export default Tag;
