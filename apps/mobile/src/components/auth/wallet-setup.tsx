/**
 * @module @orbis/mobile/components/auth/wallet-setup
 *
 * Wallet protection setup screen.
 */

import React from "react";
import {
  Pressable,
  SafeAreaView,
  StatusBar,
  StyleSheet,
  Text,
  View,
} from "react-native";
import { Ionicons } from "@expo/vector-icons";

import { GradientHeader } from "@/components/gradient-header";

export function WalletSetupDemo() {
  const handleClose = () => {
    // TODO: close wallet setup
  };

  const handleCreateDeviceKey = () => {
    // TODO: create device key
  };

  return (
    <View style={styles.container}>
      <StatusBar
        barStyle="light-content"
        backgroundColor="transparent"
        translucent
      />

      <GradientHeader
        height={300}
        animated
        duration={16000}
        colors={[
          "#7628F5",
          "#893BF2",
          "#A75AF0",
          "#C98DF0",
          "#E8CFF4",
          "#F8F3FA",
          "#FFFFFF",
        ]}
        locations={[0, 0.12, 0.27, 0.43, 0.58, 0.72, 1]}
      />

      <SafeAreaView style={styles.safeArea}>
        {/* Top navigation */}
        <View style={styles.header}>
          <Text style={styles.headerTitle}>{"Secure\nyour wallet"}</Text>

          <Pressable
            accessibilityRole="button"
            accessibilityLabel="Close"
            hitSlop={12}
            onPress={handleClose}
            style={({ pressed }) => [
              styles.closeButton,
              pressed && styles.closeButtonPressed,
            ]}
          >
            <Ionicons name="close" size={11} color="rgba(255,255,255,0.82)" />
          </Pressable>
        </View>

        {/* Main content */}
        <View style={styles.content}>
          <View style={styles.contentInner}>
            {/* Step indicator */}
            <View style={styles.stepIcon}>
              <Ionicons name="scan-outline" size={13} color="#D5D5D5" />
            </View>

            <Text style={styles.title}>Device Key</Text>

            <Text style={styles.description}>
              {"Your Device Key is protected by\n"}
              {"biometric verification — encrypted and\n"}
              {"stored on your phone."}
            </Text>

            {/* Alternative methods */}
            <View style={styles.options}>
              <Pressable style={styles.option}>
                <Ionicons name="cloud-outline" size={13} color="#D5D5D5" />

                <Text style={styles.optionText}>2FA Key</Text>
              </Pressable>

              <Pressable style={styles.option}>
                <Ionicons name="mail-outline" size={13} color="#D5D5D5" />

                <Text style={styles.optionText}>Recovery Key</Text>
              </Pressable>
            </View>
          </View>
        </View>

        {/* Bottom action */}
        <View style={styles.footer}>
          <Pressable
            accessibilityRole="button"
            onPress={handleCreateDeviceKey}
            style={({ pressed }) => [
              styles.primaryButton,
              pressed && styles.primaryButtonPressed,
            ]}
          >
            <Ionicons name="scan-outline" size={12} color="#FFFFFF" />

            <Text style={styles.primaryButtonText}>Create Device Key</Text>
          </Pressable>
        </View>
      </SafeAreaView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: "#FFFFFF",
  },

  safeArea: {
    flex: 1,
  },

  // ─────────────────────────────────────────────
  // Header
  // ─────────────────────────────────────────────

  header: {
    flexDirection: "row",
    alignItems: "flex-start",
    justifyContent: "space-between",

    paddingHorizontal: 24,
    paddingTop: 10,
  },

  headerTitle: {
    color: "rgba(255,255,255,0.94)",

    fontSize: 9,
    lineHeight: 10,

    fontWeight: "500",
    letterSpacing: -0.15,
  },

  closeButton: {
    width: 18,
    height: 18,

    borderRadius: 9,

    alignItems: "center",
    justifyContent: "center",

    backgroundColor: "rgba(255,255,255,0.16)",

    borderWidth: StyleSheet.hairlineWidth,
    borderColor: "rgba(255,255,255,0.12)",
  },

  closeButtonPressed: {
    opacity: 0.55,
    transform: [{ scale: 0.94 }],
  },

  // ─────────────────────────────────────────────
  // Content
  // ─────────────────────────────────────────────

  content: {
    flex: 1,

    justifyContent: "center",

    paddingHorizontal: 24,
    paddingBottom: 32,
  },

  contentInner: {
    alignItems: "flex-start",
  },

  stepIcon: {
    width: 16,
    height: 16,

    alignItems: "center",
    justifyContent: "center",

    marginBottom: 6,
  },

  title: {
    color: "#111111",

    fontSize: 17,
    lineHeight: 21,

    fontWeight: "700",
    letterSpacing: -0.5,
  },

  description: {
    marginTop: 5,

    color: "#A7A7A7",

    fontSize: 8,
    lineHeight: 11,

    fontWeight: "400",
    letterSpacing: -0.05,
  },

  // ─────────────────────────────────────────────
  // Options
  // ─────────────────────────────────────────────

  options: {
    marginTop: 17,

    gap: 10,
  },

  option: {
    minHeight: 16,

    flexDirection: "row",
    alignItems: "center",
  },

  optionText: {
    marginLeft: 7,

    color: "#9E9E9E",

    fontSize: 9,
    lineHeight: 12,

    fontWeight: "500",
  },

  // ─────────────────────────────────────────────
  // Footer
  // ─────────────────────────────────────────────

  footer: {
    paddingHorizontal: 24,
    paddingBottom: 8,
  },

  primaryButton: {
    width: "100%",
    height: 36,

    borderRadius: 18,

    backgroundColor: "#000000",

    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",

    gap: 6,
  },

  primaryButtonPressed: {
    opacity: 0.8,
    transform: [{ scale: 0.99 }],
  },

  primaryButtonText: {
    color: "#FFFFFF",

    fontSize: 9,
    lineHeight: 12,

    fontWeight: "600",
    letterSpacing: -0.1,
  },
});
