-- Your SQL goes here


CREATE TABLE "subscriptions"(
	"id" INT4 NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	"user_id" UUID NOT NULL,
	"stripe_customer_id" VARCHAR NOT NULL,
	"stripe_price_id" VARCHAR NOT NULL,
	"stripe_subscription_id" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"current_period_end" TIMESTAMPTZ,
	"canceled_at" TIMESTAMPTZ,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
	"updated_at" TIMESTAMPTZ,
	FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);
CREATE INDEX "subscriptions_user_id_index" ON "subscriptions"("user_id");
CREATE INDEX "subscriptions_stripe_customer_id_index" ON "subscriptions"("stripe_customer_id");
CREATE INDEX "subscriptions_stripe_subscription_id_index" ON "subscriptions"("stripe_subscription_id");
