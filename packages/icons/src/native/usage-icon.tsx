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

export interface UsageIconProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function UsageIcon({
  size = 20,
  color = 'currentColor',
  strokeWidth = 1.5,
  style,
  ...props
}: UsageIconProps) {
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
      <Path d="M20 20H4V4" strokeLinecap="round" strokeLinejoin="round" />
	<Path d="M4 16.5L12 9L15 12L19.5 7.5" strokeLinecap="round" strokeLinejoin="round" />
    </Svg>
  );
}

export default UsageIcon;
