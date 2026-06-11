"use client";

import { authClient } from "@orbis/auth/auth-client";
import { isDesktopApp } from "@orbis/desktop-client/platform";
import { Icons } from "@orbis/ui/icons";
import { SubmitButton } from "@orbis/ui/submit-button";
import { useState } from "react";
import { getUrl } from "@/utils/environment";

type Props = {
  showLastUsed?: boolean;
};

export function AppleSignIn({ showLastUsed = false }: Props) {
  const [isLoading, setLoading] = useState(false);

  const handleSignIn = async () => {
    setLoading(true);

    if (isDesktopApp()) {
      const redirectTo = new URL("/", getUrl());

      redirectTo.searchParams.append("provider", "apple");
      redirectTo.searchParams.append("client", "desktop");

      await authClient.signIn.social({
        provider: "apple",
        callbackURL: redirectTo.toString(),
      });
    } else {
      await authClient.signIn.social({
        provider: "apple",
        callbackURL: `${getUrl()}/`,
      });
    }

    setTimeout(() => {
      setLoading(false);
    }, 2000);
  };

  return (
    <div className="relative w-full">
      <SubmitButton
        type="button"
        onClick={handleSignIn}
        isSubmitting={isLoading}
        className="w-full bg-transparent border border-[#0e0e0e] dark:border-border text-[#0e0e0e] dark:text-foreground font-sans text-sm h-[40px] px-6 py-4 hover:bg-[#0e0e0e]/5 dark:hover:bg-border/10 transition-colors disabled:opacity-50"
      >
        <div className="flex items-center justify-center gap-2">
          <Icons.Apple size={16} />
          <span>Continue with Apple</span>
        </div>
      </SubmitButton>
      {/* Last used pill */}
      {showLastUsed && (
        <div className="absolute top-1/2 right-3 transform -translate-y-1/2 pointer-events-none">
          <span className="font-sans text-sm text-muted-foreground/50">
            Last used
          </span>
        </div>
      )}
    </div>
  );
}
