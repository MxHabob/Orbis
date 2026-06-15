import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger,
} from "@orbis/ui/context-menu";
import {
  ArrowDown,
  ArrowUp,
  ChevronDown,
  ChevronUp,
  Copy,
  Group,
  Layers,
  Lock,
  MousePointer2,
  Trash2,
  Zap,
} from "lucide-react";
import type React from "react";
import { useEditorStore } from "@/store/certificate-editor";

interface Props {
  children: React.ReactNode;
}

export function EditorContextMenu({ children }: Props) {
  const {
    selectedElementIds,
    elements,
    deleteElements,
    duplicateElements,
    bringToFront,
    sendToBack,
    bringForward,
    sendBackward,
    groupElements,
    ungroupElements,
  } = useEditorStore();

  const selectionCount = selectedElementIds.length;
  const isSelected = selectionCount > 0;
  const firstSelected = elements.find((el) => el.id === selectedElementIds[0]);
  const isGroupSelected =
    selectedElementIds.length === 1 && firstSelected?.type === "group";

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>{children}</ContextMenuTrigger>

      <ContextMenuContent className="w-64 bg-[#2c2c2c] border-[#444] text-white/90 shadow-2xl p-1 rounded-xl">
        {isSelected ? (
          <>
            <ContextMenuItem
              onClick={() => duplicateElements(selectedElementIds)}
              className="gap-2 focus:bg-blue-600 focus:text-white rounded-lg text-[11px] font-medium py-1.5 px-2"
            >
              <Copy className="w-3.5 h-3.5" />
              Duplicate
              <ContextMenuShortcut className="text-white/40">
                ⌘D
              </ContextMenuShortcut>
            </ContextMenuItem>

            <ContextMenuItem
              onClick={() => deleteElements(selectedElementIds)}
              className="gap-2 focus:bg-red-500/20 focus:text-red-400 text-red-400 rounded-lg text-[11px] font-medium py-1.5 px-2"
            >
              <Trash2 className="w-3.5 h-3.5" />
              Delete
              <ContextMenuShortcut className="text-red-400/40">
                ⌫
              </ContextMenuShortcut>
            </ContextMenuItem>

            <ContextMenuSeparator className="bg-white/10 my-1" />

            <ContextMenuSub>
              <ContextMenuSubTrigger className="gap-2 focus:bg-blue-600 focus:text-white rounded-lg text-[11px] font-medium py-1.5 px-2">
                <Layers className="w-3.5 h-3.5" />
                Arrange
              </ContextMenuSubTrigger>
              <ContextMenuSubContent className="bg-[#2c2c2c] border-[#444] text-white/90 p-1 rounded-xl">
                {(() => {
                  const firstId = selectedElementIds[0];
                  if (!firstId) return null;

                  return (
                    <>
                      <ContextMenuItem
                        onClick={() => bringToFront(firstId)}
                        className="gap-2 focus:bg-blue-600 focus:text-white py-1.5 px-2 text-[11px] rounded-lg"
                      >
                        <ArrowUp className="w-3.5 h-3.5" />
                        Bring to Front
                      </ContextMenuItem>
                      <ContextMenuItem
                        onClick={() => bringForward(firstId)}
                        className="gap-2 focus:bg-blue-600 focus:text-white py-1.5 px-2 text-[11px] rounded-lg"
                      >
                        <ChevronUp className="w-3.5 h-3.5" />
                        Bring Forward
                      </ContextMenuItem>
                      <ContextMenuItem
                        onClick={() => sendBackward(firstId)}
                        className="gap-2 focus:bg-blue-600 focus:text-white py-1.5 px-2 text-[11px] rounded-lg"
                      >
                        <ChevronDown className="w-3.5 h-3.5" />
                        Send Backward
                      </ContextMenuItem>
                      <ContextMenuItem
                        onClick={() => sendToBack(firstId)}
                        className="gap-2 focus:bg-blue-600 focus:text-white py-1.5 px-2 text-[11px] rounded-lg"
                      >
                        <ArrowDown className="w-3.5 h-3.5" />
                        Send to Back
                      </ContextMenuItem>
                    </>
                  );
                })()}
              </ContextMenuSubContent>
            </ContextMenuSub>

            <ContextMenuSeparator className="bg-white/10 my-1" />

            {isGroupSelected ? (
              <ContextMenuItem
                onClick={() => ungroupElements(firstSelected!.id)}
                className="gap-2 focus:bg-blue-600 focus:text-white rounded-lg text-[11px] font-medium py-1.5 px-2"
              >
                <Group className="w-3.5 h-3.5 rotate-180" />
                Ungroup Selection
                <ContextMenuShortcut className="text-white/40">
                  ⇧⌘G
                </ContextMenuShortcut>
              </ContextMenuItem>
            ) : (
              <ContextMenuItem
                onClick={() => groupElements()}
                className="gap-2 focus:bg-blue-600 focus:text-white rounded-lg text-[11px] font-medium py-1.5 px-2"
              >
                <Group className="w-3.5 h-3.5" />
                Group Selection
                <ContextMenuShortcut className="text-white/40">
                  ⌘G
                </ContextMenuShortcut>
              </ContextMenuItem>
            )}

            <ContextMenuItem className="gap-2 focus:bg-blue-600 focus:text-white rounded-lg text-[11px] font-medium py-1.5 px-2">
              <Lock className="w-3.5 h-3.5" />
              Lock Selection
              <ContextMenuShortcut className="text-white/40">
                ⌘L
              </ContextMenuShortcut>
            </ContextMenuItem>

            <ContextMenuSeparator className="bg-white/10 my-1" />

            <ContextMenuItem className="gap-2 focus:bg-emerald-500/20 focus:text-emerald-400 rounded-lg text-[11px] font-bold py-1.5 px-2">
              <Zap className="w-3.5 h-3.5" />
              Add Interaction
            </ContextMenuItem>
          </>
        ) : (
          <>
            <ContextMenuItem className="gap-2 focus:bg-blue-600 focus:text-white rounded-lg text-[11px] font-medium py-1.5 px-2">
              <MousePointer2 className="w-3.5 h-3.5" />
              Select All
              <ContextMenuShortcut className="text-white/40">
                ⌘A
              </ContextMenuShortcut>
            </ContextMenuItem>
            <ContextMenuSeparator className="bg-white/10 my-1" />
            <ContextMenuItem className="gap-2 focus:bg-blue-600 focus:text-white rounded-lg text-[11px] font-medium py-1.5 px-2">
              Paste Here
              <ContextMenuShortcut className="text-white/40">
                ⌘V
              </ContextMenuShortcut>
            </ContextMenuItem>
          </>
        )}
      </ContextMenuContent>
    </ContextMenu>
  );
}
