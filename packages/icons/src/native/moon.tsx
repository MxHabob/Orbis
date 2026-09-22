import React from 'react';
import Svg, { Path } from 'react-native-svg';
import type { SvgProps } from 'react-native-svg';

export interface IconProps extends SvgProps {
	size?: number;
	strokeWidth?: number;
}

export const Moon: React.FC<IconProps> = ({
	size = 24,
	strokeWidth = 1.5,
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
		{...props}
	>
		<Path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
	</Svg>
);
export default Moon;
