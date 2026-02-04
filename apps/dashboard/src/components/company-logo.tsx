"use client";

import { authClient } from "@midday/auth/auth-client";
import { Avatar, AvatarFallback, AvatarImage } from "@midday/ui/avatar";
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@midday/ui/card";
import { Spinner } from "@midday/ui/spinner";
import { useRef } from "react";

export function CompanyLogo() {
  const inputRef = useRef<HTMLInputElement>(null);
  const { data, isPending } = authClient.useSession();


  return (
    <Card>
      <div className="flex justify-between items-center pr-6">
        <CardHeader>
          <CardTitle>Company logo</CardTitle>
          <CardDescription>
            This is your company's logo. Click on the logo to upload a custom
            one from your files.
          </CardDescription>
        </CardHeader>

        <Avatar
          className="rounded-none w-16 h-16 flex items-center justify-center bg-accent cursor-pointer"
          onClick={() => inputRef?.current?.click()}
        >
          {isPending ? (
            <Spinner className="h-4 w-4" />
          ) : (
            <>
              <AvatarImage
                src={data?.user?.image ?? undefined}
                alt={data?.user?.name ?? undefined}
                width={64}
                height={64}
              />
              <AvatarFallback>
                <span className="text-md">{data?.user?.name?.charAt(0)}</span>
              </AvatarFallback>
            </>
          )}

          <input
            ref={inputRef}
            type="file"
            style={{ display: "none" }}
            multiple={false}
            onChange={() => {}}
          />
        </Avatar>
      </div>
      <CardFooter>An avatar is optional but strongly recommended.</CardFooter>
    </Card>
  );
}
