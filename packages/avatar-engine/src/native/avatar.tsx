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
import { renderAvatarSvg } from "../core/avatar-svg";
import { type AvatarHat, resolveAvatarHat } from "../core/types";

export interface NativeAvatarProps {
  controller?: AvatarController;
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
    () => new AvatarController("idle", { autoBlink }),
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
  const resolvedHat = useMemo(() => resolveAvatarHat(hat), [hat]);
  const markup = useMemo(
    () => renderAvatarSvg({ hat: resolvedHat, state, faceColor, featureColor }),
    [resolvedHat, state, faceColor, featureColor],
  );
  const animatedStyle = useMemo(
    () => ({
      transform: [{ translateX }, { translateY }, { scale }],
    }),
    [translateX, translateY, scale],
  );

  useEffect(() => {
    activeController.setAutoBlink(autoBlink && !reduceMotion);
  }, [activeController, autoBlink, reduceMotion]);

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
    scale.setValue(1);
    translateX.setValue(0);
    translateY.setValue(0);

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
    }

    animation?.start();
    return () => animation?.stop();
  }, [reduceMotion, scale, state, translateX, translateY]);

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
