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

export interface BookOpenProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function BookOpen({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: BookOpenProps) {
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
		strokeLinecap="round"
		strokeLinejoin="round"
		d="M12 6.5c-2.5-1.75-5.5-2.25-8.5-1.5v13c3-.75 6-.25 8.5 1.5m0-13c2.5-1.75 5.5-2.25 8.5-1.5v13c-3-.75-6-.25-8.5 1.5m0-13v13"
	/>
    </Svg>
  );
}

export default BookOpen;
