"use client";

import { DropdownMenuItem } from "@midday/ui/dropdown-menu";
import { useRouter } from "next/navigation";
import { authClient } from "@midday/auth/auth-client";

export function SignOut() {
  const router = useRouter();

  const handleSignOut = async () => {
    await authClient.signOut();
    router.push("/login");
  };

  return (
    <DropdownMenuItem className="text-xs" onClick={handleSignOut}>
      Sign out
    </DropdownMenuItem>
  );
}
