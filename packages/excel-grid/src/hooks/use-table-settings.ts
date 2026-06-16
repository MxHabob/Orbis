import type {
  ColumnOrderState,
  ColumnSizingState,
  VisibilityState,
} from "@tanstack/react-table";
import { useCallback, useEffect, useRef, useState } from "react";

export const TABLE_SETTINGS_COOKIE = "table-settings";

export interface TableSettings {
  columns: VisibilityState;
  sizing: ColumnSizingState;
  order: ColumnOrderState;
}

interface UseTableSettingsProps {
  tableId: string;
  initialSettings?: Partial<TableSettings>;
  defaultHiddenColumns?: string[];
}

interface UseTableSettingsReturn {
  columnVisibility: VisibilityState;
  setColumnVisibility: React.Dispatch<React.SetStateAction<VisibilityState>>;
  columnSizing: ColumnSizingState;
  setColumnSizing: React.Dispatch<React.SetStateAction<ColumnSizingState>>;
  columnOrder: ColumnOrderState;
  setColumnOrder: React.Dispatch<React.SetStateAction<ColumnOrderState>>;
}

export function useTableSettings({
  tableId,
  initialSettings,
  defaultHiddenColumns = [],
}: UseTableSettingsProps): UseTableSettingsReturn {
  
  // Merge defaults
  const getDefaultSettings = (): TableSettings => {
    const visibility: VisibilityState = {};
    for (const col of defaultHiddenColumns) {
      visibility[col] = false;
    }
    return {
      columns: visibility,
      sizing: {},
      order: [],
    };
  };

  const defaults = getDefaultSettings();
  
  const mergeWithDefaults = (saved: Partial<TableSettings> | undefined): TableSettings => {
    const settings = {
      columns: saved?.columns ?? defaults.columns,
      sizing: saved?.sizing ?? defaults.sizing,
      order: saved?.order ?? defaults.order,
    };

    if (tableId === "grades" && settings.order.length > 0) {
      const hasTotal = settings.order.includes("total");
      if (hasTotal) {
        settings.order = [
          ...settings.order.filter((id) => id !== "total"),
          "total",
        ];
      }
    }

    return settings;
  };

  const settings = mergeWithDefaults(initialSettings);

  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>(
    settings.columns,
  );
  const [columnSizing, setColumnSizing] = useState<ColumnSizingState>(
    settings.sizing,
  );
  const [columnOrder, setColumnOrder] = useState<ColumnOrderState>(
    settings.order,
  );

  const isInitialMount = useRef(true);
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const persistSettings = useCallback(
    (
      visibility: VisibilityState,
      sizing: ColumnSizingState,
      order: ColumnOrderState,
    ) => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }

      debounceRef.current = setTimeout(async () => {
        try {
          const existingCookie = document.cookie
            .split("; ")
            .find((row) => row.startsWith(`${TABLE_SETTINGS_COOKIE}=`));

          let allSettings: Record<string, Partial<TableSettings>> = {};
          if (existingCookie) {
            try {
              allSettings = JSON.parse(
                decodeURIComponent(existingCookie.split("=")[1] ?? "{}"),
              );
            } catch {
              allSettings = {};
            }
          }

          allSettings[tableId] = {
            columns: visibility,
            sizing: sizing,
            order: order,
          };

          document.cookie = `${TABLE_SETTINGS_COOKIE}=${encodeURIComponent(JSON.stringify(allSettings))};path=/;max-age=31536000`;
        } catch (error) {
          console.error("Failed to persist table settings:", error);
        }
      }, 300);
    },
    [tableId],
  );

  useEffect(() => {
    if (isInitialMount.current) {
      isInitialMount.current = false;
      return;
    }

    persistSettings(columnVisibility, columnSizing, columnOrder);
  }, [columnVisibility, columnSizing, columnOrder, persistSettings]);

  useEffect(() => {
    return () => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
    };
  }, []);

  return {
    columnVisibility,
    setColumnVisibility,
    columnSizing,
    setColumnSizing,
    columnOrder,
    setColumnOrder,
  };
}
