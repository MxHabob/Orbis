import React from 'react';
import Svg, { Circle, Path, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface StudentGirlProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function StudentGirl({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: StudentGirlProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Path d="M5 12.5c-.8-5.9 2-10 7-10s7.8 4.1 7 10l-1.2 2.7-1.2-1.8H7.4l-1.2 1.8L5 12.5Z" />
      <Path d="M7.1 10.5a4.9 4.9 0 0 1 9.8 0v4.1a4.9 4.9 0 0 1-9.8 0v-4.1Zm2.2-3.1c2.1.2 4-.4 5.4-1.8m-6.3 7h.1m6.8 0h.1m-4.3 3.1c1 .8 2 .8 3 0" />
      <Circle cx="9.5" cy="12.6" r=".35" fill={color} />
      <Circle cx="14.5" cy="12.6" r=".35" fill={color} />
      <Path d="M3 22c.6-3.6 3.5-5.5 9-5.5s8.4 1.9 9 5.5H3Z" />
      <Path d="m9 17 3 3 3-3m-3 3v2M5.1 7.3l-2-1.5m15.8 1.5 2-1.5" />
    </Svg>
  );
}

export default StudentGirl;
