CREATE TYPE "public"."document_type" AS ENUM('file', 'folder');--> statement-breakpoint
CREATE TABLE "documents" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"name" varchar(255) NOT NULL,
	"title" varchar(255),
	"size" integer,
	"type" "document_type" DEFAULT 'file' NOT NULL,
	"mimetype" varchar(100),
	"extension" varchar(20),
	"key" text,
	"path_tokens" jsonb,
	"user_id" uuid NOT NULL,
	"parent_folder_id" uuid,
	"metadata" jsonb,
	"processing_status" varchar(20) DEFAULT 'completed',
	"is_pinned" boolean DEFAULT false NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL,
	"deleted_at" timestamp
);
--> statement-breakpoint
ALTER TABLE "certificate_templates" ADD COLUMN "content" jsonb;--> statement-breakpoint
ALTER TABLE "documents" ADD CONSTRAINT "documents_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "documents" ADD CONSTRAINT "documents_parent_folder_id_documents_id_fk" FOREIGN KEY ("parent_folder_id") REFERENCES "public"."documents"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "user_document_idx" ON "documents" USING btree ("user_id","created_at" DESC NULLS LAST);--> statement-breakpoint
ALTER TABLE "certificate_templates" DROP COLUMN "header_content";--> statement-breakpoint
ALTER TABLE "certificate_templates" DROP COLUMN "main_content";--> statement-breakpoint
ALTER TABLE "certificate_templates" DROP COLUMN "footer_content";--> statement-breakpoint
ALTER TABLE "certificate_templates" DROP COLUMN "table_config";