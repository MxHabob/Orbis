-- Orbis schema cleanup v3
ALTER TABLE classes DROP COLUMN schedule_template;
ALTER TABLE scheduled_lessons ADD COLUMN room_name TEXT;
