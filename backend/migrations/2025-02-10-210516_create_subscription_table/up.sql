-- Your SQL goes here




CREATE TABLE "subscriptions"(
	"id" INT4 NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	"user_id" UUID NOT NULL,
	"rate_id" INT4 NOT NULL,
	"stripe_subscription_id" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"current_period_end" TIMESTAMPTZ,
	"canceled_at" TIMESTAMPTZ,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
	"updated_at" TIMESTAMPTZ,
	FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE,
	FOREIGN KEY ("rate_id") REFERENCES "rates"("id")
);

