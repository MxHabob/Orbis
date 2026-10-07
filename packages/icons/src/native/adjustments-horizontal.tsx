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

export interface AdjustmentsHorizontalProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function AdjustmentsHorizontal({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: AdjustmentsHorizontalProps) {
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
    
      strokeLinecap="round"
      strokeLinejoin="round">
      <Path
		strokeLinecap="round"
		strokeLinejoin="round"
		d="M3.5 6.5h4m3 0h10M3.5 12h9m3 0h5m-17 5.5h3m3 0h11"
	/>
      <Circle cx="9" cy="6.5" r="1.5" />
      <Circle cx="14" cy="12" r="1.5" />
      <Circle cx="8" cy="17.5" r="1.5" />
    </Svg>
  );
}

export default AdjustmentsHorizontal;
