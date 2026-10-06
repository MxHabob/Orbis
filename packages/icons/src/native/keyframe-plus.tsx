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

export interface KeyframePlusProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function KeyframePlus({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: KeyframePlusProps) {
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
      <Path d="M16 5H19M22 5H19M19 5V2M19 5V8" />
	<Path
		d="M16.8189 14.3287L11.4948 20.3183C10.6992 21.2134 9.30076 21.2134 8.50518 20.3183L3.18109 14.3287C2.50752 13.571 2.50752 12.429 3.18109 11.6713L8.50518 5.68167C9.30076 4.78664 10.6992 4.78664 11.4948 5.68167L16.8189 11.6713C17.4925 12.429 17.4925 13.571 16.8189 14.3287Z"
	/>
    </Svg>
  );
}

export default KeyframePlus;
