"use client";

import React, { useEffect, useCallback } from "react";
import { useEditorStore } from "./store/use-editor-store";
import { Sidebar } from "./components/sidebar";
import { PropertyPanel } from "./components/property-panel";
import { Canvas } from "./components/canvas";
import { FloatingToolbar } from "./components/floating-toolbar";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useToast } from "@midday/ui/use-toast";
import { type EditorElement } from "./types";
import { useDropzone } from "react-dropzone";
import { authClient } from "@midday/auth/auth-client";

interface Props {
  mode?: "edit" | "create";
  initialData?: {
    id?: string;
    name?: string;
    description?: string;
    category?: string;
    tags?: string[];
    thumbnailUrl?: string;
    content?: { 
       elements: EditorElement[];
       canvasConfig?: any;
    };
  };
}

export function ModernCertificateEditor({ initialData }: Props) {
  const { setElements, setCanvasConfig, elements, canvasConfig, addElement } = useEditorStore();
  const trpc = useTRPC();
  const { toast } = useToast();
  const queryClient = useQueryClient();
  const { data: session } = authClient.useSession();

  // Initialize store
  useEffect(() => {
    if (initialData?.content) {
      if (initialData.content.elements) {
        setElements(initialData.content.elements);
      }
      if (initialData.content.canvasConfig) {
        setCanvasConfig(initialData.content.canvasConfig);
      }
    }
  }, [initialData]);

  // Real Upload Logic (following packages/storage logic)
  const processDocumentMutation = useMutation(
    trpc.documents.processDocument.mutationOptions(),
  );

  const handleUploadFiles = useCallback(async (files: File[]) => {
    if (!files.length) return;
    
    toast({ title: "Uploading to cloud...", variant: "default" });

    try {
      const results = await Promise.all(
        files.map(async (file) => {
          const formData = new FormData();
          formData.append("file", file);
          formData.append("bucket", "vault");
          formData.append("path", `${session?.user.id || "anonymous"}/${file.name}`);

          const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/upload`, {
            method: "POST",
            body: formData,
            credentials: "include",
          });

          if (!response.ok) throw new Error("Upload failed");
          const data = await response.json();
          return { file, path: data.path, url: data.url };
        })
      );

      // Save to Global Vault (Documents table)
      await processDocumentMutation.mutateAsync(
        results.map((r) => ({
          filePath: r.path.split("/"),
          mimetype: r.file.type,
          size: r.file.size,
        }))
      );

      // Add to Canvas at drop position (if applicable) or center
      results.forEach(res => {
        addElement({
          type: 'image',
          content: res.url,
          width: 400,
          height: 400,
          name: res.file.name,
          x: 200,
          y: 200
        });
      });

      toast({ title: "Assets uploaded & added", variant: "success" });
    } catch (error) {
      toast({ title: "Upload failed", variant: "destructive" });
    }
  }, [session, addElement, processDocumentMutation]);

  // Handle Paste
  useEffect(() => {
    const handlePaste = (e: ClipboardEvent) => {
      const items = e.clipboardData?.items;
      if (!items) return;

      const files: File[] = [];
      for (const item of Array.from(items)) {
        if (item.type.indexOf("image") !== -1) {
          const file = item.getAsFile();
          if (file) files.push(file);
        }
      }
      
      if (files.length > 0) {
        handleUploadFiles(files);
      }
    };

    window.addEventListener("paste", handlePaste);
    return () => window.removeEventListener("paste", handlePaste);
  }, [handleUploadFiles]);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop: handleUploadFiles,
    noClick: true, // Allow clicks for selection
    accept: { "image/*": [".png", ".jpg", ".jpeg", ".svg", ".webp"] }
  });

  const upsertTemplate = useMutation(
    trpc.certificateTemplates.upsert.mutationOptions({
      onSuccess: () => {
        toast({ title: "Design saved to cloud" });
        queryClient.invalidateQueries(trpc.certificateTemplates.list.queryFilter());
      },
      onError: (error) => {
        toast({ title: "Sync failed", variant: "destructive", description: error.message });
      },
    })
  );

  const handleSave = () => {
    upsertTemplate.mutate({
      id: initialData?.id,
      name: initialData?.name ?? "Untitled",
      description: initialData?.description ?? "",
      category: initialData?.category ?? "design",
      content: { elements, canvasConfig },
    });
  };

  return (
    <div 
        {...getRootProps()}
        className="flex h-screen w-full bg-[#121212] overflow-hidden font-sans text-white/90 outline-none"
    >
      <input {...getInputProps()} />
      
      {isDragActive && (
          <div className="absolute inset-0 z-[1000] bg-[#0c8ce9]/20 backdrop-blur-sm border-4 border-[#0c8ce9] border-dashed flex items-center justify-center pointer-events-none">
             <div className="bg-[#1e1e1e] p-8 rounded-2xl shadow-2xl flex flex-col items-center gap-4 border border-white/10">
                <div className="w-16 h-16 rounded-full bg-[#0c8ce9]/10 flex items-center justify-center">
                    <Icons.Upload className="w-8 h-8 text-[#0c8ce9]" />
                </div>
                <p className="text-xl font-bold">Drop to add images to design</p>
                <p className="text-sm text-white/40 tracking-wider font-bold">RELEASE TO UPLOAD TO CLOUD</p>
             </div>
          </div>
      )}

      <Sidebar templateName={initialData?.name || "Untitled"} />
      
      <main className="flex-1 relative flex flex-col min-w-0">
        <Canvas />
        <FloatingToolbar />
      </main>

      <PropertyPanel onSave={handleSave} isSaving={upsertTemplate.isPending} />
    </div>
  );
}

const Icons = {
    Upload: (props: any) => (
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" {...props}>
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
            <polyline points="17 8 12 3 7 8" />
            <line x1="12" y1="3" x2="12" y2="15" />
        </svg>
    )
}
