import { useCallback, useEffect } from "react";
import { Pressable } from "react-native";
import Animated, {
  cancelAnimation,
  Easing,
  useAnimatedStyle,
  useReducedMotion,
  useSharedValue,
  withSpring,
  withTiming,
} from "react-native-reanimated";

export const AnimatedPressable = Animated.createAnimatedComponent(Pressable);

export function useTactilePress(
  enabled: boolean,
  disabled: boolean,
  shadowColor: string,
) {
  const progress = useSharedValue(0);
  const reduceMotion = useReducedMotion();

  useEffect(() => {
    return () => cancelAnimation(progress);
  }, [progress]);

  const animateTo = useCallback(
    (toValue: number) => {
      if (reduceMotion) {
        progress.set(toValue);
        return;
      }
      progress.set(
        toValue === 1
          ? withTiming(1, {
              duration: 90,
              easing: Easing.out(Easing.quad),
            })
          : withSpring(0, { damping: 18, stiffness: 240, mass: 0.7 }),
      );
    },
    [progress, reduceMotion],
  );

  const onPressIn = useCallback(() => {
    if (enabled && !disabled) animateTo(1);
  }, [animateTo, disabled, enabled]);

  const onPressOut = useCallback(() => animateTo(0), [animateTo]);
  const style = useAnimatedStyle(() => ({
    transform: [
      { translateY: enabled ? progress.value * 2 : 0 },
      { scale: enabled ? 1 - progress.value * 0.015 : 1 },
    ],
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: enabled ? 0.14 * (1 - progress.value) : 0,
    shadowRadius: enabled ? 3 * (1 - progress.value) : 0,
    elevation: enabled ? 3 * (1 - progress.value) : 0,
  }));

  return {
    onPressIn,
    onPressOut,
    shadowStyle: { shadowColor },
    style,
  };
}
