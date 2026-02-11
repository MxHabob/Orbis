import Papa from "papaparse";

export type ImportCsvResult<T> = {
  data: T[];
  meta: Papa.ParseMeta;
  errors: Papa.ParseError[];
};

export type ImportCsvOptions = Papa.ParseConfig;

export function parseCsv<T>(
  file: File | string,
  options?: ImportCsvOptions
): Promise<ImportCsvResult<T>> {
  return new Promise((resolve, reject) => {
    Papa.parse(file, {
      header: true,
      skipEmptyLines: true,
      ...options,
      complete: (results) => {
        resolve(results as ImportCsvResult<T>);
      },
      error: (error) => {
        reject(error);
      },
    });
  });
}
