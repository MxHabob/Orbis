import React from 'react';
import Svg, { Path, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface ReadingBookProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function ReadingBook({ size = 16, color = 'currentColor', strokeWidth = 1.75, style, ...props }: ReadingBookProps) {
  return (
    <Svg viewBox="0 0 24 24" width={size} height={size} fill="none" stroke={color} strokeWidth={strokeWidth} style={style} strokeLinecap="round" strokeLinejoin="round" aria-hidden={true} {...props}>
      <Path d="M3 5.1c3.5-1.2 6.4-.7 9 1.4 2.6-2.1 5.5-2.6 9-1.4v13c-3.5-1.2-6.4-.7-9 1.4-2.6-2.1-5.5-2.6-9-1.4v-13Z" />
      <Path d="M12 6.5v13M5.3 8.1c1.8-.4 3.2-.1 4.6.7m-4.6 2c1.8-.4 3.2-.1 4.6.7m-4.6 2c1.8-.4 3.2-.1 4.6.7m4.1-5.4c1.4-.8 2.8-1.1 4.6-.7m-4.6 2.7c1.4-.8 2.8-1.1 4.6-.7m-4.6 2.7c1.4-.8 2.8-1.1 4.6-.7" />
      <Path d="M2 6.8v12.4c3.6-1.1 6.8-.6 10 1.6 3.2-2.2 6.4-2.7 10-1.6V6.8" />
    </Svg>
  );
}

export default ReadingBook;
