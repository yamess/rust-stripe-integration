-- Your SQL goes here


CREATE TABLE "plans"(
	"id" INT4 NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
	"name" VARCHAR NOT NULL,
	"description" VARCHAR,
	"stripe_product_id" VARCHAR NOT NULL
);

