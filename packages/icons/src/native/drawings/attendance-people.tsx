import React from 'react';
import Svg, { Circle, Path, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface AttendancePeopleProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function AttendancePeople({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: AttendancePeopleProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Circle cx="12" cy="5" r="2.5" />
      <Circle cx="5" cy="9" r="2" />
      <Circle cx="19" cy="9" r="2" />
      <Circle cx="7.5" cy="5.5" r="1.5" />
      <Circle cx="16.5" cy="5.5" r="1.5" />
      <Path d="M6.5 20v-2.5a5.5 5.5 0 0 1 11 0V20H6.5Zm-1-8.8a3.5 3.5 0 0 0-3 3.5V18h3m13 0h3v-3.3a3.5 3.5 0 0 0-3-3.5m-7.4 1.3a4.5 4.5 0 0 0-3.2-1.3m7.2 0a4.5 4.5 0 0 1 3.2 1.3" />
    </Svg>
  );
}

export default AttendancePeople;
