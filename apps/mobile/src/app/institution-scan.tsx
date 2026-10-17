import { CameraView, useCameraPermissions } from "expo-camera";
import { useRouter } from "expo-router";
import { useCallback, useRef, useState } from "react";
import {
  Pressable,
  StyleSheet,
  Text,
  useWindowDimensions,
  View,
} from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Camera, XMark } from "@orbis/icons/native";
import { Button } from "@/components/ui/button";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { ThemedStatusBar } from "@/components/ui/themed-status-bar";
import { useToast } from "@/components/ui/toast";
import { useI18n } from "@/lib/i18n";

export default function InstitutionScanScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const { height, width } = useWindowDimensions();
  const toast = useToast();
  const { t } = useI18n();
  const [permission, requestPermission] = useCameraPermissions();
  const [cameraReady, setCameraReady] = useState(false);
  const [cameraError, setCameraError] = useState(false);
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
    try {
      const result = await requestPermission();
      if (!result.granted) {
        toast.error(t("auth_institution_local_camera_error"));
      }
    } catch (error: unknown) {
      console.error(
        "[Institution Scan] Camera permission request failed.",
        error,
      );
      toast.error(t("auth_institution_local_camera_error"));
    }
  }, [requestPermission, t, toast]);

  const retryCamera = useCallback(() => {
    setCameraError(false);
    setCameraReady(false);
  }, []);

  return (
    <View style={styles.screen}>
      <ThemedStatusBar backgroundColor="#000000" style="light" />

      {permission?.granted ? (
        <View style={styles.preview}>
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
              toast.error(t("auth_institution_local_camera_error"));
            }}
          />
          <View pointerEvents="none" style={styles.shade} />

          {!cameraReady && !cameraError ? (
            <View pointerEvents="none" style={styles.cameraLoading}>
              <LoadingIndicator size="lg" color="#FFFFFF" />
            </View>
          ) : null}

          {cameraError ? (
            <View style={styles.cameraError}>
              <View style={styles.stateIcon}>
                <Camera size={26} color="#FFFFFF" />
              </View>
              <Text className="text-center text-base font-semibold text-white">
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
        <View style={styles.permissionState}>
          <View style={styles.stateIcon}>
            <Camera size={28} color="#FFFFFF" />
          </View>
          <Text className="text-center text-2xl leading-8 font-bold text-white">
            {t("auth_institution_local_camera_title")}
          </Text>
          <Text className="text-center text-base leading-6 text-white/75">
            {permission
              ? t("auth_institution_local_camera_error")
              : t("auth_institution_local_camera_loading")}
          </Text>
          {permission ? (
            <Button
              title={t("auth_institution_local_camera_enable")}
              onPress={() => void enableCamera()}
            />
          ) : (
            <LoadingIndicator size="lg" color="#FFFFFF" />
          )}
        </View>
      )}

      <View
        pointerEvents="box-none"
        style={[styles.header, { paddingTop: insets.top + 12 }]}
      >
        {permission?.granted && !cameraError ? (
          <View style={styles.headerText}>
            <Text className="text-2xl leading-8 font-bold text-white">
              {t("auth_institution_local_camera_title")}
            </Text>
            <Text className="mt-1 text-base leading-6 text-white/85">
              {t("auth_institution_local_camera_hint")}
            </Text>
          </View>
        ) : (
          <View style={styles.headerText} />
        )}
        <Pressable
          accessibilityRole="button"
          accessibilityLabel={t("common_cancel")}
          onPress={() => router.back()}
          hitSlop={8}
          className="h-12 w-12 items-center justify-center rounded-full border border-white/20 bg-black/40"
        >
          <XMark size={22} color="#FFFFFF" />
        </Pressable>
      </View>

      {cameraReady && !cameraError ? (
        <View
          pointerEvents="none"
          style={[
            styles.footer,
            { paddingBottom: Math.max(insets.bottom, 16) + 16 },
          ]}
        >
          <View style={styles.readyIndicator} />
          <Text className="text-sm font-semibold text-white">
            {t("auth_institution_local_camera_ready")}
          </Text>
        </View>
      ) : null}
    </View>
  );
}

const styles = StyleSheet.create({
  screen: {
    flex: 1,
    backgroundColor: "#000000",
  },
  preview: {
    ...StyleSheet.absoluteFill,
    overflow: "hidden",
  },
  camera: {
    flex: 1,
  },
  shade: {
    ...StyleSheet.absoluteFill,
    backgroundColor: "rgba(0, 0, 0, 0.24)",
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
    gap: 20,
    paddingHorizontal: 32,
    backgroundColor: "rgba(0, 0, 0, 0.76)",
  },
  permissionState: {
    ...StyleSheet.absoluteFill,
    alignItems: "center",
    justifyContent: "center",
    gap: 20,
    paddingHorizontal: 32,
  },
  stateIcon: {
    width: 64,
    height: 64,
    alignItems: "center",
    justifyContent: "center",
    borderRadius: 32,
    backgroundColor: "rgba(255, 255, 255, 0.14)",
  },
  header: {
    position: "absolute",
    top: 0,
    left: 0,
    right: 0,
    flexDirection: "row",
    alignItems: "flex-start",
    justifyContent: "space-between",
    gap: 16,
    paddingHorizontal: 20,
  },
  headerText: {
    flex: 1,
    paddingTop: 4,
  },
  scanGuide: {
    ...StyleSheet.absoluteFill,
    alignItems: "center",
    justifyContent: "center",
  },
  scanFrame: {
    position: "relative",
    borderWidth: 1,
    borderColor: "rgba(255, 255, 255, 0.48)",
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
  footer: {
    position: "absolute",
    left: 24,
    right: 24,
    bottom: 0,
    alignItems: "center",
    justifyContent: "center",
    gap: 10,
  },
  readyIndicator: {
    width: 8,
    height: 8,
    borderRadius: 4,
    backgroundColor: "#A3E635",
  },
});
