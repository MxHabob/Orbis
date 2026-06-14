"use client";

import { Button } from "@orbis/ui/button";
import { useNavigate } from "react-router-dom";

export function ErrorFallback() {
  const navigate = useNavigate();

  return (
    <div className="flex flex-col items-center justify-center h-full space-y-4">
      <div>
        <h2 className="text-md">Something went wrong</h2>
      </div>
      <Button onClick={() => window.location.reload()} variant="outline">
        Try again
      </Button>
    </div>
  );
}
