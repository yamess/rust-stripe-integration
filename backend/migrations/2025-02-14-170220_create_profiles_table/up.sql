-- Your SQL goes here

CREATE TABLE "profiles"(
	"id" INT4 NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	"user_id" UUID NOT NULL UNIQUE,
	"first_name" VARCHAR,
	"last_name" VARCHAR,
	"phone" VARCHAR,
	"photo_url" VARCHAR,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ,
	FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);
CREATE INDEX "profiles_user_id_index" ON "profiles"("user_id");

