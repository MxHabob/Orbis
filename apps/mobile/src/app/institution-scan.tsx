import { CameraView, useCameraPermissions } from "expo-camera";
import { useRouter } from "expo-router";
import { useCallback, useRef, useState } from "react";
import { Pressable, StyleSheet, useWindowDimensions, View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Camera, XMark } from "@orbis/icons/native";
import { Button } from "@/components/ui/button";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import { Text } from "@/components/ui/text";

export default function InstitutionScanScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const { height, width } = useWindowDimensions();
  const theme = useTheme();
  const { t } = useI18n();
  const [permission, requestPermission] = useCameraPermissions();
  const [cameraReady, setCameraReady] = useState(false);
  const [cameraError, setCameraError] = useState(false);
  const [permissionRequestError, setPermissionRequestError] = useState(false);
  const scannedRef = useRef(false);
  const frameSize = Math.min(width - 72, height * 0.36, 320);

  const scanCode = useCallback(
    ({ data }: { data: string }) => {
      if (scannedRef.current) return;
      scannedRef.current = true;
      router.dismissTo({
        pathname: "/institution-select",
        params: { institutionQr: data },
      });
    },
    [router],
  );

  const enableCamera = useCallback(async () => {
    setPermissionRequestError(false);
    try {
      await requestPermission();
    } catch (error: unknown) {
      console.error(
        "[Institution Scan] Camera permission request failed.",
        error,
      );
      setPermissionRequestError(true);
    }
  }, [requestPermission]);

  const retryCamera = useCallback(() => {
    setCameraError(false);
    setCameraReady(false);
  }, []);

  return (
    <View style={[styles.screen, { backgroundColor: theme.background }]}>
      <View
        style={[
          styles.content,
          {
            paddingTop: insets.top + 12,
            paddingBottom: Math.max(insets.bottom, 16),
          },
        ]}
      >
        <View style={styles.header}>
          <View style={styles.headerText}>
            <Text
              className="text-2xl leading-8 font-bold"
              style={{ color: theme.foreground }}
            >
              {t("auth_institution_local_camera_title")}
            </Text>
            <Text
              className="mt-1 text-sm leading-5"
              style={{ color: theme.mutedForeground }}
            >
              {t("auth_institution_local_camera_hint")}
            </Text>
          </View>
          <Pressable
            accessibilityRole="button"
            accessibilityLabel={t("common_cancel")}
            onPress={() => router.back()}
            hitSlop={8}
            style={[
              styles.closeButton,
              {
                borderColor: theme.border,
                backgroundColor: theme.backgroundElement,
              },
            ]}
          >
            <XMark size={22} color={theme.foreground} />
          </Pressable>
        </View>

        {permission?.granted ? (
          <View
            style={[
              styles.preview,
              { backgroundColor: theme.backgroundElement },
            ]}
          >
            <CameraView
              key={cameraError ? "camera-retry" : "camera"}
              active
              facing="back"
              style={styles.camera}
              barcodeScannerSettings={{ barcodeTypes: ["qr"] }}
              onCameraReady={() => setCameraReady(true)}
              onBarcodeScanned={scanCode}
              onMountError={(error) => {
                console.error(
                  "[Institution Scan] Camera preview failed to start.",
                  error,
                );
                setCameraError(true);
              }}
            />
            {!cameraReady && !cameraError ? (
              <View pointerEvents="none" style={styles.cameraLoading}>
                <LoadingIndicator size="lg" color={theme.primary} />
              </View>
            ) : null}

            {cameraError ? (
              <View
                style={[
                  styles.cameraError,
                  { backgroundColor: theme.backgroundElement },
                ]}
              >
                <Text
                  className="text-center text-base font-semibold"
                  style={{ color: theme.foreground }}
                >
                  {t("auth_institution_local_camera_error")}
                </Text>
                <Button
                  title={t("common_retry")}
                  variant="outline"
                  onPress={retryCamera}
                />
              </View>
            ) : null}

            {cameraReady && !cameraError ? (
              <View pointerEvents="none" style={styles.scanGuide}>
                <View
                  style={[
                    styles.scanFrame,
                    { width: frameSize, height: frameSize },
                  ]}
                >
                  <View style={[styles.corner, styles.topStart]} />
                  <View style={[styles.corner, styles.topEnd]} />
                  <View style={[styles.corner, styles.bottomStart]} />
                  <View style={[styles.corner, styles.bottomEnd]} />
                </View>
              </View>
            ) : null}
          </View>
        ) : (
          <View
            style={[
              styles.permissionState,
              { backgroundColor: theme.backgroundElement },
            ]}
          >
            <Camera size={28} color={theme.primary} />
            <Text
              className="text-center text-base leading-6"
              style={{ color: theme.mutedForeground }}
            >
              {permission || permissionRequestError
                ? t("auth_institution_local_camera_error")
                : t("auth_institution_local_camera_loading")}
            </Text>
            {permission ? (
              <Button
                title={t("auth_institution_local_camera_enable")}
                onPress={() => void enableCamera()}
              />
            ) : (
              <LoadingIndicator size="lg" color={theme.primary} />
            )}
          </View>
        )}
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  screen: {
    flex: 1,
  },
  content: {
    flex: 1,
    paddingHorizontal: 16,
    gap: 20,
  },
  preview: {
    flex: 1,
    minHeight: 240,
    overflow: "hidden",
    borderRadius: 28,
  },
  camera: {
    flex: 1,
  },
  cameraLoading: {
    ...StyleSheet.absoluteFill,
    alignItems: "center",
    justifyContent: "center",
  },
  cameraError: {
    ...StyleSheet.absoluteFill,
    alignItems: "center",
    justifyContent: "center",
    gap: 16,
    paddingHorizontal: 24,
  },
  permissionState: {
    flex: 1,
    minHeight: 240,
    alignItems: "center",
    justifyContent: "center",
    gap: 16,
    paddingHorizontal: 24,
    borderRadius: 28,
  },
  closeButton: {
    height: 48,
    width: 48,
    alignItems: "center",
    justifyContent: "center",
    borderRadius: 24,
    borderWidth: 1,
  },
  header: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    gap: 16,
    minHeight: 56,
  },
  scanGuide: {
    ...StyleSheet.absoluteFill,
    alignItems: "center",
    justifyContent: "center",
  },
  scanFrame: {
    position: "relative",
    borderWidth: 1,
    borderColor: "rgba(255, 255, 255, 0.72)",
    borderRadius: 28,
    backgroundColor: "rgba(255, 255, 255, 0.025)",
  },
  corner: {
    position: "absolute",
    width: 32,
    height: 32,
    borderColor: "#FFFFFF",
    borderWidth: 4,
    borderRadius: 8,
  },
  topStart: {
    top: -2,
    left: -2,
    borderRightWidth: 0,
    borderBottomWidth: 0,
  },
  topEnd: {
    top: -2,
    right: -2,
    borderLeftWidth: 0,
    borderBottomWidth: 0,
  },
  bottomStart: {
    bottom: -2,
    left: -2,
    borderRightWidth: 0,
    borderTopWidth: 0,
  },
  bottomEnd: {
    bottom: -2,
    right: -2,
    borderLeftWidth: 0,
    borderTopWidth: 0,
  },
  headerText: {
    flex: 1,
  },
});
