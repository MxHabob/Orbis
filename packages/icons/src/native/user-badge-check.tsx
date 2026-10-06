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

export interface UserBadgeCheckProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function UserBadgeCheck({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: UserBadgeCheckProps) {
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
      <Circle cx={7.5} cy={7.5} r={3.25} />
      <Path d="M1.75 20v-1.25a5.75 5.75 0 0 1 11.5 0V20H1.75Z" strokeLinecap="round" strokeLinejoin="round" />
      <Circle cx={18} cy={17} r={4.25} />
      <Path d="m16.25 17 1.2 1.2 2.35-2.5" strokeLinecap="round" strokeLinejoin="round" />
    </Svg>
  );
}

export default UserBadgeCheck;
