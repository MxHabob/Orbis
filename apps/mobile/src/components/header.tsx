/**
 * @module @orbis/mobile/components/header
 *
 * Universal Screen Header.
 *
 * The Header is intentionally background-free.
 * Screen-level gradients/backgrounds must be rendered by the screen
 * itself, not by this component.
 */

import React from 'react';

import {
  View,
  Text,
  StyleSheet,
  Pressable,
  type StyleProp,
  type ViewStyle,
} from 'react-native';

import {
  router,
} from 'expo-router';

import {
  Ionicons,
} from '@expo/vector-icons';

import {
  useSafeAreaInsets,
} from 'react-native-safe-area-context';

import {
  useTheme,
} from '@/hooks/use-theme';

import {
  Spacing,
  Typography,
} from '@/constants/theme';


export interface HeaderProps {
  title: string;

  subtitle?: string;

  showBack?: boolean;

  onBack?: () => void;

  rightAction?: React.ReactNode;

  style?: StyleProp<ViewStyle>;

  /**
   * Kept for API compatibility.
   *
   * Header itself never renders a background.
   */
  transparent?: boolean;
}


export function Header({
  title,
  subtitle,
  showBack = false,
  onBack,
  rightAction,
  style,
}: HeaderProps) {

  const insets =
    useSafeAreaInsets();

  const theme =
    useTheme();


  // ───────────────────────────────────────────────────────────────────────
  // Back Navigation
  // ───────────────────────────────────────────────────────────────────────

  const handleBack = () => {

    if (onBack) {
      onBack();

      return;
    }


    if (router.canGoBack()) {
      router.back();

      return;
    }


    router.replace('/');
  };


  // ───────────────────────────────────────────────────────────────────────
  // Render
  // ───────────────────────────────────────────────────────────────────────

  return (
    <View
      style={[
        styles.container,

        {
          paddingTop:
            insets.top +
            Spacing.two,
        },

        style,
      ]}>

      <View
        style={
          styles.contentRow
        }>

        {/* ─────────────────────────────────────────────────────────
            Left Group
          ───────────────────────────────────────────────────────── */}

        <View
          style={
            styles.leftGroup
          }>

          {showBack && (
            <View
              style={[
                styles.floatingLeft,

                {
                  backgroundColor:
                    theme.backgroundElement,

                  borderColor:
                    theme.border,

                  shadowColor:
                    theme.foreground,
                },
              ]}>

              <Pressable
                onPress={
                  handleBack
                }

                style={({
                  pressed,
                }) => [
                  styles.backButton,

                  pressed && {
                    opacity:
                      0.7,
                  },
                ]}

                hitSlop={
                  8
                }>

                <Ionicons
                  name="chevron-back"
                  size={22}
                  color={
                    theme.text
                  }
                />

              </Pressable>

            </View>
          )}


          {title ? (
            <View
              style={[
                styles.titleWrap,

                {
                  backgroundColor:
                    theme.backgroundElement,

                  borderColor:
                    theme.border,

                  shadowColor:
                    theme.foreground,
                },
              ]}>

              <Text
                style={[
                  styles.title,

                  {
                    color:
                      theme.text,
                  },
                ]}

                numberOfLines={
                  1
                }>

                {title}

              </Text>


              {subtitle ? (
                <Text
                  style={[
                    styles.subtitle,

                    {
                      color:
                        theme.textSecondary,
                    },
                  ]}

                  numberOfLines={
                    1
                  }>

                  {subtitle}

                </Text>
              ) : null}

            </View>
          ) : null}

        </View>


        {/* ─────────────────────────────────────────────────────────
            Right Action
          ───────────────────────────────────────────────────────── */}

        <View
          style={
            styles.rightSlot
          }>

          {rightAction ? (
            <View
              style={[
                styles.floatingRight,

                {
                  backgroundColor:
                    theme.backgroundElement,

                  borderColor:
                    theme.border,

                  shadowColor:
                    theme.foreground,
                },
              ]}>

              {rightAction}

            </View>
          ) : null}

        </View>

      </View>

    </View>
  );
}


// ─────────────────────────────────────────────────────────────────────────
// Styles
// ─────────────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({

  container: {
    width:
      '100%',

    paddingHorizontal:
      Spacing.four,

    paddingBottom:
      0,

    backgroundColor:
      'transparent',
  },


  contentRow: {
    width:
      '100%',

    flexDirection:
      'row',

    alignItems:
      'center',

    justifyContent:
      'space-between',

    minHeight:
      56,
  },


  leftGroup: {
    flexDirection:
      'row',

    alignItems:
      'center',

    flexShrink:
      1,

    gap:
      Spacing.one,
  },


  floatingLeft: {
    borderRadius:
      20,

    borderWidth:
      1,

    overflow:
      'hidden',

    shadowOffset: {
      width:
        0,

      height:
        3,
    },

    shadowOpacity:
      0.10,

    shadowRadius:
      8,

    elevation:
      3,
  },


  backButton: {
    width:
      36,

    height:
      36,

    borderRadius:
      18,

    alignItems:
      'center',

    justifyContent:
      'center',
  },


  titleWrap: {
    alignItems:
      'center',

    justifyContent:
      'center',

    paddingHorizontal:
      Spacing.three,

    paddingVertical:
      Spacing.one,

    borderRadius:
      22,

    borderWidth:
      1,

    minWidth:
      170,

    maxWidth:
      260,

    shadowOffset: {
      width:
        0,

      height:
        4,
    },

    shadowOpacity:
      0.08,

    shadowRadius:
      12,

    elevation:
      3,
  },


  title: {
    ...Typography.bodyBold,

    fontSize:
      17,

    textAlign:
      'center',
  },


  subtitle: {
    ...Typography.caption,

    fontSize:
      11,

    textAlign:
      'center',

    marginTop:
      2,
  },


  rightSlot: {
    width:
      44,

    alignItems:
      'flex-end',

    justifyContent:
      'center',

    marginLeft:
      Spacing.two,
  },


  floatingRight: {
    minWidth:
      38,

    minHeight:
      38,

    borderRadius:
      20,

    borderWidth:
      1,

    alignItems:
      'center',

    justifyContent:
      'center',

    paddingHorizontal:
      Spacing.one,

    shadowOffset: {
      width:
        0,

      height:
        3,
    },

    shadowOpacity:
      0.10,

    shadowRadius:
      8,

    elevation:
      3,
  },
});