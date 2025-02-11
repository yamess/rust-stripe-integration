use std::sync::Arc;

pub struct PaymentService<C> {
    client: Arc<C>,
}
