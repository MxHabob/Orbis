/**
 * @module @orbis/mobile/components/ui/error-boundary
 *
 * S.C.R.A.P.S. Crash & Error Boundary for Orbis Mobile.
 * Gracefully catches React rendering errors and displays a themed fallback UI.
 */

import React from "react";
import { StyleSheet, Text, View } from "react-native";
import { InfoCircle } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";
import { Radii, Spacing, Typography } from "@/constants/theme";
import { Button } from "./button";

// ─── Error Fallback UI ─────────────────────────────────────────────────────────

export interface ErrorFallbackProps {
  error: Error;
  resetErrorBoundary: () => void;
}

export function ErrorFallback({
  error,
  resetErrorBoundary,
}: ErrorFallbackProps) {
  const theme = useTheme();

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <View
        style={[
          styles.card,
          {
            backgroundColor: theme.card,
            borderColor: theme.border,
          },
        ]}
      >
        <View
          style={[styles.iconWrapper, { backgroundColor: theme.errorLight }]}
        >
          <InfoCircle size={40} color={theme.destructive} />
        </View>

        <Text style={[styles.title, { color: theme.text }]}>
          Something went wrong
        </Text>

        <Text style={[styles.message, { color: theme.textSecondary }]}>
          {__DEV__
            ? error.message
            : "An unexpected error occurred. Please try again."}
        </Text>

        <Button
          title="Try Again"
          onPress={resetErrorBoundary}
          variant="default"
          size="default"
          style={styles.button}
        />
      </View>
    </View>
  );
}

// ─── Class-based ErrorBoundary ──────────────────────────────────────────────────

interface Props {
  children: React.ReactNode;
  fallback?: React.ComponentType<ErrorFallbackProps>;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class AppErrorBoundary extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    if (__DEV__) {
      console.warn("[AppErrorBoundary] Caught error:", error, errorInfo);
    }
  }

  resetErrorBoundary = () => {
    this.setState({ hasError: false, error: null });
  };

  render() {
    if (this.state.hasError && this.state.error) {
      const FallbackComponent = this.props.fallback || ErrorFallback;
      return (
        <FallbackComponent
          error={this.state.error}
          resetErrorBoundary={this.resetErrorBoundary}
        />
      );
    }

    return this.props.children;
  }
}

export const ErrorBoundary = AppErrorBoundary;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
    padding: Spacing.six,
  },
  card: {
    width: "100%",
    maxWidth: 380,
    padding: Spacing.eight,
    borderRadius: Radii["2xl"],
    borderWidth: 1,
    alignItems: "center",
    gap: Spacing.four,
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 8 },
    shadowOpacity: 0.1,
    shadowRadius: 16,
    elevation: 6,
  },
  iconWrapper: {
    width: 64,
    height: 64,
    borderRadius: Radii.full,
    alignItems: "center",
    justifyContent: "center",
    marginBottom: Spacing.two,
  },
  title: {
    ...Typography.h2,
    textAlign: "center",
  },
  message: {
    ...Typography.body,
    textAlign: "center",
    lineHeight: 20,
  },
  button: {
    marginTop: Spacing.two,
    minWidth: 160,
  },
});
