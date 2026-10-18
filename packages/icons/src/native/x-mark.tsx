import type { StyleProp, ViewStyle } from 'react-native';
import Svg, {
  Path,
  type SvgProps,
} from 'react-native-svg';

export interface XMarkProps extends SvgProps {
  size?: number;
  color?: string;
  strokeWidth?: number;
  style?: StyleProp<ViewStyle>;
}

export function XMark({
  size = 16,
  color = 'currentColor',
  strokeWidth = 1.75,
  style,
  ...props
}: XMarkProps) {
  return (
    <Svg
      viewBox="0 0 20 20"
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
      <Path d="m5.75 5.75 8.5 8.5m0-8.5-8.5 8.5" />
    </Svg>
  );
}

export default XMark;
