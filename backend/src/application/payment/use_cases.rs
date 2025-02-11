use std::sync::Arc;

pub struct CreateCustomerUseCase<C> {
    service: PaymentService<C>,
}