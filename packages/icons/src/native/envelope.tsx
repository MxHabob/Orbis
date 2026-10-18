import React from 'react';
import Svg, { Path, Rect, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface EnvelopeProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Envelope({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: EnvelopeProps) {
  return (
    <Svg
      viewBox="0 0 24 24"
      width={size}
      height={size}
      fill="none"
      stroke={color}
      strokeWidth={strokeWidth}
      style={style}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden={true}
      {...props}>
      <Rect x="3" y="5" width="18" height="14" rx="2" />
      <Path d="m3 7 9 6 9-6" />
    </Svg>
  );
}

export default Envelope;
