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

export interface XMarkProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function XMark({
  size = 20,
  color = 'currentColor',
  strokeWidth = 1.5,
  style,
  ...props
}: XMarkProps) {
  return (
    <Svg
      viewBox="0 0 20 20"
      width={size}
      height={size}
      fill={color}
      stroke={color}
      strokeWidth={strokeWidth}
      style={style}
      aria-hidden={true}
      {...props}
    >
      <slot />
	<Path
		d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z"
	/>
    </Svg>
  );
}

export default XMark;
