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

export interface AlignVerticalProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function AlignVertical({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: AlignVerticalProps) {
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
      <Path d="M22 3L2 3" strokeLinecap="round" strokeLinejoin="round"></Path><Path
		d="M22 21L2 21"
		strokeLinecap="round"
		strokeLinejoin="round"
	></Path><Path
		d="M8 15V9C8 7.89543 8.89543 7 10 7H14C15.1046 7 16 7.89543 16 9V15C16 16.1046 15.1046 17 14 17H10C8.89543 17 8 16.1046 8 15Z"
	></Path>
    </Svg>
  );
}

export default AlignVertical;
