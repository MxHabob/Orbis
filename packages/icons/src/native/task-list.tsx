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

export interface TaskListProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function TaskList({
  size = 20,
  color = 'currentColor',
  strokeWidth = 1.5,
  style,
  ...props
}: TaskListProps) {
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
		d="M9 6L20 6"
		stroke="currentColor"
		strokeWidth="1.5"
		strokeLinecap="round"
		strokeLinejoin="round"
	/>
	<Path
		d="M3.80002 5.79999L4.60002 6.59998L6.60001 4.59999"
		stroke="currentColor"
		strokeWidth="1.5"
		strokeLinecap="round"
		strokeLinejoin="round"
	/>
	<Path
		d="M3.80002 11.8L4.60002 12.6L6.60001 10.6"
		stroke="currentColor"
		strokeWidth="1.5"
		strokeLinecap="round"
		strokeLinejoin="round"
	/>
	<Path
		d="M3.80002 17.8L4.60002 18.6L6.60001 16.6"
		stroke="currentColor"
		strokeWidth="1.5"
		strokeLinecap="round"
		strokeLinejoin="round"
	/>
	<Path
		d="M9 12L20 12"
		stroke="currentColor"
		strokeWidth="1.5"
		strokeLinecap="round"
		strokeLinejoin="round"
	/>
	<Path
		d="M9 18L20 18"
		stroke="currentColor"
		strokeWidth="1.5"
		strokeLinecap="round"
		strokeLinejoin="round"
	/>
    </Svg>
  );
}

export default TaskList;
