/**
 * @module @orbis/mobile/components/ui/floating-tab-bar
 *
 * Orbis Floating Tab Bar.
 *
 * Three independent floating controls:
 *
 *   [ Apps ]       [ Practice ]       [ Analytics ]
 *
 * There is intentionally no shared floating container.
 */

import React from 'react';

import {
  Pressable,
  StyleSheet,
  Text,
  View,
  type StyleProp,
  type ViewStyle,
} from 'react-native';

import { Ionicons } from '@expo/vector-icons';

import { useSafeAreaInsets } from 'react-native-safe-area-context';

import {
  Fonts,
  Radii,
  Spacing,
  Typography,
} from '@/constants/theme';

import { useColorScheme } from '@/hooks/use-color-scheme';

import { useTheme } from '@/hooks/use-theme';


type Route = {
  key: string;
  name: string;
  params?: unknown;
};


type FloatingTabBarProps = {
  state: {
    index: number;
    routes: Route[];
  };

  navigation: {
    emit: (event: {
      type: string;
      target: string;
      canPreventDefault: boolean;
    }) => {
      defaultPrevented: boolean;
    };

    navigate: (
      name: string,
      params?: unknown,
    ) => void;
  };

  descriptors: Record<
    string,
    {
      options: {
        title?: string;
      };
    }
  >;
};


type FloatingControlProps = {
  children: React.ReactNode;

  onPress: () => void;

  accessibilityLabel: string;

  style?: StyleProp<ViewStyle>;
};


export function FloatingTabBar({
  state,
  navigation,
  descriptors,
}: FloatingTabBarProps) {

  const insets =
    useSafeAreaInsets();

  const theme =
    useTheme();

  const colorScheme =
    useColorScheme();

  const isDark =
    colorScheme === 'dark';


  // ───────────────────────────────────────────────────────────────────────
  // Bottom Position
  // ───────────────────────────────────────────────────────────────────────

  const bottom =
    Math.max(
      insets.bottom,
      Spacing.three,
    ) +
    Spacing.one;


  // ───────────────────────────────────────────────────────────────────────
  // Routes
  // ───────────────────────────────────────────────────────────────────────

  const getRoute = (
    name: string,
  ) =>
    state.routes.find(
      (route) =>
        route.name === name,
    );


  const getRouteIndex = (
    name: string,
  ) =>
    state.routes.findIndex(
      (route) =>
        route.name === name,
    );


  const isRouteFocused = (
    name: string,
  ) => {

    const index =
      getRouteIndex(name);

    return (
      index !== -1 &&
      state.index === index
    );
  };


  const navigateToRoute = (
    name: string,
  ) => {

    const routeIndex =
      getRouteIndex(name);

    if (routeIndex === -1) {
      return;
    }

    const route =
      state.routes[routeIndex];

    const event =
      navigation.emit({
        type: 'tabPress',
        target: route.key,
        canPreventDefault: true,
      });

    if (
      !event.defaultPrevented
    ) {
      navigation.navigate(
        route.name,
        route.params,
      );
    }
  };


  const indexRoute =
    getRoute('index');

  const behaviorRoute =
    getRoute('behavior');

  const IdCardDialog = getRoute('');


  const indexFocused =
    isRouteFocused('index');

  const behaviorFocused =
    isRouteFocused('behavior');

  const profileFocused =
    isRouteFocused('profile');


  // ───────────────────────────────────────────────────────────────────────
  // Surface
  // ───────────────────────────────────────────────────────────────────────

  const surface =
    isDark
      ? theme.cardElevated
      : theme.cardElevated;

  const border =
    theme.cardBorder;

  const iconColor =
    theme.foreground;


  // ───────────────────────────────────────────────────────────────────────
  // Render
  // ───────────────────────────────────────────────────────────────────────

  return (
    <View
      pointerEvents="box-none"
      style={[
        styles.navigation,

        {
          bottom,
        },
      ]}>

      {/* ─────────────────────────────────────────────────────────────
          LEFT — Apps
        ───────────────────────────────────────────────────────────── */}

      <View
        style={
          styles.appsWrapper
        }>

        <View
          pointerEvents="none"
          style={[
            styles.newBadge,

            {
              backgroundColor:
                theme.backgroundSelected,
            },
          ]}>

          <Text
            style={[
              styles.newBadgeText,

              {
                color:
                  theme.foreground,

                fontFamily:
                  Fonts.sans,
              },
            ]}>

            NEW

          </Text>

        </View>


        <FloatingControl
          accessibilityLabel={
            indexRoute
              ? descriptors[
                  indexRoute.key
                ]?.options.title ??
                'Apps'
              : 'Apps'
          }

          onPress={() =>
            navigateToRoute(
              'index',
            )
          }

          style={[
            styles.iconButton,

            {
              backgroundColor:
                surface,

              borderColor:
                border,
            },
          ]}>

          <Ionicons
            name={
              indexFocused
                ? 'grid'
                : 'grid-outline'
            }

            size={21}

            color={
              iconColor
            }
          />

        </FloatingControl>

      </View>


      {/* ─────────────────────────────────────────────────────────────
          CENTER — Practice
        ───────────────────────────────────────────────────────────── */}

      <FloatingControl
        accessibilityLabel={
          behaviorRoute
            ? descriptors[
                behaviorRoute.key
              ]?.options.title ??
              'Practice'
            : 'Practice'
        }

        onPress={() =>
          navigateToRoute(
            'behavior',
          )
        }

        style={[
          styles.practiceButton,

          {
            backgroundColor:
              surface,

            borderColor:
              border,
          },
        ]}>

        <Ionicons
          name={
            behaviorFocused
              ? 'school'
              : 'school-outline'
          }

          size={21}

          color={
            iconColor
          }
        />


        <Text
          numberOfLines={1}
          style={[
            styles.practiceText,

            {
              color:
                theme.foreground,

              fontFamily:
                Fonts.sans,
            },
          ]}>

          {behaviorRoute
            ? descriptors[
                behaviorRoute.key
              ]?.options.title ??
              'Practice'
            : 'Practice'}

        </Text>

      </FloatingControl>


      {/* ─────────────────────────────────────────────────────────────
          RIGHT — Analytics
        ───────────────────────────────────────────────────────────── */}

      <FloatingControl
        accessibilityLabel={
          behaviorRoute
            ? descriptors[
                behaviorRoute.key
              ]?.options.title ??
              'Analytics'
            : 'Analytics'
        }

        onPress={() =>
          navigateToRoute(
            'profile',
          )
        }

        style={[
          styles.iconButton,

          {
            backgroundColor:
              surface,

            borderColor:
              border,
          },
        ]}>

        <Ionicons
          name={
            profileFocused
              ? 'bar-chart'
              : 'bar-chart-outline'
          }

          size={21}

          color={
            iconColor
          }
        />

      </FloatingControl>

    </View>
  );
}


