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

export interface GlassesProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Glasses({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: GlassesProps) {
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
      
	<Path
		d="M2 14
		   C2 16.2091 3.79086 18 6 18
		   C8.20914 18 10 16.2091 10 14
		   C10 11.7909 8.20914 10 6 10
		   C3.79086 10 2 11.7909 2 14Z"
		strokeLinecap="round"
		strokeLinejoin="round"
	/>

	
	<Path
		d="M14 14
		   C14 16.2091 15.7909 18 18 18
		   C20.2091 18 22 16.2091 22 14
		   C22 11.7909 20.2091 10 18 10
		   C15.7909 10 14 11.7909 14 14Z"
		strokeLinecap="round"
		strokeLinejoin="round"
	/>

	
	<Path d="M2 14L0.8 14.4" strokeLinecap="round" />

	
	<Path d="M22 14L23.2 14.4" strokeLinecap="round" />

	
	<Path d="M14 14H10" strokeLinecap="round" strokeLinejoin="round" />
    </Svg>
  );
}

export default Glasses;
