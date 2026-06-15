import { useCallback, useState } from "react";

type ActionStatus = "idle" | "executing" | "hasSucceeded" | "hasErrored";

/** Desktop replacement for next-safe-action useAction hook. */
export function useSafeAction<TInput, TResult>(
  action: (input: TInput) => Promise<TResult>,
  options?: {
    onSuccess?: (data: TResult) => void;
    onError?: (error: Error) => void;
  },
) {
  const [status, setStatus] = useState<ActionStatus>("idle");

  const execute = useCallback(
    async (input: TInput) => {
      setStatus("executing");
      try {
        const result = await action(input);
        setStatus("hasSucceeded");
        options?.onSuccess?.(result);
        return result;
      } catch (error) {
        setStatus("hasErrored");
        options?.onError?.(
          error instanceof Error ? error : new Error(String(error)),
        );
        throw error;
      }
    },
    [action, options],
  );

  return {
    execute,
    status,
    isExecuting: status === "executing",
  };
}

/** @deprecated Use useSafeAction — alias for migration from next-safe-action */
export const useAction = useSafeAction;
