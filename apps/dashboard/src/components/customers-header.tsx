import { CustomersColumnVisibility } from "./customers-column-visibility";
import { OpenStudentSheet } from "./open-customer-sheet";
import { SearchField } from "./search-field";

export async function StudentsHeader() {
  return (
    <div className="flex items-center justify-between">
      <SearchField placeholder="Search students" />

      <div className="flex items-center gap-2">
        <CustomersColumnVisibility />
        <div className="hidden sm:block">
          <OpenStudentSheet />
        </div>
      </div>
    </div>
  );
}
