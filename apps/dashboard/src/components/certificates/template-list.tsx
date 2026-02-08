
"use client";

import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@midday/ui/card";
import { Button } from "@midday/ui/button";
import { useRouter } from "next/navigation";
import { Edit, Trash, Star, Copy, Image as ImageIcon } from "lucide-react";
import { useTRPC } from "@/trpc/client";
import { formatDate } from "date-fns";
import { Badge } from "@midday/ui/badge";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";
import { toast } from "@midday/ui/use-toast";
import { useState } from "react";
import { useMutation } from "@tanstack/react-query";
import { Icons } from "@midday/ui/icons";

export function CertificateTemplateList({ initialData }: { initialData: any[] }) {
  const router = useRouter();
  const trpc = useTRPC();
  const [templates, setTemplates] = useState(initialData);

  const deleteMutation = useMutation(trpc.certificateTemplate.delete.mutationOptions());
  const setDefaultMutation = useMutation(trpc.certificateTemplate.setDefault.mutationOptions());

  const handleDelete = async (id: string) => {
    try {
      await deleteMutation.mutateAsync({ id });
      setTemplates(templates.filter((t) => t.id !== id));
      toast({
        title: "Template deleted",
        variant: "success",
      });
    } catch (e) {
      toast({
        title: "Failed to delete template",
        variant: "error",
      });
    }
  };

  const handleSetDefault = async (id: string) => {
    try {
      await setDefaultMutation.mutateAsync({ id });
      setTemplates(templates.map((t) => ({
        ...t,
        isDefault: t.id === id
      })));
      toast({
        title: "Default template updated",
        variant: "success",
      });
    } catch (e) {
      toast({
        title: "Failed to update default template",
        variant: "error",
      });
    }
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {templates.length === 0 && (
        <div className="col-span-full text-center py-20 text-muted-foreground border-2 border-dashed rounded-xl">
           <div className="flex flex-col items-center justify-center gap-4">
             <div className="w-12 h-12 bg-secondary rounded-full flex items-center justify-center">
               <ImageIcon className="w-6 h-6" />
             </div>
             <div>
               <h3 className="text-lg font-medium">No templates found</h3>
               <p className="text-sm text-balance">Create your first certificate template to get started.</p>
             </div>
           </div>
        </div>
      )}

      {templates.map((template) => (
        <Card key={template.id} className="group hover:border-primary/50 transition-colors cursor-pointer" onClick={() => router.push(`/certificates/${template.id}`)}>
          <CardHeader className="flex flex-row justify-between items-start pb-2">
            <div>
              <CardTitle className="text-lg flex items-center gap-2">
                {template.name}
                {template.isDefault && <Badge variant="secondary" className="text-xs">Default</Badge>}
              </CardTitle>
              <div className="text-xs text-muted-foreground mt-1">
                Last updated {formatDate(new Date(template.updatedAt), "PP")}
              </div>
            </div>
          </CardHeader>
          <CardContent className="h-40 bg-secondary/20 relative m-4 rounded-md border flex items-center justify-center overflow-hidden">
             {template.backgroundImageUrl ? (
               <img src={template.backgroundImageUrl} alt={template.name} className="w-full h-full object-cover opacity-80" />
             ) : (
               <div className="flex flex-col items-center gap-2 text-muted-foreground/50">
                 <ImageIcon className="w-8 h-8" />
                 <span className="text-xs">No Preview</span>
               </div>
             )}
             
             {/* Thumbnail Overlay on Hover could go here */}
          </CardContent>
          <CardFooter className="flex justify-between items-center bg-secondary/10 py-3">
             <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <span className="capitalize">{template.size}</span>
                <span>•</span>
                <span className="capitalize">{template.orientation}</span>
             </div>
             
             <div onClick={(e) => e.stopPropagation()}>
               <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <Button variant="ghost" size="icon" className="h-8 w-8">
                      <Icons.MoreHoriz className="w-4 h-4" />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align="end">
                    <DropdownMenuItem onClick={() => router.push(`/certificates/${template.id}`)}>
                      <Edit className="mr-2 w-4 h-4" /> Edit
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => handleSetDefault(template.id)}>
                      <Star className="mr-2 w-4 h-4" /> Set as Default
                    </DropdownMenuItem>
                    <DropdownMenuItem className="text-destructive focus:text-destructive" onClick={() => handleDelete(template.id)}>
                      <Trash className="mr-2 w-4 h-4" /> Delete
                    </DropdownMenuItem>
                  </DropdownMenuContent>
               </DropdownMenu>
             </div>
          </CardFooter>
        </Card>
      ))}
    </div>
  );
}
