-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	"email" VARCHAR NOT NULL UNIQUE,
	"firebase_id" VARCHAR NOT NULL UNIQUE,
	"stripe_customer_id" VARCHAR NOT NULL UNIQUE,
	"status" VARCHAR NOT NULL DEFAULT 'active' check ( status IN ('active', 'inactive', 'banned', 'suspended', 'pending')),
	"role" VARCHAR NOT NULL DEFAULT 'user' CHECK( role IN ('user', 'admin', 'guest', 'super')),
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ
);
