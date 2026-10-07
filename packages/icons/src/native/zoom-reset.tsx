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

export interface ZoomResetProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function ZoomReset({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: ZoomResetProps) {
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
      <Rect x="2" y="4" width="20" height="16" rx="3" fill="none" />
	
	<Path strokeLinecap="round" strokeLinejoin="round" d="M7.5 9v6M6 10.5l1.5-1.5" />
	
	<Path
		strokeLinecap="round"
		strokeLinejoin="round"
		d="M12 10.5v.01M12 13.5v.01"
		strokeWidth="2"
	/>
	
	<Path strokeLinecap="round" strokeLinejoin="round" d="M16.5 9v6M15 10.5l1.5-1.5" />
    </Svg>
  );
}

export default ZoomReset;
