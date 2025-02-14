-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	"email" VARCHAR NOT NULL UNIQUE,
	"firebase_id" VARCHAR NOT NULL UNIQUE,
	"stripe_customer_id" VARCHAR,
	"status" VARCHAR NOT NULL,
	"role" VARCHAR NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ
);

CREATE INDEX "users_email_index" ON "users"("email");
CREATE INDEX "users_firebase_id_index" ON "users"("firebase_id");
CREATE INDEX "users_stripe_customer_id_index" ON "users"("stripe_customer_id");
