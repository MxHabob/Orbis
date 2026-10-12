import React from 'react';
import Svg, { Path, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface BookStackProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function BookStack({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: BookStackProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Path d="m3 8.2 12.8-3 5.2 2.4-12.8 3L3 8.2Z" />
      <Path d="M3 8.2v2.7l5.2 2.3 12.8-3V7.6M4.2 13.2l11.6-2.7 4.9 2.3-11.6 2.7-4.9-2.3Z" />
      <Path d="M4.2 13.2v2.5l4.9 2.4 11.6-2.7v-2.6m-13 5.5 1.4 1.1 11.4-2.6M6.6 8.9v2.7m.9 3.1v2.5" />
    </Svg>
  );
}

export default BookStack;
