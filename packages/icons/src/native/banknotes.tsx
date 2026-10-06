import React from 'react';
import Svg, {
  Path,
  type SvgProps,
} from 'react-native-svg';
import type { StyleProp, ViewStyle } from 'react-native';

export interface BanknotesProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function Banknotes({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: BanknotesProps) {
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
		strokeLinecap="round"
		strokeLinejoin="round"
		d="M2.25 18.75a60.07 60.07 0 0 1 15.797 2.101c.727.198 1.453-.342 1.453-1.096V18.75M3.75 4.5v.75A.75.75 0 0 1 3 6H2.25m0 0v8.25m0 0H3a.75.75 0 0 1 .75.75v.75M2.25 15a3.75 3.75 0 0 0 3.75 3.75H18m0 0h.75A.75.75 0 0 0 19.5 18v-.75M18 18.75a3.75 3.75 0 0 0 3.75-3.75V6a3.75 3.75 0 0 0-3.75-3.75H5.25A3.75 3.75 0 0 0 1.5 6v9a3.75 3.75 0 0 0 3.75 3.75M12 9a3 3 0 1 0 0 6 3 3 0 0 0 0-6Z"
	 />
    </Svg>
  );
}

export default Banknotes;
