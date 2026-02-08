CREATE TABLE "certificate_templates" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"name" varchar(100) NOT NULL,
	"is_default" boolean DEFAULT false NOT NULL,
	"orientation" varchar(20) DEFAULT 'portrait' NOT NULL,
	"size" varchar(20) DEFAULT 'a4' NOT NULL,
	"primary_color" varchar(20) DEFAULT '#000000',
	"background_image_url" text,
	"header_content" jsonb,
	"footer_content" jsonb,
	"table_config" jsonb,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL
);
