import { cn } from "@orbis/ui/cn";
import { useQuery } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { useEffect } from "react";
import { useTRPC } from "@/ipc/query-adapter";

interface GradesTableProps {
  studentId?: string;
  type?: string;
  month?: number;
  className?: string;
  isPreview?: boolean;
  isPlaceholder?: boolean;
  onLoad?: () => void;
  fontSize?: number;
}

// Print-safe grade colors using solid colors (no opacity modifiers that fail in html2canvas)
const GRADE_COLORS: Record<string, { text: string; bg: string }> = {
  ممتاز: { text: "#047857", bg: "#ecfdf5" },
  "جيد جداً": { text: "#1d4ed8", bg: "#eff6ff" },
  جيد: { text: "#b45309", bg: "#fffbeb" },
  مقبول: { text: "#c2410c", bg: "#fff7ed" },
  راسب: { text: "#dc2626", bg: "#fef2f2" },
};

const DUMMY_DATA = {
  columns: ["شهر 1", "شهر 2", "مواظبة"],
  rows: [
    {
      subject: "لغة عربية",
      grades: { "شهر 1": 15, "شهر 2": 18, مواظبة: 5 },
      total: 38,
      maxTotal: 40,
      gradeLabel: "ممتاز",
    },
    {
      subject: "رياضيات",
      grades: { "شهر 1": 12, "شهر 2": 14, مواظبة: 4 },
      total: 30,
      maxTotal: 40,
      gradeLabel: "جيد جداً",
    },
    {
      subject: "علوم",
      grades: { "شهر 1": 18, "شهر 2": 19, مواظبة: 5 },
      total: 42,
      maxTotal: 50,
      gradeLabel: "ممتاز",
    },
    {
      subject: "إنجليزي",
      grades: { "شهر 1": 10, "شهر 2": 11, مواظبة: 3 },
      total: 24,
      maxTotal: 30,
      gradeLabel: "جيد",
    },
  ],
  type: "monthly",
};

