// components/header.tsx

import React from 'react';
import {
  Pressable,
  StyleProp,
  StyleSheet,
  Text,
  View,
  ViewStyle,
  useWindowDimensions,
} from 'react-native';
import { useRouter } from 'expo-router';
import { Ionicons, MaterialCommunityIcons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import {
  Colors,
  Fonts,
  Radii,
  Spacing,
  Typography,
} from '@/constants/theme';
import { useTheme } from '@/hooks/use-theme';

type HeaderProps = {
  title?: string;
  subtitle?: string;

  showBack?: boolean;
  onBack?: () => void;

  onProfilePress?: () => void;
  onCrownPress?: () => void;

  rightAction?: React.ReactNode;

  /**
   * Kept for compatibility with existing screens.
   * Progress is no longer displayed in the header.
   */
  progress?: number;
  progressTotal?: number;
  onProgressPress?: () => void;

  /**
   * Show the center page title.
   */
  showTitle?: boolean;

  /**
   * Show crown on the right.
   */
  showCrown?: boolean;

  transparent?: boolean;

  style?: StyleProp<ViewStyle>;
};

export function Header({
  title,
  subtitle,

  showBack = false,
  onBack,

  onProfilePress,
  onCrownPress,

  rightAction,

  showTitle = true,
  showCrown = true,

  style,
}: HeaderProps) {
  const router = useRouter();
  const theme = useTheme();
  const insets = useSafeAreaInsets();
  const { width } = useWindowDimensions();

  const sideSize = width < 360 ? 46 : 48;

  const titleWidth = Math.min(
    Math.max(width * 0.42, 128),
    190,
  );

  const handleBack = () => {
    if (onBack) {
      onBack();
      return;
    }

    router.back();
  };

  const handleProfile = () => {
    if (onProfilePress) {
      onProfilePress();
      return;
    }

    router.push('/profile');
  };

  const handleCrown = () => {
    onCrownPress?.();
  };

  return (
    <View
      style={[
        styles.container,
        {
          paddingTop: insets.top + Spacing.two,
          paddingHorizontal: Spacing.three,
        },
        style,
      ]}
    >
      <View style={styles.content}>
        {/* LEFT */}
        <View
          style={[
            styles.sideSlot,
            {
              width: sideSize,
              height: sideSize,
            },
          ]}
        >
          <Pressable
            onPress={showBack ? handleBack : handleProfile}
            hitSlop={8}
            accessibilityRole="button"
            accessibilityLabel={
              showBack ? 'Go back' : 'Open profile'
            }
            style={({ pressed }) => [
              styles.floatingControl,
              {
                width: sideSize,
                height: sideSize,
                borderRadius: sideSize / 2,
                backgroundColor: theme.card,
                borderColor: theme.border,
                shadowColor: Colors.light.text,
              },
              pressed && styles.pressed,
            ]}
          >
            <Ionicons
              name={
                showBack
                  ? 'chevron-back'
                  : 'person-outline'
              }
              size={21}
              color={theme.text}
            />
          </Pressable>
        </View>

        {/* CENTER — PAGE TITLE */}
        <View style={styles.centerSlot}>
          {showTitle && title ? (
            <View
              style={[
                styles.titleControl,
                {
                  width: titleWidth,
                  backgroundColor: theme.card,
                  borderColor: theme.border,
                  shadowColor: Colors.light.text,
                },
              ]}
            >
              <View style={styles.titleContent}>
                <Text
                  numberOfLines={1}
                  ellipsizeMode="tail"
                  style={[
                    styles.title,
                    { color: theme.text },
                  ]}
                >
                  {title}
                </Text>

                {subtitle ? (
                  <Text
                    numberOfLines={1}
                    ellipsizeMode="tail"
                    style={[
                      styles.subtitle,
                      { color: theme.textSecondary },
                    ]}
                  >
                    {subtitle}
                  </Text>
                ) : null}
              </View>
            </View>
          ) : null}
        </View>

        {/* RIGHT */}
        <View
          style={[
            styles.sideSlot,
            {
              width: sideSize,
              height: sideSize,
            },
          ]}
        >
          {rightAction ? (
            rightAction
          ) : showCrown ? (
            <Pressable
              onPress={handleCrown}
              disabled={!onCrownPress}
              hitSlop={8}
              accessibilityRole={
                onCrownPress ? 'button' : undefined
              }
              accessibilityLabel="Premium"
              style={({ pressed }) => [
                styles.floatingControl,
                {
                  width: sideSize,
                  height: sideSize,
                  borderRadius: sideSize / 2,
                  backgroundColor: theme.card,
                  borderColor: theme.border,
                  shadowColor: Colors.light.text,
                  opacity: onCrownPress ? 1 : 0.96,
                },
                pressed &&
                  onCrownPress &&
                  styles.pressed,
              ]}
            >
              <MaterialCommunityIcons
                name="crown-outline"
                size={21}
                color={theme.text}
              />
            </Pressable>
          ) : (
            <View
              style={{
                width: sideSize,
                height: sideSize,
              }}
            />
          )}
        </View>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    width: '100%',
    backgroundColor: 'transparent',
    zIndex: 10,
  },

  content: {
    width: '100%',
    minHeight: 48,

    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },

  sideSlot: {
    flexShrink: 0,
    alignItems: 'center',
    justifyContent: 'center',
  },

  centerSlot: {
    flex: 1,
    height: 48,

    marginHorizontal: Spacing.three,

    alignItems: 'center',
    justifyContent: 'center',
  },

  floatingControl: {
    alignItems: 'center',
    justifyContent: 'center',

    borderWidth: StyleSheet.hairlineWidth,

    shadowOffset: {
      width: 0,
      height: 2,
    },

    shadowOpacity: 0.055,
    shadowRadius: 4,

    elevation: 2,
  },

  titleControl: {
    height: 40,

    paddingHorizontal: Spacing.four,

    borderRadius: Radii.full,
    borderWidth: StyleSheet.hairlineWidth,

    alignItems: 'center',
    justifyContent: 'center',

    shadowOffset: {
      width: 0,
      height: 2,
    },

    shadowOpacity: 0.055,
    shadowRadius: 4,

    elevation: 2,
  },

  titleContent: {
    alignItems: 'center',
    justifyContent: 'center',
    maxWidth: '100%',
  },

  title: {
    ...Typography.bodyBold,
    fontFamily: Fonts.sans,
    textAlign: 'center',
  },

  subtitle: {
    ...Typography.small,
    fontFamily: Fonts.sans,
    textAlign: 'center',
    marginTop: 1,
  },

  pressed: {
    transform: [{ scale: 0.94 }],
  },
});

export default Header;