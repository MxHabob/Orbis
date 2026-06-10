"use client";

import { DropdownMenuItem } from "@orbis/ui/dropdown-menu";
import { useRouter } from "next/navigation";
import { authClient } from "@orbis/auth/auth-client";

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
