-- Your SQL goes here



CREATE TABLE "rates"(
	"id" INT4 NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	"stripe_price_id" VARCHAR NOT NULL UNIQUE,
	"plan_id" INT4 NOT NULL,
	"currency" VARCHAR NOT NULL,
	"amount" NUMERIC NOT NULL,
	"billing_cycle" VARCHAR NOT NULL,
	"active" BOOL NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
	"updated_at" TIMESTAMPTZ,
	FOREIGN KEY ("plan_id") REFERENCES "plans"("id") ON DELETE CASCADE
);

