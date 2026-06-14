"use client";

import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui/avatar";
import { cn } from "@orbis/ui/cn";
import { Icons } from "@orbis/ui/icons";
import { stripSpecialCharacters } from "@orbis/utils";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { forwardRef, useRef, useState } from "react";
import { useUpload } from "@/hooks/use-upload";
import { useTRPC } from "@/trpc/client";

type Props = {
  userId: string;
  avatarUrl?: string | null;
  onUpload?: (url: string) => void;
  size?: number;
  className?: string;
  folder?: string;
  skipUserUpdate?: boolean;
};

export const AvatarUpload = forwardRef<HTMLInputElement, Props>(
  (
    {
      userId,
      avatarUrl: initialAvatarUrl,
      size = 65,
      onUpload,
      className,
      folder,
      skipUserUpdate,
    },
    ref,
  ) => {
    const [avatar, setAvatar] = useState(initialAvatarUrl);
    const inputRef = useRef<HTMLInputElement>(null);
    const trpc = useTRPC();
    const queryClient = useQueryClient();

    const updateUserMutation = useMutation(
      trpc.user.update.mutationOptions({
        onSuccess: () => {
          queryClient.invalidateQueries({
            queryKey: trpc.user.me.queryKey(),
          });
        },
      }),
    );

    const { isLoading, uploadFile } = useUpload();

    const handleUpload = async (evt: React.ChangeEvent<HTMLInputElement>) => {
      const { files } = evt.target;
      const selectedFile = files as FileList;

      const filename = stripSpecialCharacters(selectedFile[0]?.name ?? "");

      const { url } = await uploadFile({
        bucket: "avatars",
        path: folder ? [folder, userId, filename] : [userId, filename],
        file: selectedFile[0] as File,
      });

      if (url) {
        if (!skipUserUpdate) {
          updateUserMutation.mutate({ image: url });
        }
        setAvatar(url);
        onUpload?.(url);
      }
    };

    const fileInputRef = ref || inputRef;

    return (
      <Avatar
        className={cn(
          "rounded-full flex items-center justify-center bg-accent cursor-pointer border border-border",
          className,
        )}
        style={{ width: size, height: size }}
        onClick={() => {
          if ("current" in fileInputRef && fileInputRef.current) {
            fileInputRef.current.click();
          }
        }}
      >
        {isLoading ? (
          <Loader2 className="size-4 animate-spin" />
        ) : (
          <>
            <AvatarImage src={avatar ?? undefined} />
            <AvatarFallback>
              <Icons.AccountCircle className="size-5" />
            </AvatarFallback>
          </>
        )}
        <input
          ref={fileInputRef}
          type="file"
          style={{ display: "none" }}
          multiple={false}
          onChange={handleUpload}
        />
      </Avatar>
    );
  },
);

AvatarUpload.displayName = "AvatarUpload";
