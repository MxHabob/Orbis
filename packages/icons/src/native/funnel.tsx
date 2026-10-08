import React from 'react';
import Svg, {
  Path,
  type SvgProps,
} from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface FunnelProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Funnel({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: FunnelProps) {
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
      <Path
		strokeLinecap="round"
		strokeLinejoin="round"
		d="M12 3c2.755 0 5.455.232 8.083.678.539.092.917.56.917 1.108v1.396a2.25 2.25 0 0 1-.659 1.591L15 13.064V19.5a2.25 2.25 0 0 1-.879 1.795l-3 2.25A2.25 2.25 0 0 1 9.75 22.5v-9.436L4.659 7.773A2.25 2.25 0 0 1 4 6.182V4.786c0-.548.378-1.016.917-1.108A48.507 48.507 0 0 1 12 3Z"
	 />
    </Svg>
  );
}

export default Funnel;