// ─────────────────────────────────────────────────────────────────────────
// Floating Control
// ─────────────────────────────────────────────────────────────────────────

function FloatingControl({
  children,
  onPress,
  accessibilityLabel,
  style,
}: FloatingControlProps) {

  return (
    <Pressable
      accessibilityRole="button"

      accessibilityLabel={
        accessibilityLabel
      }

      onPress={
        onPress
      }

      style={({
        pressed,
      }) => [
        styles.control,

        style,

        pressed &&
        styles.pressed,
      ]}>

      {children}

    </Pressable>
  );
}


// ─────────────────────────────────────────────────────────────────────────
// Styles
// ─────────────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({

  /**
   * The navigation container is only a layout container.
   *
   * No background.
   * No border.
   * No shadow.
   */
  navigation: {
    position: 'absolute',

    left:
      Spacing.four,

    right:
      Spacing.four,

    minHeight:
      48,

    flexDirection:
      'row',

    alignItems:
      'center',

    justifyContent:
      'space-between',

    gap:
      Spacing.two,

    zIndex:
      100,
  },


  /**
   * Shared floating surface.
   */
  control: {
    alignItems:
      'center',

    justifyContent:
      'center',

    borderWidth:
      StyleSheet.hairlineWidth,

    elevation:
      4,

    shadowColor:
      '#000000',

    shadowOffset: {
      width: 0,
      height: 2,
    },

    shadowOpacity:
      0.07,

    shadowRadius:
      6,
  },


  /**
   * LEFT + RIGHT.
   *
   * 48 × 48
   */
  iconButton: {
    width:
      48,

    height:
      48,

    flexShrink:
      0,

    borderRadius:
      Radii.full,
  },


  /**
   * Wrapper only exists so NEW can sit above Apps.
   */
  appsWrapper: {
    width:
      48,

    height:
      48,

    flexShrink:
      0,

    alignItems:
      'center',

    justifyContent:
      'center',

    position:
      'relative',
  },


  /**
   * NEW badge.
   *
   * Small and visually subordinate to the button.
   */
  newBadge: {
    position:
      'absolute',

    top:
      -16,

    left:
      -2,

    minWidth:
      58,

    height:
      30,

    paddingHorizontal:
      Spacing.two,

    borderRadius:
      Radii.full,

    alignItems:
      'center',

    justifyContent:
      'center',

    zIndex:
      10,
  },


  newBadgeText: {
    ...Typography.small,

    fontSize:
      12,

    lineHeight:
      14,

    fontWeight:
      '500',

    letterSpacing:
      0.1,
  },


  /**
   * CENTER.
   *
   * Flexible width.
   *
   * Typical:
   *   140–160 × 44
   */
  practiceButton: {
    flex: 1,

    height:
      44,

    minWidth:
      120,

    maxWidth:
      160,

    paddingHorizontal:
      Spacing.three,

    borderRadius:
      Radii.full,

    flexDirection:
      'row',

    alignItems:
      'center',

    justifyContent:
      'center',

    gap:
      Spacing.one,
  },


  practiceText: {
    ...Typography.body,

    fontSize:
      14,

    lineHeight:
      18,

    fontWeight:
      '500',

    flexShrink:
      1,
  },


  pressed: {
    transform: [
      {
        scale: 0.96,
      },
    ],
  },
});