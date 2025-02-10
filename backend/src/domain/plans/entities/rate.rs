use crate::domain::plans::value_objects::billing_cycle::BillingCycle;
use crate::domain::plans::value_objects::currency::Currency;
use crate::domain::plans::value_objects::price::Price;

#[derive(Debug, Clone)]
pub struct Rate {
    id: i32,
    stripe_price_id: String,
    plan_id: i32,
    currency: Currency,
    amount: Price,
    billing_cycle: BillingCycle,
    active: bool,
}

impl Rate {
    pub fn new(stripe_price_id: String, plan_id: i32, currency: Currency, amount: Price,
               billing_cycle: BillingCycle) -> Self {
        Self {
            id: Default::default(),
            stripe_price_id,
            plan_id,
            currency,
            amount,
            billing_cycle,
            active: true
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn stripe_price_id(&self) -> &str {
        &self.stripe_price_id
    }

    pub fn plan_id(&self) -> i32 {
        self.plan_id
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn amount(&self) -> &Price {
        &self.amount
    }

    pub fn billing_cycle(&self) -> &BillingCycle {
        &self.billing_cycle
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn update(&mut self, stripe_price_id: Option<String>, plan_id: Option<i32>, currency:
    Option<Currency>, amount: Option<Price>, billing_cycle: Option<BillingCycle>, active: Option<bool>) {
        self.stripe_price_id = stripe_price_id.unwrap_or_else(|| self.stripe_price_id.clone());
        self.plan_id = plan_id.unwrap_or_else(|| self.plan_id);
        self.currency = currency.unwrap_or_else(|| self.currency.clone());
        self.amount = amount.unwrap_or_else(|| self.amount.clone());
        self.billing_cycle = billing_cycle.unwrap_or_else(|| self.billing_cycle.clone());
        self.active = active.unwrap_or_else(|| self.active);
    }

    pub fn construct(id: i32, stripe_price_id: String, plan_id: i32, currency: Currency, amount:
    Price, billing_cycle: BillingCycle, active: bool) -> Self {
        Self {
            id,
            stripe_price_id,
            plan_id,
            currency,
            amount,
            billing_cycle,
            active
        }
    }
}