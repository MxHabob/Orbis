import { Button } from "@orbis/ui/button";
import { useNavigate } from "react-router-dom";

export function EmptyState() {
  const navigate = useNavigate();

  return (
    <div className="flex items-center justify-center ">
      <div className="flex flex-col items-center mt-40">
        <div className="text-center mb-6 space-y-2">
          <h2 className="font-medium text-lg">No templates</h2>
          <p className="text-[#606060] text-sm">
            You haven't created any certificate templates yet. <br />
            Go ahead and create your first one.
          </p>
        </div>

        <Button
          variant="outline"
          onClick={() => navigate("/certificates/create")}
        >
          Create template
        </Button>
      </div>
    </div>
  );
}

export function NoResults() {
  return (
    <div className="flex items-center justify-center ">
      <div className="flex flex-col items-center mt-40">
        <div className="text-center mb-6 space-y-2">
          <h2 className="font-medium text-lg">No results</h2>
          <p className="text-[#606060] text-sm">
            Try another search, or adjusting the filters
          </p>
        </div>
      </div>
    </div>
  );
}
