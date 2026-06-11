"use client";

import { authClient } from "@orbis/auth/auth-client";
import { DropdownMenuItem } from "@orbis/ui/dropdown-menu";
import { useRouter } from "next/navigation";

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
