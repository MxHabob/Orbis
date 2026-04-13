"use client";

import React, { useEffect, useCallback } from "react";
import { useEditorStore } from "@/store/certificate-editor";
import { EditorSidebar } from "./editor-sidebar";
import { EditorPropertyPanel } from "./editor-property-panel";
import { EditorCanvas } from "./editor-canvas";
import { EditorFloatingToolbar } from "./editor-floating-toolbar";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useToast } from "@midday/ui/use-toast";
import { type EditorElement } from "@/types/certificate-editor";
import { useDropzone } from "react-dropzone";
import { authClient } from "@midday/auth/auth-client";
import "@/styles/certificate-editor.css";

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

export function CertificateEditor({ initialData }: Props) {
  const { setElements, setCanvasConfig, elements, canvasConfig, addElement, setName, name } = useEditorStore();
  const trpc = useTRPC();
  const { toast } = useToast();
  const queryClient = useQueryClient();
  const { data: session } = authClient.useSession();

  // Initialize store with clean data
  useEffect(() => {
    if (initialData?.name) {
      setName(initialData.name);
    }
    if (initialData?.content) {
      if (initialData.content.elements) {
        setElements(initialData.content.elements);
      }
      if (initialData.content.canvasConfig) {
        setCanvasConfig(initialData.content.canvasConfig);
      }
    }
  }, [initialData, setElements, setCanvasConfig, setName]);

  const processDocumentMutation = useMutation(
    trpc.documents.processDocument.mutationOptions(),
  );

  const handleUploadFiles = useCallback(async (files: File[]) => {
    if (!files.length) return;
    
    toast({ title: "Uploading assets...", variant: "default" });

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

      await processDocumentMutation.mutateAsync(
        results.map((r) => ({
          filePath: r.path.split("/"),
          mimetype: r.file.type,
          size: r.file.size,
        }))
      );

      results.forEach(res => {
        addElement({
          type: 'image',
          content: res.url,
          name: res.file.name,
        });
      });

      toast({ title: "Assets ready", variant: "success" });
    } catch (error) {
      toast({ title: "Upload failed", variant: "destructive" });
    }
  }, [session, addElement, processDocumentMutation, toast]);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop: handleUploadFiles,
    noClick: true, 
    accept: { "image/*": [".png", ".jpg", ".jpeg", ".svg", ".webp"] }
  });

  const upsertTemplate = useMutation(
    trpc.certificateTemplates.upsert.mutationOptions({
      onSuccess: () => {
        toast({ title: "Template saved" });
        queryClient.invalidateQueries(trpc.certificateTemplates.list.queryFilter());
      },
      onError: (error) => {
        toast({ title: "Save failed", variant: "destructive", description: error.message });
      },
    })
  );

  const handleSave = useCallback(() => {
    upsertTemplate.mutate({
      id: initialData?.id,
      name: name || "Untitled Template",
      description: initialData?.description ?? "",
      category: initialData?.category ?? "general",
      content: { elements, canvasConfig },
    });
  }, [initialData, elements, canvasConfig, upsertTemplate, name]);

  // Shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
        if (["INPUT", "TEXTAREA"].includes((e.target as HTMLElement).tagName)) return;

        const store = useEditorStore.getState();

        if ((e.metaKey || e.ctrlKey) && e.key === "c") {
            e.preventDefault();
            store.copySelection();
        }

        if ((e.metaKey || e.ctrlKey) && e.key === "v") {
            e.preventDefault();
            store.pasteSelection();
        }

        if ((e.metaKey || e.ctrlKey) && e.key === "z") {
            e.preventDefault();
            if (e.shiftKey) store.redo();
            else store.undo();
        }

        if ((e.metaKey || e.ctrlKey) && e.key === "s") {
            e.preventDefault();
            handleSave();
        }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [handleSave]);

  return (
    <div 
        {...getRootProps()}
        className="flex h-full w-full certificate-editor-root overflow-hidden font-sans outline-none text-white/90 bg-[#09090b]"
    >
      <input {...getInputProps()} />
      
      {isDragActive && (
          <div className="absolute inset-0 z-[1000] flex items-center justify-center pointer-events-none">
             <div className="glass-panel p-10 rounded-3xl flex flex-col items-center gap-6 border-white/20">
             </div>
          </div>
      )}

      <EditorSidebar />
      
      <main className="flex-1 relative flex flex-col min-w-0 bg-[#0c0c0e]">
        <div className="absolute inset-0 canvas-grid opacity-30 pointer-events-none" />
        <EditorCanvas />
        <EditorFloatingToolbar />
      </main>

      <EditorPropertyPanel onSave={handleSave} isSaving={upsertTemplate.isPending} />
    </div>
  );
}
