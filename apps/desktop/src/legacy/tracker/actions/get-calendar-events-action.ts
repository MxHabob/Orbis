export type CalendarEvent = {
  id: string;
  date: string;
  start?: string | null;
  duration?: number | null;
  description?: string | null;
  assigned?: { full_name?: string | null } | null;
};

export async function getCalendarEventsAction(_input: Record<string, unknown>) {
  return { events: [] as CalendarEvent[] };
}
