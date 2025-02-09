//use std::collections::HashMap;
//use serde::Serialize;
//use stripe::{CheckoutSession, CheckoutSessionMode, Client, CreateCheckoutSession, CreateCheckoutSessionLineItems, CreatePrice, CreatePriceRecurring, CreatePriceRecurringInterval, CreateProduct, Currency, IdOrCreate, Price, Product};
//
//#[derive(Debug, Clone, Serialize)]
//pub struct PaymentSessionResponse {
//    pub url: String
//}
//
//#[derive(Clone)]
//pub struct Payment {
//    client: Client,
//    success_page: Option<String>,
//    failure_page: Option<String>,
//}
//impl Payment {
//    pub fn new(secret_key: &str, success_page: Option<String>, failure_page: Option<String>) ->
//                                                                                            Self {
//        let client = Client::new(secret_key);
//        Self {
//            client,
//            success_page,
//            failure_page
//        }
//    }
//
//    pub async fn create_stripe_session(&self) -> String {
//        let product = {
//            let mut create_product = CreateProduct::new("Premium Plan");
//            create_product.metadata = Some(HashMap::from([(
//                "description".to_string(),
//                "Premium Plan".to_string()
//                )]));
//            Product::create(&self.client, create_product).await.unwrap()
//        };
//        let mut create_price = CreatePrice::new(Currency::CAD);
//        create_price.product = Some(IdOrCreate::Id(&product.id));
//        create_price.unit_amount = Some(2999);
//        create_price.recurring = Some(CreatePriceRecurring {
//            interval: CreatePriceRecurringInterval::Day,
//            ..Default::default()
//        });
//        create_price.expand = &["product"];
//
//        let price = Price::create(&self.client, create_price).await.unwrap();
//
//        let mut session = CreateCheckoutSession::new();
//        session.cancel_url = self.failure_page.as_deref();
//        session.success_url = self.success_page.as_deref();
//        session.customer = None;
//        session.mode = Some(CheckoutSessionMode::Subscription);
//        session.line_items = Some(vec![CreateCheckoutSessionLineItems{
//            quantity: Some(1),
//            price: Some(price.id.to_string()),
//            ..Default::default()
//        }]);
//        session.expand = &["line_items", "line_items.data.price.product"];
//
//        let checkout_session = CheckoutSession::create(
//            &self.client,
//            session
//        ).await.unwrap();
//
//        checkout_session.url.unwrap()
//    }
//}