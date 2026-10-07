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

export interface DocumentArrowUpSolidProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function DocumentArrowUpSolid({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: DocumentArrowUpSolidProps) {
  return (
    <Svg
      viewBox="0 0 24 24"
      width={size}
      height={size}
      fill={color}
      stroke={color}
      strokeWidth={strokeWidth}
      style={style}
      aria-hidden={true}
      {...props}
    
      strokeLinecap="round"
      strokeLinejoin="round">
      <Path
		fillRule="evenodd"
		stroke="none"
		d="M6.25 2.5h7.3l5.95 5.95v11.8A1.75 1.75 0 0 1 17.75 22h-11A1.75 1.75 0 0 1 5 20.25V3.75A1.25 1.25 0 0 1 6.25 2.5ZM14 4.05v3.2c0 .55.45 1 1 1h3.2L14 4.05ZM12 10.2l-4.2 4.2h2.7v3.8h3v-3.8h2.7L12 10.2Z"
	/>
    </Svg>
  );
}

export default DocumentArrowUpSolid;
