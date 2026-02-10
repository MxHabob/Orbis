ALTER TABLE "users" ADD COLUMN "timezone" varchar(255) DEFAULT '';--> statement-breakpoint
ALTER TABLE "users" ADD COLUMN "time_format" integer DEFAULT 24;--> statement-breakpoint
ALTER TABLE "users" ADD COLUMN "week_starts_on_monday" boolean DEFAULT false;