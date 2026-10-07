import React from 'react';
import Svg, { Path } from 'react-native-svg';
import type { SvgProps } from 'react-native-svg';

export interface IconProps extends SvgProps {
	size?: number;
	strokeWidth?: number;
}

export const Bell: React.FC<IconProps> = ({
	size = 16,
	strokeWidth = 1.75,
	color = 'currentColor',
	...props
}) => (
	<Svg
		width={size}
		height={size}
		viewBox="0 0 24 24"
		fill="none"
		stroke={color}
		strokeWidth={strokeWidth}
      strokeLinecap="round"
      strokeLinejoin="round"
      {...props}>
		<Path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" />
		<Path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
	</Svg>
);
export const AppNotification = Bell;
export default Bell;
