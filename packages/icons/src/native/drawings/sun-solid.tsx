import React from 'react';
import Svg, { Circle, Path, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface SunSolidProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function SunSolid({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: SunSolidProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Circle cx="12" cy="12" r="4.2" fill={color} />
      <Path d="M12 2v2m0 16v2M4.93 4.93l1.42 1.42m11.3 11.3 1.42 1.42M2 12h2m16 0h2M4.93 19.07l1.42-1.42m11.3-11.3 1.42-1.42" />
    </Svg>
  );
}

export default SunSolid;
