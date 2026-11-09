import React from "react";
import Svg, { Rect, Path } from "react-native-svg";
import type { SvgProps } from "react-native-svg";

export interface IconProps extends SvgProps {
  size?: number;
  strokeWidth?: number;
}

export const Restore: React.FC<IconProps> = ({
  size = 16,
  strokeWidth = 1.75,
  color = "currentColor",
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
    <Rect x="3" y="7" width="14" height="14" rx="2" />
    <Path d="M7 4h12a2 2 0 0 1 2 2v12" />
  </Svg>
);
export default Restore;
