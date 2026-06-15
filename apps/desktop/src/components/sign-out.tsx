"use client";

import { authClient } from "@/lib/auth-client";
import { DropdownMenuItem } from "@orbis/ui/dropdown-menu";
import { useNavigate } from "react-router-dom";

export function SignOut() {
  const navigate = useNavigate();

  const handleSignOut = async () => {
    await authClient.signOut();
    navigate("/login");
  };

  return (
    <DropdownMenuItem className="text-xs" onClick={handleSignOut}>
      Sign out
    </DropdownMenuItem>
  );
}
