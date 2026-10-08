import React from 'react';
import Svg, { Path, type SvgProps } from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface TemplateLibraryProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function TemplateLibrary({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: TemplateLibraryProps) {
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
      <Path d="M6 3.25h8l4.75 4.5v12A1.25 1.25 0 0 1 17.5 21h-11A1.25 1.25 0 0 1 5.25 19.75V4.5A1.25 1.25 0 0 1 6.5 3.25Z" />
      <Path d="M14 3.75v4.5h4.25M8.25 11.5h3v3h-3zm5 0h2.5v3h-2.5zm-5 5h3v2h-3zm5 0h2.5v2h-2.5z" />
    </Svg>
  );
}

export default TemplateLibrary;
