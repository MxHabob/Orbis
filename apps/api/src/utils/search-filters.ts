import { z } from "zod";

const schema = z.object({
  searchTerm: z.string().describe("The query to search for"),
  startDate: z
    .string()
    .optional()
    .describe("The start date when to retrieve from. Return ISO-8601 format."),
  endDate: z
    .string()
    .optional()
    .describe(
      "The end date when to retrieve data from. If not provided, defaults to the current date. Return ISO-8601 format.",
    ),
  types: z
    .array(
      z.enum([
        "transactions",
        "invoices",
        "tracker_projects",
        "customers",
        "documents",
      ]),
    )
    .describe("The type of the items to search for"),
  amount: z
    .number()
    .optional()
    .describe(
      "The exact amount to search for if the type is transactions or invoices.",
    ),
  amountMin: z
    .number()
    .optional()
    .describe("Minimum amount filter for transactions or invoices."),
  amountMax: z
    .number()
    .optional()
    .describe("Maximum amount filter for transactions or invoices."),
  status: z
    .enum(["paid", "unpaid", "overdue", "draft"])
    .optional()
    .describe(
      "The status filter (e.g. 'paid', 'unpaid', 'overdue', 'draft') for invoices or projects.",
    ),
  currency: z
    .string()
    .optional()
    .describe("The currency code to filter by (e.g., 'USD', 'EUR')."),
  language: z
    .string()
    .describe(
      "The language to search in based on the query. Return in PostgreSQL text search configuration name (e.g., 'english', 'swedish', 'german', 'french').",
    ),
  dueDateStart: z
    .string()
    .optional()
    .describe("Start date for invoice due dates (ISO-8601)."),
  dueDateEnd: z
    .string()
    .optional()
    .describe("End date for invoice due dates (ISO-8601)."),
});
