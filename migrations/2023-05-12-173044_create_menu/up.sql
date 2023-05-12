-- Your SQL goes here
CREATE TABLE "menu_category" (
  "id" integer PRIMARY KEY,
  "name" varchar,
  "menu_id" integer,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "menu" (
  "id" integer PRIMARY KEY,
  "name" varchar,
  "is_active" bool,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "menu_category_item" (
  "id" integer PRIMARY KEY,
  "item_id" integer,
  "menu_category_id" integer
);

CREATE TABLE "item" (
  "id" integer PRIMARY KEY,
  "name" varchar,
  "description" text,
  "available" bool,
  "created_at" timestamp,
  "updated_at" timestamp
);

CREATE TABLE "item_ingredient" (
  "id" integer PRIMARY KEY,
  "name" varchar,
  "item_id" integer,
  "available" bool,
  "created_at" timestamp,
  "updated_at" timestamp
);

ALTER TABLE "menu_category" ADD FOREIGN KEY ("menu_id") REFERENCES "menu" ("id");

ALTER TABLE "item_ingredient" ADD FOREIGN KEY ("item_id") REFERENCES "item" ("id");

ALTER TABLE "menu_category_item" ADD FOREIGN KEY ("item_id") REFERENCES "item" ("id");

ALTER TABLE "menu_category_item" ADD FOREIGN KEY ("menu_category_id") REFERENCES "menu_category" ("id");