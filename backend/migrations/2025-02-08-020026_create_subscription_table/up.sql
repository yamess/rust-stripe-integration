-- Your SQL goes here



CREATE TABLE "subscriptions"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"user_id" UUID NOT NULL,
	"plan_id" INT4 NOT NULL,
	"stripe_subscription_id" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"current_period_start" TIMESTAMPTZ NOT NULL,
	"current_period_end" TIMESTAMPTZ,
	"canceled_at" TIMESTAMPTZ,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ,
	FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE,
	FOREIGN KEY ("plan_id") REFERENCES "plans"("id")
);

