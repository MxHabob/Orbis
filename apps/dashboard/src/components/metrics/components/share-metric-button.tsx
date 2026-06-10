"use client";

import { Button } from "@orbis/ui/button";
import { Icons } from "@orbis/ui/icons";

interface ShareMetricButtonProps {
  type: string;
  from: string;
  to: string;
  currency?: string;
}

export function ShareMetricButton({ type, from, to, currency }: ShareMetricButtonProps) {
  // return (
  //   <Button
  //     variant="ghost"
  //     size="icon"
  //     className="h-8 w-8"
  //     onClick={() => {
  //       console.log("Sharing metric:", { type, from, to, currency });
  //     }}
  //   >
  //     <Icons.Share className="h-4 w-4" />
  //   </Button>
  // );
  return null;
}
