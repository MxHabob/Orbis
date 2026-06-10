"use client";

import { useTRPC } from "@/trpc/client";
import { ComboboxDropdown } from "@orbis/ui/combobox-dropdown";
import { Spinner } from "@orbis/ui/spinner";
import { useQuery } from "@tanstack/react-query";

type Selected = {
  id: number;
  name: string;
};

type Props = {
  parentId?: number | null;
  onChange: (selected: Selected | null) => void;
  excludeIds?: number[];
  hideLoading?: boolean;
};

export function SelectParentClass({
  parentId,
  onChange,
  excludeIds = [],
  hideLoading,
}: Props) {
  const trpc = useTRPC();
  const { data, isLoading } = useQuery(
    trpc.classes.getAll.queryOptions({}),
  );

  const parentClasses =
    data?.data
      ?.filter(
        (cls) => !cls.parentClassId && !excludeIds.includes(cls.id),
      )
      .map((cls) => ({
        id: cls.id.toString(),
        label: cls.name,
      })) ?? [];

  const selectedParent = parentId
    ? data?.data?.find((cls) => cls.id === parentId)
    : null;
    
  const selectedValue = selectedParent
    ? { id: selectedParent.id.toString(), label: selectedParent.name }
    : undefined;

  if (!selectedParent && isLoading && !hideLoading) {
    return (
      <div className="w-full h-full flex items-center justify-center py-2">
        <Spinner />
      </div>
    );
  }

  return (
    <ComboboxDropdown
      placeholder="Select parent class (Optional)"
      searchPlaceholder="Search parent class..."
      items={parentClasses}
      selectedItem={selectedValue}
      onSelect={(item) => {
        if (!item || item.id === "none") {
          onChange(null);
        } else {
          onChange({
            id: Number(item.id),
            name: item.label,
          });
        }
      }}
      renderSelectedItem={(selectedItem) => (
        <span className="text-left truncate max-w-[90%] font-normal">
          {selectedItem.label}
        </span>
      )}
      renderListItem={({ item }) => {
        return <span className="line-clamp-1">{item.label}</span>;
      }}
    />
  );
}
