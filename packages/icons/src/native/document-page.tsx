import React from 'react';
import Svg, {
  Path,
  Circle,
  Rect,
  Line,
  Polyline,
  Polygon,
  G,
  Defs,
  ClipPath,
  Mask,
  Use,
  Symbol,
  LinearGradient,
  RadialGradient,
  Stop,
  type SvgProps,
} from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface DocumentPageProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function DocumentPage({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: DocumentPageProps) {
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
      <Path d="M6.75 3.5H13l5 5v11.25a1.25 1.25 0 0 1-1.25 1.25h-10a1.25 1.25 0 0 1-1.25-1.25V4.75A1.25 1.25 0 0 1 6.75 3.5Z" />
      <Path d="M13 3.75V8.5h4.75M9 12h6m-6 3.5h6M9 19h3.5" />
    </Svg>
  );
}

export default DocumentPage;