export function GradesTable({
  studentId,
  type,
  month,
  className,
  isPreview,
  isPlaceholder,
  onLoad,
  fontSize = 10,
}: GradesTableProps) {
  const trpc = useTRPC();

  const { data: realData, isLoading } = useQuery(
    trpc.grades.getReportData.queryOptions(
      { studentId: studentId!, type, month },
      {
        enabled: !!studentId && !isPlaceholder,
        staleTime: 1000 * 60 * 5,
      },
    ),
  );

  const data = isPlaceholder ? DUMMY_DATA : realData;

  useEffect(() => {
    if (!isLoading || isPlaceholder) {
      onLoad?.();
    }
  }, [isLoading, data, onLoad, isPlaceholder]);

  if (isLoading && !isPlaceholder) {
    return (
      <div
        style={{
          padding: "24px",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          gap: "8px",
          background: "#ffffff",
          border: "1px solid #e5e7eb",
          borderRadius: "8px",
        }}
      >
        <Loader2
          className="w-4 h-4 animate-spin"
          style={{ color: "#9ca3af" }}
        />
        <span style={{ fontSize: "11px", color: "#9ca3af" }}>
          جاري تحميل الدرجات...
        </span>
      </div>
    );
  }

  const columns = data?.columns || [];
  const rows = data?.rows || [];

  if (rows.length === 0) {
    return (
      <div
        style={{
          padding: "24px",
          textAlign: "center",
          background: "#ffffff",
          border: "1px dashed #d1d5db",
          borderRadius: "8px",
          color: "#9ca3af",
          fontSize: "11px",
          direction: "rtl",
        }}
      >
        لا توجد بيانات درجات مسجلة لهذا الطالب في هذا الوقت.
      </div>
    );
  }

  const hasGradeLabel = rows.some((row: any) => row.gradeLabel);
  const totalFinalScore = rows.reduce(
    (acc: number, curr: any) => acc + (curr.total || 0),
    0,
  );
  const totalMaxScore = rows.reduce(
    (acc: number, curr: any) => acc + (curr.maxTotal || 0),
    0,
  );

  // Dynamic font sizes
  const headerSize = `${Math.max(6, fontSize - 1)}px`;
  const bodySize = `${fontSize}px`;
  const badgeSize = `${Math.max(6, fontSize - 2)}px`;
  const footerLabelSize = `${fontSize}px`;
  const footerValueSize = `${Math.max(10, fontSize + 2)}px`;

  return (
    <div
      className={cn("w-full flex flex-col", className)}
      dir="rtl"
      style={{ color: "#18181b" }}
    >
      <div
        style={{
          width: "100%",
          background: "#ffffff",
          borderRadius: "6px",
          border: "1.5px solid #e4e4e7",
          overflow: "hidden",
        }}
      >
        <table
          style={{
            width: "100%",
            borderCollapse: "collapse",
            tableLayout: "fixed",
          }}
        >
          <thead>
            <tr
              style={{
                fontSize: headerSize,
                borderBottom: "2px solid #d4d4d8",
                background: "#f4f4f5",
              }}
            >
              <th
                style={{
                  padding: "6px 10px",
                  textAlign: "right",
                  color: "#52525b",
                  fontWeight: 700,
                  borderLeft: "1px solid #e4e4e7",
                  width: "25%",
                }}
              >
                المادة
              </th>
              {columns.map((col: string) => (
                <th
                  key={col}
                  style={{
                    padding: "6px 4px",
                    textAlign: "center",
                    borderLeft: "1px solid #e4e4e7",
                    color: "#52525b",
                    fontWeight: 600,
                  }}
                >
                  {col}
                </th>
              ))}
              <th
                style={{
                  padding: "6px 4px",
                  textAlign: "center",
                  fontWeight: 700,
                  color: "#18181b",
                  background: "#e4e4e7",
                  borderLeft: "1px solid #d4d4d8",
                  width: "12%",
                }}
              >
                المجموع
              </th>
              {hasGradeLabel && (
                <th
                  style={{
                    padding: "6px 4px",
                    textAlign: "center",
                    fontWeight: 700,
                    color: "#18181b",
                    background: "#e4e4e7",
                    width: "15%",
                  }}
                >
                  التقدير
                </th>
              )}
            </tr>
          </thead>

          <tbody>
            {rows.map((row: any, i: number) => (
              <tr
                key={i}
                style={{
                  fontSize: bodySize,
                  borderBottom:
                    i < rows.length - 1 ? "1px solid #f4f4f5" : "none",
                  textAlign: "center",
                }}
              >
                <td
                  style={{
                    padding: "5px 10px",
                    textAlign: "right",
                    fontWeight: 700,
                    color: "#27272a",
                    borderLeft: "1px solid #f4f4f5",
                  }}
                >
                  {row.subject}
                </td>

                {columns.map((col: string) => (
                  <td
                    key={col}
                    style={{
                      padding: "5px 4px",
                      borderLeft: "1px solid #f4f4f5",
                      color: row.grades[col] != null ? "#3f3f46" : "#d4d4d8",
                      textAlign: "center",
                    }}
                  >
                    {row.grades[col] != null ? (
                      <span style={{ fontWeight: 500 }}>{row.grades[col]}</span>
                    ) : (
                      <span>-</span>
                    )}
                  </td>
                ))}

                <td
                  style={{
                    padding: "5px 4px",
                    fontWeight: 700,
                    color: "#18181b",
                    background: "#fafafa",
                    borderLeft: "1px solid #f4f4f5",
                    textAlign: "center",
                  }}
                >
                  {row.total || 0}
                </td>

                {hasGradeLabel && (
                  <td style={{ padding: "5px 4px", textAlign: "center" }}>
                    {row.gradeLabel ? (
                      <span
                        style={{
                          padding: "2px 6px",
                          borderRadius: "4px",
                          fontWeight: 700,
                          whiteSpace: "nowrap",
                          display: "inline-block",
                          fontSize: badgeSize,
                          color:
                            GRADE_COLORS[row.gradeLabel]?.text || "#52525b",
                          background:
                            GRADE_COLORS[row.gradeLabel]?.bg || "#f4f4f5",
                        }}
                      >
                        {row.gradeLabel}
                      </span>
                    ) : (
                      <span style={{ color: "#d4d4d8" }}>-</span>
                    )}
                  </td>
                )}
              </tr>
            ))}
          </tbody>

          <tfoot>
            <tr
              style={{
                borderTop: "2px solid #18181b",
                background: "#18181b",
                color: "#ffffff",
                fontWeight: 700,
              }}
            >
              <td
                style={{
                  padding: "8px 10px",
                  textAlign: "right",
                  fontSize: footerLabelSize,
                }}
              >
                المجموع الكلي
              </td>
              <td
                style={{
                  padding: "6px 10px",
                  textAlign: "center",
                  fontSize: footerValueSize,
                  letterSpacing: "-0.02em",
                }}
                colSpan={columns.length + 1 + (hasGradeLabel ? 1 : 0)}
              >
                {totalFinalScore}
                {totalMaxScore > 0 && (
                  <span
                    style={{
                      color: "#a1a1aa",
                      marginRight: "8px",
                      fontSize: headerSize,
                    }}
                  >
                    / {totalMaxScore}
                  </span>
                )}
              </td>
            </tr>
          </tfoot>
        </table>
      </div>
    </div>
  );
}
