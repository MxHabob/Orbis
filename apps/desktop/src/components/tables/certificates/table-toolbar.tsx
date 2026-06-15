import { Button } from "@orbis/ui/button";
import { Icons } from "@orbis/ui/icons";
import { Input } from "@orbis/ui/input";
import { Plus } from "lucide-react";
import { Link } from "react-router-dom";
import { useNavigate } from "react-router-dom";
import { useRef, useState } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useCertificateFilterParams } from "@/hooks/use-certificate-filter-params";

export function TableToolbar() {
  const { filter, setFilter } = useCertificateFilterParams();
  const navigate = useNavigate();
  const [input, setInput] = useState(filter.q ?? "");
  const [exportOpen, setExportOpen] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  useHotkeys(
    "esc",
    () => {
      setInput("");
      setFilter({ q: null });
    },
    {
      enableOnFormTags: true,
      enabled: Boolean(input),
    },
  );

  useHotkeys("meta+s", (evt) => {
    evt.preventDefault();
    inputRef.current?.focus();
  });

  const handleSearch = (evt: React.ChangeEvent<HTMLInputElement>) => {
    const value = evt.target.value;
    if (value) {
      setInput(value);
    } else {
      setFilter({ q: null });
      setInput("");
    }
  };

  const handleSubmit = (e?: React.FormEvent) => {
    e?.preventDefault();
    setFilter({ q: input.length > 0 ? input : null });
  };

  return (
    <div className="flex items-center py-4 justify-between">
      <form
        className="relative flex-1 sm:flex-initial"
        onSubmit={(e) => {
          e.preventDefault();
          handleSubmit();
        }}
      >
        <Icons.Search className="absolute pointer-events-none left-3 top-[11px]" />
        <Input
          ref={inputRef}
          placeholder="Search templates..."
          className="pl-9 w-full sm:w-[350px] pr-8"
          value={input}
          onChange={handleSearch}
          autoComplete="off"
          autoCapitalize="none"
          autoCorrect="off"
          spellCheck="false"
        />
      </form>

      <div className="flex items-center gap-2">
        <Button
          size="icon"
          variant="outline"
          className="size-[36px] shrink-0"
          asChild
        >
          <Link to="/certificates/create" target="_blank">
            <Plus className="w-4 h-4" />
          </Link>
        </Button>
      </div>
    </div>
  );
}
