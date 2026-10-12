import React from 'react';
import Svg, { Path, Rect, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface CalculatorWarningProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function CalculatorWarning({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: CalculatorWarningProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Path d="m2.2 18.6 5.5-12a1.2 1.2 0 0 1 2.2 0l5.4 12a1.2 1.2 0 0 1-1.1 1.7H3.3a1.2 1.2 0 0 1-1.1-1.7Z" />
      <Path d="m7.1 11 2.7 4.8m0-4.8-2.7 4.8m1.35-8v1" />
      <Rect x="14.5" y="4" width="7" height="16" rx="1.4" />
      <Rect x="16.1" y="5.8" width="3.8" height="2.6" rx=".3" />
      <Path d="M16.4 11.2h.1m2.7 0h.1m-2.9 2.6h.1m2.7 0h.1m-2.9 2.6h.1m2.7 0h.1" />
    </Svg>
  );
}

export default CalculatorWarning;
