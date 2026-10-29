import React from "react";
import Svg, { Path, Circle } from "react-native-svg";
import type { SvgProps } from "react-native-svg";

export interface IconProps extends SvgProps {
  size?: number;
  strokeWidth?: number;
}

export const Sun: React.FC<IconProps> = ({
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
    <Circle cx="12" cy="12" r="4" />
    <Path d="M12 2v2m0 16v2M4.93 4.93l1.41 1.41m11.32 11.32l1.41 1.41M2 12h2m16 0h2M6.34 17.66l-1.41 1.41m14.14-14.14l-1.41 1.41" />
  </Svg>
);
export default Sun;
