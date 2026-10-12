import React from 'react';
import Svg, { Path, Rect, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface TaskCalendarProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function TaskCalendar({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: TaskCalendarProps) {
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
      <Path d="M7.5 6V3.75m4.5 1.85V3.35m4.5 1.85V3" />
      <Path d="m5.7 5.65 13.55-1.2a1.5 1.5 0 0 1 1.62 1.36l1.02 11.48a1.5 1.5 0 0 1-1.36 1.62L7 20.1a1.5 1.5 0 0 1-1.62-1.36L4.36 7.27A1.5 1.5 0 0 1 5.7 5.65Z" />
      <Path d="m5.05 9.1 16.26-1.43" />
      <Rect x="7.3" y="11" width="2.5" height="2.5" rx=".35" />
      <Rect x="11.2" y="10.65" width="2.5" height="2.5" rx=".35" />
      <Rect x="7.65" y="15.1" width="2.5" height="2.5" rx=".35" />
      <Path d="m13.4 15.05 1.65 1.55 3.15-4.15" />
    </Svg>
  );
}

export default TaskCalendar;
