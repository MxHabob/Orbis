import React from 'react';
import Svg, { Path, Rect, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface StudentBackpackProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function StudentBackpack({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: StudentBackpackProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Path d="M8 6.5V5a4 4 0 0 1 8 0v1.5m-8 0A3.5 3.5 0 0 0 4.5 10v9A2.5 2.5 0 0 0 7 21.5h10a2.5 2.5 0 0 0 2.5-2.5v-9A3.5 3.5 0 0 0 16 6.5H8Z" />
      <Path d="M4.7 11h14.6m-13.8 0v7.5a1.5 1.5 0 0 0 1.5 1.5h9.9a1.5 1.5 0 0 0 1.5-1.5V11" />
      <Rect x="7.2" y="13" width="9.6" height="5" rx="1.3" />
      <Path d="M10.5 13v-1m3 1v-1m-1.5 2.4.3.7.8.1-.6.5.1.8-.6-.4-.7.4.2-.8-.6-.5.8-.1.3-.7ZM3 12l1.5 1m16.5-1-1.5 1" />
    </Svg>
  );
}

export default StudentBackpack;
