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

export interface MicProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Mic({
  size = 20,
  color = 'currentColor',
  strokeWidth = strokeWidth,
  style,
  ...props
}: MicProps) {
  return (
    <Svg
      viewBox="0 0 24 24"
      width={size}
      height={size}
      fill="none"
      stroke={color}
      strokeWidth={strokeWidth}
      style={style}
      aria-hidden={true}
      {...props}
    >
      <Rect x="9" y="2" width="6" height="12" rx="3" />
	<Path
		strokeLinecap="round"
		strokeLinejoin="round"
		d="M5 10v1a7 7 0 0 0 14 0v-1M12 18v4m0 0H9m3 0h3"
	/>
    </Svg>
  );
}

export default Mic;
