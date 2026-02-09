import { formatAmount } from "@/utils/format";
import {
  Area,
  CartesianGrid,
  ComposedChart,
  Line,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import {
  commonChartConfig,
  createCompactTickFormatter,
  useChartMargin,
} from "./chart-utils";

export interface SchoolMetricData {
  month: string;
  value: number;
  average: number;
}

interface SchoolMetricChartProps {
  data: SchoolMetricData[];
  height?: number;
  currency?: string;
  locale?: string;
  valueFormatter?: (value: number) => string;
}

const CustomTooltip = ({ active, payload, label, valueFormatter, currency = "USD", locale }: any) => {
  if (active && payload && payload.length) {
    const value = payload[0].value;
    const average = payload[1]?.value;

    const formatValue = (val: number) => 
      valueFormatter ? valueFormatter(val) : 
      formatAmount({
        amount: val,
        currency,
        locale: locale ?? undefined,
        maximumFractionDigits: 0,
      }) ?? val.toLocaleString();

    return (
      <div className="border p-2 text-[10px] font-hedvig-sans bg-white dark:bg-[#0c0c0c] border-[#e6e6e6] dark:border-[#1d1d1d] text-black dark:text-white shadow-sm">
        <p className="mb-1 text-[#707070] dark:text-[#666666]">{label}</p>
        <p className="text-black dark:text-white">
          Current: {formatValue(value)}
        </p>
        {average !== undefined && (
          <p className="text-black dark:text-white">
            Average: {formatValue(average)}
          </p>
        )}
      </div>
    );
  }
  return null;
};

export function SchoolMetricChart({
  data,
  height = 320,
  currency = "USD",
  locale,
  valueFormatter,
}: SchoolMetricChartProps) {
  const tickFormatter = createCompactTickFormatter();
  const { marginLeft } = useChartMargin(data, "value", tickFormatter);

  return (
    <div className="w-full">
      <div style={{ height }}>
        <ResponsiveContainer width="100%" height="100%">
          <ComposedChart
            data={data}
            margin={{ top: 6, right: 6, left: -marginLeft, bottom: 6 }}
          >
            <defs>
              <pattern
                id="schoolMetricPattern"
                x="0"
                y="0"
                width="8"
                height="8"
                patternUnits="userSpaceOnUse"
              >
                <rect width="8" height="8" fill="var(--chart-pattern-bg)" />
                <path
                  d="M0,0 L8,8 M-2,6 L6,16 M-4,4 L4,12"
                  stroke="var(--chart-pattern-stroke)"
                  strokeWidth="0.8"
                  opacity="0.6"
                />
              </pattern>
              <linearGradient id="schoolMetricGradient" x1="0" y1="0" x2="1" y2="0">
                <stop offset="0%" stopColor="var(--chart-gradient-start)" />
                <stop offset="100%" stopColor="var(--chart-gradient-end)" />
              </linearGradient>
            </defs>
            <CartesianGrid
              strokeDasharray="3 3"
              stroke="var(--chart-grid-stroke)"
            />
            <XAxis
              dataKey="month"
              axisLine={false}
              tickLine={false}
              tick={{
                fill: "var(--chart-axis-text)",
                fontSize: 10,
                fontFamily: commonChartConfig.fontFamily,
              }}
            />
            <YAxis
              axisLine={false}
              tickLine={false}
              tick={{
                fill: "var(--chart-axis-text)",
                fontSize: 10,
                fontFamily: commonChartConfig.fontFamily,
              }}
              tickFormatter={tickFormatter}
              dataKey="value"
            />
            <Tooltip
              content={<CustomTooltip currency={currency} locale={locale} valueFormatter={valueFormatter} />}
              wrapperStyle={{ zIndex: 9999 }}
            />
            <Area
              type="monotone"
              dataKey="value"
              stroke="url(#schoolMetricGradient)"
              strokeWidth={2}
              fill="url(#schoolMetricPattern)"
              dot={{
                fill: "var(--chart-actual-line)",
                strokeWidth: 0,
                r: 3,
              }}
              activeDot={{
                r: 5,
                fill: "var(--chart-actual-line)",
                stroke: "var(--chart-actual-line)",
                strokeWidth: 2,
              }}
              isAnimationActive={false}
            />
            <Line
              type="monotone"
              dataKey="average"
              stroke="var(--chart-axis-text)"
              strokeWidth={1}
              strokeDasharray="5 5"
              dot={false}
              isAnimationActive={false}
            />
          </ComposedChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

