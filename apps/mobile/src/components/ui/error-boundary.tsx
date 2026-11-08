/**
 * @module @orbis/mobile/components/ui/error-boundary
 *
 * S.C.R.A.P.S. Crash & Error Boundary for Orbis Mobile.
 * Gracefully catches React rendering errors and displays a themed fallback UI.
 */

import React from "react";
import { Text, View } from "react-native";
import { InfoCircle } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";
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
    <View className="flex-1 items-center justify-center bg-background p-six">
      <View className="w-full max-w-[380px] items-center gap-four rounded-2xl border border-border bg-card p-eight shadow-lg">
        <View className="mb-two h-16 w-16 items-center justify-center rounded-full bg-error-light">
          <InfoCircle size={40} color={theme.destructive} />
        </View>

        <Text className="text-center text-lg leading-6 font-bold text-foreground">
          Something went wrong
        </Text>

        <Text className="text-center text-sm leading-5 text-muted-foreground">
          {__DEV__
            ? error.message
            : "An unexpected error occurred. Please try again."}
        </Text>

        <View className="mt-two">
          <Button
            title="Try Again"
            onPress={resetErrorBoundary}
            variant="default"
            size="default"
            style={{ minWidth: 160 }}
          />
        </View>
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
