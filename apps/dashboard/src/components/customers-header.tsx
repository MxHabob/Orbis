import { CustomersColumnVisibility } from "./customers-column-visibility";
import { SearchField } from "./search-field";
import { ReportActions } from "./report-actions";

export async function StudentsHeader({ id }: { id?: string }) {
  return (
    <div className="flex items-center justify-between">
      <SearchField placeholder="Search students" />

      <div className="flex items-center gap-2">
        <CustomersColumnVisibility />
        <div className="hidden sm:block">
          {id ? (
            <ReportActions classId={id} />
          ) : null}
        </div>
      </div>
    </div>
  );
}
