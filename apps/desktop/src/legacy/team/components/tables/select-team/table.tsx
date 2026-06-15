import type { RouterOutputs } from "@/ipc/query-adapter";
import { Table, TableBody } from "@orbis/ui/table";
import { TableRow } from "./table-row";

type Props = {
  data: RouterOutputs["team"]["list"];
};

export function SelectTeamTable({ data }: Props) {
  return (
    <Table>
      <TableBody className="border-none">
        {data.map((row) => (
          <TableRow key={row.id} row={row} />
        ))}
      </TableBody>
    </Table>
  );
}
