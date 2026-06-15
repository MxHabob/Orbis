import { Search } from "@/components/search/search";
import { SearchFooter } from "@/components/search/search-footer";

export default function DesktopSearchPage() {
  return (
    <div className="desktop-search flex flex-col h-full rounded-[10px] overflow-hidden">
      <Search />
      <SearchFooter />
    </div>
  );
}
