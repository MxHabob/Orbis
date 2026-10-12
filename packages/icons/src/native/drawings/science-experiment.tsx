import React from 'react';
import Svg, { Circle, Path, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface ScienceExperimentProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function ScienceExperiment({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: ScienceExperimentProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Path d="M7.8 3.1c2.3 2.4 2.3 5.4 0 8.1-2.3 2.7-5.3 2.9-7 .5m2.1-7.9c2.4-.2 4.7 1.6 6 4.7 1.3 3.1.8 6.1-1.4 7.4M1 8.7c3.1-1.2 6-.4 8.1 2.2 2.1 2.6 2.1 5.6-.1 7.6" />
      <Circle cx="4.7" cy="11.3" r="1.2" />
      <Path d="M15.6 3.5h5m-4 .1v5.1l-4.1 9.1a1.7 1.7 0 0 0 1.5 2.4h5.6a1.7 1.7 0 0 0 1.5-2.4L17 8.7" />
      <Path d="M14.2 15.1c1.8.8 3.6.8 5.5 0m-4.6 2.3.1.1m2.9-1.4.1.1" />
    </Svg>
  );
}

export default ScienceExperiment;
