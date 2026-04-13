import { Popover, PopoverContent, PopoverTrigger } from "@midday/ui/popover";
import { HexColorPicker } from "react-colorful";

type Props = {
  value: string;
  onSelect: (value: string) => void;
};

export function ColorPicker({ value, onSelect }: Props) {
  return (
    <Popover>
      <PopoverTrigger asChild>
        <button
          type="button"
          className="w-full h-full rounded-sm transition-transform active:scale-95 shadow-sm"
          style={{
            backgroundColor: value,
          }}
        />
      </PopoverTrigger>
      <PopoverContent className="p-0 w-auto" sideOffset={14}>
        <HexColorPicker
          className="color-picker"
          color={value}
          onChange={(c) => {
            onSelect(c);
          }}
        />
      </PopoverContent>
    </Popover>
  );
}
