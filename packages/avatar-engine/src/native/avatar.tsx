import {
  useEffect,
  useMemo,
  useRef,
  useState,
  useSyncExternalStore,
} from "react";
import {
  AccessibilityInfo,
  Animated,
  Pressable,
  type StyleProp,
  type ViewStyle,
} from "react-native";
import { SvgXml } from "react-native-svg";
import { AvatarController } from "../core/avatar-controller";
import type { AvatarState } from "../core/avatar-state-machine";
import { renderAvatarSvg } from "../core/avatar-svg";
import { type AvatarHat, resolveAvatarHat } from "../core/types";

export interface NativeAvatarProps {
  controller?: AvatarController;
  state?: AvatarState;
  hat?: AvatarHat;
  size?: number;
  width?: number | `${number}%`;
  height?: number | `${number}%`;
  label?: string;
  faceColor?: string;
  featureColor?: string;
  autoBlink?: boolean;
  onPress?: () => void;
  accessibilityHint?: string;
  style?: StyleProp<ViewStyle>;
}

export function Avatar({
  controller,
  state: controlledState,
  hat = "random",
  size = 120,
  width,
  height,
  label = "Orbis avatar",
  faceColor = "#f1f5f9",
  featureColor = "#18181b",
  autoBlink = true,
  onPress,
  accessibilityHint,
  style,
}: NativeAvatarProps) {
  const [ownedController] = useState(
    () => new AvatarController(controlledState ?? "idle", { autoBlink }),
  );
  const activeController = controller ?? ownedController;
  const state = useSyncExternalStore(
    activeController.subscribe,
    activeController.getSnapshot,
    activeController.getSnapshot,
  );
  const [reduceMotion, setReduceMotion] = useState(false);
  const scale = useRef(new Animated.Value(1)).current;
  const translateX = useRef(new Animated.Value(0)).current;
  const translateY = useRef(new Animated.Value(0)).current;
  const rotate = useRef(new Animated.Value(0)).current;
  const resolvedHat = useMemo(() => resolveAvatarHat(hat), [hat]);
  const markup = useMemo(
    () => renderAvatarSvg({ hat: resolvedHat, state, faceColor, featureColor }),
    [resolvedHat, state, faceColor, featureColor],
  );
  const animatedRotate = useMemo(
    () =>
      rotate.interpolate({
        inputRange: [-1, 1],
        outputRange: ["-8deg", "8deg"],
      }),
    [rotate],
  );
  const animatedStyle = useMemo(
    () => ({
      transform: [
        { translateX },
        { translateY },
        { scale },
        { rotate: animatedRotate },
      ],
    }),
    [animatedRotate, translateX, translateY, scale],
  );

  useEffect(() => {
    activeController.setAutoBlink(autoBlink && !reduceMotion);
  }, [activeController, autoBlink, reduceMotion]);

  useEffect(() => {
    if (controlledState && activeController.state !== controlledState) {
      activeController.transitionTo(controlledState);
    }
  }, [activeController, controlledState]);

  useEffect(() => () => ownedController.destroy(), [ownedController]);

  useEffect(() => {
    let mounted = true;
    AccessibilityInfo.isReduceMotionEnabled().then((enabled) => {
      if (mounted) setReduceMotion(enabled);
    });
    const subscription = AccessibilityInfo.addEventListener(
      "reduceMotionChanged",
      setReduceMotion,
    );
    return () => {
      mounted = false;
      subscription.remove();
    };
  }, []);

  useEffect(() => {
    scale.stopAnimation();
    translateX.stopAnimation();
    translateY.stopAnimation();
    rotate.stopAnimation();
    scale.setValue(1);
    translateX.setValue(0);
    translateY.setValue(0);
    rotate.setValue(0);

    if (reduceMotion) return;

    let animation: Animated.CompositeAnimation | undefined;
    if (state === "happy") {
      animation = Animated.sequence([
        Animated.parallel([
          Animated.timing(scale, {
            toValue: 1.04,
            duration: 160,
            useNativeDriver: true,
          }),
          Animated.timing(translateY, {
            toValue: -3,
            duration: 160,
            useNativeDriver: true,
          }),
        ]),
        Animated.parallel([
          Animated.timing(scale, {
            toValue: 1,
            duration: 240,
            useNativeDriver: true,
          }),
          Animated.timing(translateY, {
            toValue: 0,
            duration: 240,
            useNativeDriver: true,
          }),
        ]),
      ]);
    } else if (state === "error") {
      animation = Animated.sequence([
        Animated.timing(translateX, {
          toValue: -2,
          duration: 55,
          useNativeDriver: true,
        }),
        Animated.timing(translateX, {
          toValue: 2,
          duration: 55,
          useNativeDriver: true,
        }),
        Animated.timing(translateX, {
          toValue: 0,
          duration: 55,
          useNativeDriver: true,
        }),
      ]);
    } else if (state === "celebrating") {
      animation = Animated.sequence([
        Animated.parallel([
          Animated.sequence([
            Animated.timing(translateY, {
              toValue: -8,
              duration: 150,
              useNativeDriver: true,
            }),
            Animated.timing(translateY, {
              toValue: 0,
              duration: 170,
              useNativeDriver: true,
            }),
          ]),
          Animated.sequence([
            Animated.timing(scale, {
              toValue: 1.08,
              duration: 150,
              useNativeDriver: true,
            }),
            Animated.timing(scale, {
              toValue: 1,
              duration: 170,
              useNativeDriver: true,
            }),
          ]),
        ]),
        Animated.parallel([
          Animated.sequence([
            Animated.timing(translateY, {
              toValue: -5,
              duration: 130,
              useNativeDriver: true,
            }),
            Animated.timing(translateY, {
              toValue: 0,
              duration: 170,
              useNativeDriver: true,
            }),
          ]),
          Animated.sequence([
            Animated.timing(rotate, {
              toValue: 1,
              duration: 130,
              useNativeDriver: true,
            }),
            Animated.timing(rotate, {
              toValue: 0,
              duration: 170,
              useNativeDriver: true,
            }),
          ]),
        ]),
      ]);
    } else if (state === "curious") {
      animation = Animated.sequence([
        Animated.timing(rotate, {
          toValue: 1,
          duration: 260,
          useNativeDriver: true,
        }),
        Animated.delay(320),
        Animated.timing(rotate, {
          toValue: 0,
          duration: 300,
          useNativeDriver: true,
        }),
      ]);
    } else if (state === "sleepy") {
      animation = Animated.loop(
        Animated.sequence([
          Animated.timing(scale, {
            toValue: 1.025,
            duration: 1100,
            useNativeDriver: true,
          }),
          Animated.timing(scale, {
            toValue: 1,
            duration: 1100,
            useNativeDriver: true,
          }),
        ]),
      );
    }

    animation?.start();
    return () => animation?.stop();
  }, [reduceMotion, rotate, scale, state, translateX, translateY]);

  const avatar = (
    <Animated.View
      accessibilityRole="image"
      accessibilityLabel={onPress ? undefined : label}
      accessible={!onPress}
      importantForAccessibility={onPress ? "no" : "yes"}
      style={[
        { width: width ?? size, height: height ?? size, overflow: "visible" },
        style,
        animatedStyle,
      ]}
    >
      <SvgXml xml={markup} width="100%" height="100%" />
    </Animated.View>
  );

  if (!onPress) return avatar;

  return (
    <Pressable
      accessibilityRole="button"
      accessibilityLabel={label}
      accessibilityHint={accessibilityHint}
      onPress={onPress}
      hitSlop={8}
      style={{ width: width ?? size, height: height ?? size }}
    >
      {avatar}
    </Pressable>
  );
}
