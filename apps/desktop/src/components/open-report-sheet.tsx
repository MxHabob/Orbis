import { Button } from "@orbis/ui/button";
import { Icons } from "@orbis/ui/icons";
import { useInvoiceParams } from "@/hooks/use-report-params";

export function OpenReportsSheet() {
  const { setParams } = useInvoiceParams();

  return (
    <div>
      <Button
        variant="outline"
        size="icon"
        onClick={() => setParams({ type: "create" })}
      >
        <Icons.Add />
      </Button>
    </div>
  );
}
