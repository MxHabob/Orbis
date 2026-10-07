import React from 'react';
import Svg, { Path, Circle, Rect, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface EmployeeAffairsProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function EmployeeAffairs({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: EmployeeAffairsProps) {
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
      <Rect x="3.25" y="3.5" width="17.5" height="17" rx="2.75" />
      <Circle cx="8.5" cy="9" r="2.25" />
      <Path d="M5.5 16a3 3 0 0 1 6 0m2-5.75h4.25M13.5 14h4.25m-4.25 3.75h2.5" />
    </Svg>
  );
}

export default EmployeeAffairs;
