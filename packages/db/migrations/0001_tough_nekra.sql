ALTER TYPE "public"."student_status" ADD VALUE 'auditor';--> statement-breakpoint
ALTER TYPE "public"."student_status" ADD VALUE 'homeschooled';--> statement-breakpoint
ALTER TABLE "students" ADD COLUMN "notes" text;--> statement-breakpoint
ALTER TABLE "students" ADD COLUMN "metadata" jsonb DEFAULT '{}';--> statement-breakpoint
ALTER TABLE "subjects" ADD COLUMN "sort_order" integer DEFAULT 0 NOT NULL;--> statement-breakpoint
CREATE INDEX "student_metadata_gin_idx" ON "students" USING gin ("metadata");--> statement-breakpoint
CREATE INDEX "subjects_sort_order_idx" ON "subjects" USING btree ("sort_order");