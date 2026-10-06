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
      aria-hidden={true}
      {...props}
    >
      <Path
		strokeLinecap="round"
		strokeLinejoin="round"
		d="M12 6.5c-2.5-2-5.5-2.75-8.5-2v14c3-1 6-.25 8.5 2m0-14c2.5-2 5.5-2.75 8.5-2v14c-3-1-6-.25-8.5 2m0-14v14"
	/>
    </Svg>
  );
}

export default BookOpen;
