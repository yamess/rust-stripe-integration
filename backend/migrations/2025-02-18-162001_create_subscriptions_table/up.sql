-- Your SQL goes here


CREATE TABLE "subscriptions"(
	"id" INT4 NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	"user_id" UUID NOT NULL UNIQUE ,
	"stripe_customer_id" VARCHAR NOT NULL UNIQUE,
	"stripe_price_id" VARCHAR NOT NULL,
	"stripe_product_id" VARCHAR NOT NULL,
	"stripe_subscription_id" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"has_used_trial" BOOL NOT NULL,
	"current_period_end" TIMESTAMPTZ,
	"cancel_at_period_end" BOOL NOT NULL,
	"canceled_at" TIMESTAMPTZ,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ,
	FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);

