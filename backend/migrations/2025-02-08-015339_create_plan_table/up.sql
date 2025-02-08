-- Your SQL goes here


CREATE TABLE "plans"(
	"id" INT4 NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	"name" VARCHAR NOT NULL,
	"description" VARCHAR,
	"price" NUMERIC NOT NULL CHECK (price >= 0),
	"currency" VARCHAR NOT NULL,
	"billing_cycle" VARCHAR NOT NULL,
	"stripe_price_id" VARCHAR NOT NULL,
	"stripe_product_id" VARCHAR NOT NULL,
	"trial_period_days" INT4 NOT NULL,
	"active" BOOL NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ
);

