import React from 'react';
import Svg, { Path, Rect, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface MathBoardProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function MathBoard({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: MathBoardProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Rect x="2.5" y="3.5" width="19" height="14.5" rx="1.8" />
      <Path d="M4.5 5.5h15v10.5h-15zM8 20.5h8m-4-2.5v2.5" />
      <Path d="m7 11 1.5 1.5 1.2-4h2.1m1.8 3.9 2.7-2.7m0 2.7-2.7-2.7" />
      <Path d="M5 19h2l.6-1h8.8l.6 1h2" />
    </Svg>
  );
}

export default MathBoard;
