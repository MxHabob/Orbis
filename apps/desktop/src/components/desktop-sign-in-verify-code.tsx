"use client";


import appIcon from "public/appicon.png";
import { useEffect, useRef } from "react";

interface DesktopSignInVerifyCodeProps {
  code: string;
}

export function DesktopSignInVerifyCode({
  code,
}: DesktopSignInVerifyCodeProps) {
  const hasRunned = useRef(false);

  useEffect(() => {
    if (code && !hasRunned.current) {
      window.location.replace(`orbis://api/auth/callback?code=${code}`);
      hasRunned.current = true;
    }
  }, [code]);

  return (
    <div>
      <div className="h-screen flex flex-col items-center justify-center text-center text-sm text-[#606060]">
        <img
          src={appIcon}
          width={80}
          height={80}
          alt="Orbis"
          quality={100}
          className="mb-10"
        />
        <p>Signing in...</p>
        <p className="mb-4">
          If Orbis dosen't open in a few seconds,{" "}
          <a
            className="underline"
            href={`orbis://api/auth/callback?code=${code}`}
          >
            click here
          </a>
          .
        </p>
        <p>You may close this browser tab when done</p>
      </div>
    </div>
  );
}
