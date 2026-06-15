import { useSearchParams } from "react-router-dom";
import { CheckoutSuccessDesktop } from "@/components/checkout-success-desktop";

export default function CheckoutSuccessPage() {
  const [searchParams] = useSearchParams();
  const redirectPath = searchParams.get("redirectPath") || "/";
  return <CheckoutSuccessDesktop redirectPath={redirectPath} />;
}
