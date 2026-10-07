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

export interface PencilProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Pencil({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: PencilProps) {
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
    
      strokeLinecap="round"
      strokeLinejoin="round">
      <Path
		strokeLinecap="round"
		strokeLinejoin="round"
		d="m4.1 16.9 12.2-12.2a2.4 2.4 0 0 1 3.4 3.4L7.5 20.3 2.5 21.5l1.6-4.6Zm11-11 3.4 3.4"
	/>
    </Svg>
  );
}

export default Pencil;
