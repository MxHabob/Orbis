import React from 'react';
import Svg, { Circle, Path, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface NotebookCheckProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function NotebookCheck({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: NotebookCheckProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Path d="M6 3.5h10.5l4 4v12A1.5 1.5 0 0 1 19 21H6a1.5 1.5 0 0 1-1.5-1.5V5A1.5 1.5 0 0 1 6 3.5Z" />
      <Path d="M16.5 3.8v4h3.8M8 8.5h4m-4 3h3.5m-3.5 3h2.5" />
      <Circle cx="16.5" cy="15.4" r="3.3" />
      <Path d="m15.1 15.4 1 1 1.9-2.1M3.3 5.5h2m-2 3h2m-2 3h2m-2 3h2m-2 3h2" />
    </Svg>
  );
}

export default NotebookCheck;
