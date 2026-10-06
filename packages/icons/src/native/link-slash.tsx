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

export interface LinkSlashProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function LinkSlash({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: LinkSlashProps) {
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
		d="M7.14286 16.9953C6.75006 16.9953 6.36756 16.9525 6 16.8715C3.70973 16.3665 2 14.3761 2 11.9977C2 9.284 4.22573 7.07548 7 7.00195"
		strokeLinecap="round"
		strokeLinejoin="round"
	></Path><Path
		d="M13.3184 9.63429C12.7858 8.73635 11.9737 7.96977 11 7.4989"
		strokeLinecap="round"
		strokeLinejoin="round"
	></Path><Path
		d="M16.8571 6.99999C17.2499 6.99999 17.6324 7.04278 18 7.12383C20.2903 7.62884 22 9.6192 22 11.9976C22 14.7577 19.6975 16.9952 16.8571 16.9952C16.581 16.9952 15.4776 16.9952 15.1429 16.9952C12.317 16.9952 10 14.4893 10 11.9976C10 11.9976 10 11 10.5 10.5"
		strokeLinecap="round"
		strokeLinejoin="round"
	></Path><Path d="M3 3L21 21" strokeLinecap="round" strokeLinejoin="round"></Path>
    </Svg>
  );
}

export default LinkSlash;
