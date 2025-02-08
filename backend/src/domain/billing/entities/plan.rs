use chrono::{DateTime, Utc};
use crate::domain::billing::value_objects::billing_cycle::BillingCycle;
use crate::domain::billing::value_objects::currency::Currency;
use crate::domain::billing::value_objects::price::Price;

#[derive(Debug, Clone)]
pub struct Plan {
    id: i32,
    name: String,
    description: Option<String>,
    price: Price,
    currency: Currency,
    billing_cycle: BillingCycle,
    stripe_price_id: String,
    stripe_product_id: String,
    trial_period_days: i32,
    active: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}
impl Plan {
    pub fn new(
        name: String,
        description: Option<String>,
        price: Price,
        currency: Currency,
        billing_cycle: BillingCycle,
        stripe_price_id: String,
        stripe_product_id: String,
        trial_period_days: i32,
        active: bool,
    ) -> Self {
        Self {
            id: 0,
            name,
            description,
            price,
            currency,
            billing_cycle,
            stripe_price_id,
            stripe_product_id,
            trial_period_days,
            active,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn price(&self) -> &Price {
        &self.price
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn billing_cycle(&self) -> &BillingCycle {
        &self.billing_cycle
    }

    pub fn stripe_price_id(&self) -> &str {
        &self.stripe_price_id
    }

    pub fn stripe_product_id(&self) -> &str {
        &self.stripe_product_id
    }

    pub fn trial_period_days(&self) -> i32 {
        self.trial_period_days
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at
    }

    pub fn update(
        &mut self,
        name: String,
        description: Option<String>,
        price: Price,
        currency: Currency,
        billing_cycle: BillingCycle,
        stripe_price_id: String,
        stripe_product_id: String,
        trial_period_days: i32,
        active: bool,
    ) {
        self.name = name;
        self.description = description;
        self.price = price;
        self.currency = currency;
        self.billing_cycle = billing_cycle;
        self.stripe_price_id = stripe_price_id;
        self.stripe_product_id = stripe_product_id;
        self.trial_period_days = trial_period_days;
        self.active = active;
        self.updated_at = Some(Utc::now());
    }

    pub fn construct(
        id: i32,
        name: String,
        description: Option<String>,
        price: Price,
        currency: Currency,
        billing_cycle: BillingCycle,
        stripe_price_id: String,
        stripe_product_id: String,
        trial_period_days: i32,
        active: bool,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            price,
            currency,
            billing_cycle,
            stripe_price_id,
            stripe_product_id,
            trial_period_days,
            active,
            created_at,
            updated_at,
        }
    }
}
