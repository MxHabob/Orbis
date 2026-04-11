ALTER TABLE "students" ADD COLUMN "seating_number" varchar(20);--> statement-breakpoint
CREATE INDEX "seating_num_idx" ON "students" USING btree ("seating_number");