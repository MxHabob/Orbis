-- Orbis schema cleanup v3 — post-Timefold legacy column removal.
-- Safe to re-run: failures are ignored when columns are already absent.

ALTER TABLE classes DROP COLUMN schedule_template;
ALTER TABLE scheduled_lessons DROP COLUMN room_name;
