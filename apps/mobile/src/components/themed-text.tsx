import { Platform, StyleSheet, Text, type TextProps } from 'react-native';

import { Fonts, Typography, type ThemeColor } from '@/constants/theme';
import { useTheme } from '@/hooks/use-theme';

export type ThemedTextProps = TextProps & {
  type?: 'default' | 'title' | 'small' | 'smallBold' | 'subtitle' | 'link' | 'linkPrimary' | 'code';
  themeColor?: ThemeColor;
};

export function ThemedText({ style, type = 'default', themeColor, ...rest }: ThemedTextProps) {
  const theme = useTheme();

  const getTextColor = () => {
    if (themeColor) return theme[themeColor];
    if (type === 'linkPrimary') return theme.primary;
    return theme.text;
  };

  return (
    <Text
      style={[
        { color: getTextColor(), fontFamily: Fonts?.sans },
        type === 'default' && styles.default,
        type === 'title' && styles.title,
        type === 'small' && styles.small,
        type === 'smallBold' && styles.smallBold,
        type === 'subtitle' && styles.subtitle,
        type === 'link' && styles.link,
        type === 'linkPrimary' && styles.linkPrimary,
        type === 'code' && styles.code,
        style,
      ]}
      {...rest}
    />
  );
}

const styles = StyleSheet.create({
  small: {
    ...Typography.caption,
    fontWeight: '500',
  },
  smallBold: {
    ...Typography.captionBold,
    fontWeight: '700',
  },
  default: {
    ...Typography.body,
  },
  title: {
    ...Typography.h1,
  },
  subtitle: {
    ...Typography.h2,
  },
  link: {
    ...Typography.body,
  },
  linkPrimary: {
    ...Typography.bodyBold,
  },
  code: {
    fontFamily: Fonts?.mono,
    fontWeight: Platform.select({ android: '700' }) ?? '500',
    fontSize: 12,
  },
});
