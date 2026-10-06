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

export interface VoiceProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Voice({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: VoiceProps) {
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
    >
      <Path d="M12 4L12 20" strokeLinecap="round" strokeLinejoin="round"></Path><Path
		d="M8 9L8 15"
		strokeLinecap="round"
		strokeLinejoin="round"
	></Path><Path d="M20 10L20 14" strokeLinecap="round" strokeLinejoin="round"></Path><Path
		d="M4 10L4 14"
		strokeLinecap="round"
		strokeLinejoin="round"
	></Path><Path d="M16 7L16 17" strokeLinecap="round" strokeLinejoin="round"></Path>
    </Svg>
  );
}

export default Voice;
