"use client";

import { authClient } from "@midday/auth/auth-client";
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@midday/ui/card";
import { AvatarUpload } from "./avatar-upload";

export function UserAvatar() {
  const { data: session } = authClient.useSession();
  const user = session?.user;

  return (
    <Card>
      <div className="flex justify-between items-center pr-6">
        <CardHeader>
          <CardTitle>Avatar</CardTitle>
          <CardDescription>
            This is your avatar. Click on the avatar to upload a custom one from
            your files.
          </CardDescription>
        </CardHeader>

        <AvatarUpload userId={user?.id!} avatarUrl={user?.image} />
      </div>
      <CardFooter>An avatar is optional but strongly recommended.</CardFooter>
    </Card> 
  );
}
