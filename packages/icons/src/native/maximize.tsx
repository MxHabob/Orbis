import React from "react";
import Svg, { Rect } from "react-native-svg";
import type { SvgProps } from "react-native-svg";

export interface IconProps extends SvgProps {
  size?: number;
  strokeWidth?: number;
}

export const Maximize: React.FC<IconProps> = ({
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
    <Rect x="4" y="4" width="16" height="16" rx="2" />
  </Svg>
);
export default Maximize;
