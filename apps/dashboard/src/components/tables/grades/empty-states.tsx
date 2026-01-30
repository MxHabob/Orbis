export function EmptyState() {
  return (
    <div className="flex flex-col items-center justify-center p-20 border-2 border-dashed rounded-2xl bg-muted/5 text-muted-foreground animate-in fade-in duration-500">
      <div className="h-16 w-16 rounded-full bg-muted/10 flex items-center justify-center mb-4 text-3xl">
        📊
      </div>
      <p className="font-medium text-lg">No students found</p>
      <p className="text-sm">
        There are no students enrolled in this class for the selected academic year.
      </p>
    </div>
  );
}

export function NoResults() {
  return (
    <div className="flex flex-col items-center justify-center p-20 border border-dashed rounded-xl bg-muted/5 text-muted-foreground animate-in fade-in duration-300">
      <div className="h-12 w-12 rounded-full bg-muted/20 flex items-center justify-center mb-4 text-2xl">
        🔍
      </div>
      <p className="font-semibold text-lg">No results found</p>
      <p className="text-sm">Try adjusting your filters</p>
    </div>
  );
}
