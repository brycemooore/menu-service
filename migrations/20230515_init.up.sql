-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "menu_category" (
  "id" serial PRIMARY KEY,
  "name" varchar NOT NULL,
  "menu_id" integer NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT NOW(),
  "updated_at" timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "menu" (
  "id" serial PRIMARY KEY,
  "name" varchar NOT NULL,
  "is_active" bool NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT NOW(),
  "updated_at" timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "menu_category_item" (
  "item_id" integer NOT NULL,
  "menu_category_id" integer NOT NULL
);

CREATE TABLE IF NOT EXISTS "item" (
  "id" serial PRIMARY KEY,
  "name" varchar NOT NULL,
  "description" text NOT NULL,
  "available" bool NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT NOW(),
  "updated_at" timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "item_ingredient" (
  "id" serial PRIMARY KEY,
  "name" varchar NOT NULL,
  "item_id" integer NOT NULL,
  "available" bool NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT NOW(),
  "updated_at" timestamp NOT NULL DEFAULT NOW()
);

ALTER TABLE "menu_category" ADD FOREIGN KEY ("menu_id") REFERENCES "menu" ("id");

ALTER TABLE "item_ingredient" ADD FOREIGN KEY ("item_id") REFERENCES "item" ("id");

ALTER TABLE "menu_category_item" ADD FOREIGN KEY ("item_id") REFERENCES "item" ("id");

ALTER TABLE "menu_category_item" ADD FOREIGN KEY ("menu_category_id") REFERENCES "menu_category" ("id");