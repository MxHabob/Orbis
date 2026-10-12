import React from 'react';
import Svg, { Path, Rect, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface StudyMaterialsProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function StudyMaterials({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: StudyMaterialsProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Path d="m3 17.2 11.8-2.4 5.2 2.1-11.8 2.5L3 17.2Z" />
      <Path d="m3 17.2.2 2.2 5 2.1 11.8-2.5v-2.1" />
      <Path d="m4 14.2 10.9-2.3 4.8 2-10.9 2.3L4 14.2Z" />
      <Path d="m4 14.2.2 2.2 4.6 1.8 10.9-2.3v-2" />
      <Path d="m5.1 11.5 9.8-2.1 4.4 1.8-9.8 2.1-4.4-1.8Z" />
      <Rect x="7" y="6" width="1.8" height="5" rx=".25" />
      <Path d="M10.3 5.2v5.4m2.2-6.3v6.1m2.2-7.2v7.8" />
      <Path d="m17.4 2.8 4.1-.5.7 5.4-4.1.5-.7-5.4Z" />
      <Path d="m18.5 4.1 2.1-.3m-2 1.6 1.4-.2m.3 1.6.1.5" />
    </Svg>
  );
}

export default StudyMaterials;
