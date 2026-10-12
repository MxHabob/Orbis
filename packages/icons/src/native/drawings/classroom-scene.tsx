import React from 'react';
import Svg, { Path, Rect, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface ClassroomSceneProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function ClassroomScene({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: ClassroomSceneProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Rect x="3" y="2.5" width="18" height="11" rx="1.2" />
      <Path d="M5 4.5h14v7H5zM8 20.5h8m-4-7v7M6 16h4v2H6zm8 0h4v2h-4z" />
      <Path d="m8 9 2-2 2 2 2-3 2 3m-9 7v-1.5m9 1.5v-1.5M4 22l2-4m14 4-2-4" />
    </Svg>
  );
}

export default ClassroomScene;
