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
        "students",
        "classes",
        "staff",
        "subjects",
        "documents",
        "tracker_projects",
      ]),
    )
    .describe("The type of the items to search for"),
  status: z
    .enum(["active", "graduated", "transferred", "suspended"])
    .optional()
    .describe(
      "The status filter (e.g. 'active', 'graduated') for students or other entities.",
    ),
  language: z
    .string()
    .default("english")
    .describe(
      "The language to search in based on the query. Return in PostgreSQL text search configuration name (e.g., 'english', 'arabic').",
    ),
});
