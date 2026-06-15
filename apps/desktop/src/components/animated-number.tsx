import NumberFlow from "@number-flow/react";
import { useQuery } from "@tanstack/react-query";
import { useTRPC } from "@/ipc/query-adapter";

type Props = {
  value: number;
  currency?: string;
  minimumFractionDigits?: number;
  maximumFractionDigits?: number;
  locale?: string;
};

export function AnimatedNumber({
  value,
  currency,
  minimumFractionDigits,
  maximumFractionDigits,
  locale,
}: Props) {
  const trpc = useTRPC();
  // Use regular useQuery instead of useSuspenseQuery to allow it to fail gracefully
  // This is needed for public pages where user data isn't available
  const { data: user } = useQuery({
    ...trpc.user.me.queryOptions(),
    retry: false,
    // Don't throw errors - just return undefined if user isn't available
    throwOnError: false,
  });
  const localeToUse = locale || user?.locale;

  const format: Intl.NumberFormatOptions = {
    minimumFractionDigits,
    maximumFractionDigits,
  };

  if (currency) {
    format.style = "currency";
    format.currency = currency;
  }

  return (
    <NumberFlow
      value={value}
      format={format}
      willChange
      locales={localeToUse ?? "en"}
    />
  );
}
