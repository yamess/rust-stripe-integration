-- Your SQL goes here



CREATE TABLE "subscriptions"(
	"id" INT4 NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	"user_id" UUID NOT NULL,
	"plan_id" INT4 NOT NULL,
	"stripe_subscription_id" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL check ( status in ('active', 'trialing', 'past_due', 'canceled', 'unpaid') ),
	"current_period_end" TIMESTAMPTZ,
	"canceled_at" TIMESTAMPTZ,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
	"updated_at" TIMESTAMPTZ,
	FOREIGN KEY ("user_id") REFERENCES "users"("id"),
	FOREIGN KEY ("plan_id") REFERENCES "plans"("id")
);

