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

export interface UserCircleSolidProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function UserCircleSolid({
  size = 20,
  color = 'currentColor',
  strokeWidth = 1.5,
  style,
  ...props
}: UserCircleSolidProps) {
  return (
    <Svg
      viewBox="0 0 16 16"
      width={size}
      height={size}
      fill={color}
      stroke={color}
      strokeWidth={strokeWidth}
      style={style}
      aria-hidden={true}
      {...props}
    >
      <Path
		fillRule="evenodd"
		d="M15 8A7 7 0 1 1 1 8a7 7 0 0 1 14 0Zm-5-2a2 2 0 1 1-4 0 2 2 0 0 1 4 0ZM8 9c-1.825 0-3.422.977-4.295 2.437A5.49 5.49 0 0 0 8 13.5a5.49 5.49 0 0 0 4.294-2.063A4.997 4.997 0 0 0 8 9Z"
		clipRule="evenodd"
	/>
    </Svg>
  );
}

export default UserCircleSolid;
