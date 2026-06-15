import { parseFile } from "@orbis/import";
import { cn } from "@orbis/ui/cn";
import { Spinner } from "@orbis/ui/spinner";
import { useCallback, useRef, useState } from "react";
import Dropzone, { type FileRejection } from "react-dropzone";
import { Controller } from "react-hook-form";
import { useCsvContext } from "./context";

export function SelectFile() {
  const { watch, control, setFileColumns, setFirstRows, onFileProcessed } =
    useCsvContext();
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const processingRef = useRef<string | null>(null);

  const file = watch("file");

  const processFile = useCallback(
    async (fileToProcess: File) => {
      // Prevent duplicate processing of the same file
      const fileKey = `${fileToProcess.name}-${fileToProcess.size}-${fileToProcess.lastModified}`;
      if (processingRef.current === fileKey) {
        return;
      }

      if (!fileToProcess) {
        setFileColumns(null);
        return;
      }

      processingRef.current = fileKey;
      setIsLoading(true);
      setError(null);

      try {
        const { data, meta } = await parseFile(fileToProcess, {
          header: true,
          skipEmptyLines: true,
        });

        if (!data || data.length < 2) {
          setError("File must have at least 2 rows.");
          setFileColumns(null);
          setFirstRows(null);
          setIsLoading(false);
          return;
        }

        if (!meta || !meta.fields || meta.fields.length <= 1) {
          setError("Failed to retrieve column data.");
          setFileColumns(null);
          setFirstRows(null);
          setIsLoading(false);
          return;
        }

        setFileColumns(meta.fields);
        // @ts-expect-error
        setFirstRows(data.slice(0, 5)); // Take first 5 rows for preview
        setIsLoading(false);
        processingRef.current = null;
        onFileProcessed?.();
      } catch (err: any) {
        console.error("Error processing file:", err);
        setError(err.message || "Failed to read file.");
        setFileColumns(null);
        setFirstRows(null);
        setIsLoading(false);
        processingRef.current = null;
      }
    },
    [setFileColumns, setFirstRows],
  );

  return (
    <div className="flex flex-col gap-3">
      <Controller
        control={control}
        name="file"
        render={({ field: { onChange, onBlur } }) => (
          <Dropzone
            onDrop={(acceptedFiles) => {
              const file = acceptedFiles[0];
              if (file) {
                onChange(file);
                // Process file immediately to avoid waiting for watch to update
                processFile(file);
              }
            }}
            onDropRejected={(fileRejections: FileRejection[]) => {
              const rejection = fileRejections[0];
              if (rejection) {
                const error = rejection.errors[0];
                if (error?.code === "file-invalid-type") {
                  setError("Please select a CSV or Excel file.");
                } else if (error?.code === "file-too-large") {
                  setError("File size exceeds 5MB limit.");
                } else {
                  setError("File rejected. Please try again.");
                }
                console.error("File rejected:", rejection.errors);
              }
            }}
            maxFiles={1}
            accept={{
              "text/csv": [".csv"],
              "application/csv": [".csv"],
              "text/plain": [".csv"],
              "application/vnd.ms-excel": [".csv", ".xls"],
              "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet":
                [".xlsx"],
            }}
            maxSize={5000000}
          >
            {({ getRootProps, getInputProps, isDragActive, isDragReject }) => (
              <div
                {...getRootProps()}
                className={cn(
                  "w-full border border-dashed h-[200px] mt-8 mb-8 flex items-center justify-center",
                  isDragActive && "bg-secondary text-primary",
                  isDragReject && "border-destructive",
                )}
              >
                <div className="text-center flex items-center justify-center flex-col text-xs text-[#878787]">
                  <input {...getInputProps()} onBlur={onBlur} />

                  {isLoading ? (
                    <div className="flex space-x-1 items-center">
                      <Spinner />
                      <span>Loading...</span>
                    </div>
                  ) : (
                    <div>
                      <p>Drop your file here, or click to browse.</p>
                      <span>5MB file limit. </span>
                      <span className="mt-2 text-[10px]">
                        CSV or Excel format
                      </span>
                    </div>
                  )}

                  {error && (
                    <p className="text-center text-sm text-red-600 mt-4">
                      {error}
                    </p>
                  )}
                </div>
              </div>
            )}
          </Dropzone>
        )}
      />
    </div>
  );
}
