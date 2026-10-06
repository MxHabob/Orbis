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

export interface ArrowForwardProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function ArrowForward({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: ArrowForwardProps) {
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
      <Path strokeLinecap="round" strokeLinejoin="round" d="M13.25 19.25L16.75 15.75L13.25 12.25" />
	<Path
		strokeLinecap="round"
		strokeLinejoin="round"
		d="M16.75 15.75H10.75C8.54086 15.75 6.75 13.9591 6.75 11.75V7.5"
	/>
    </Svg>
  );
}

export default ArrowForward;
