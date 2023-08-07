use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::checkout::Checkout;
use crate::models::store::StoreInvoice;
use crate::repositories::checkout_repository::CheckoutRepository;
use crate::repositories::store_repository::{
    CreateStoreInvoice, StoreInvoiceRepository, StoreRepository,
};
use super::checkout_service::{CheckoutService, CreateCheckoutService};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStoreInvoiceResponse {
    pub invoice: StoreInvoice,
    pub checkout: Checkout,
    pub qr_unified: String,
    pub qr_bitcoin: String,
    pub qr_ln: String,
}

pub struct StoreService {
    pub store_repo: StoreRepository,
    pub store_invoice_repo: StoreInvoiceRepository,
    pub checkout_repo: CheckoutRepository,
}

impl StoreService {
    pub fn new(
        store_repo: StoreRepository,
        checkout_repo: CheckoutRepository,
        store_invoice_repo: StoreInvoiceRepository,
    ) -> Self {
        Self {
            store_repo,
            store_invoice_repo,
            checkout_repo,
        }
    }

    pub async fn create_invoice(
        &self,
        store_uuid: &str,
        metadata: Option<serde_json::Value>,
        checkout_data: CreateCheckoutService,
        checkout_service: CheckoutService,
    ) -> Result<CreateStoreInvoiceResponse> {
        let checkout = checkout_service
            .create(checkout_data, self.checkout_repo.clone())
            .await?;

        let create_invoice = CreateStoreInvoice {
            checkout_uuid: checkout.checkout.clone().uuid,
            store_uuid: store_uuid.to_string(),
            metadata,
        };

        let invoice = self.store_invoice_repo.create(create_invoice).await?;

        let response = CreateStoreInvoiceResponse {
            invoice: invoice.clone(),
            checkout: checkout.checkout,
            qr_unified: checkout.qr_unified,
            qr_bitcoin: checkout.qr_bitcoin,
            qr_ln: checkout.qr_ln,
        };

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::{helpers::tests::{create_test_pool, create_test_user, create_test_cluster}, repositories::{store_repository::{StoreRepository, StoreInvoiceRepository}, checkout_repository::CheckoutRepository}, services::checkout_service::{CreateCheckoutService, CheckoutService}};
    use crate::services::store_service::StoreService;

    #[tokio::test]
    async fn create_store_invoice() {
        let pool = create_test_pool().await;
        let user = create_test_user().await.unwrap();
        let cluster = create_test_cluster().await;
        let store_repo: StoreRepository = StoreRepository::new(pool.clone());
        let store = store_repo.create(&user.uuid, "Test Store").await.unwrap();
        let metadata = Some ({
            serde_json::json!({"test": "test"})
        });

        let checkout_service = CheckoutService::new(cluster);

        let store_service = StoreService::new(
            StoreRepository::new(pool.clone()),
            CheckoutRepository::new(pool.clone()),
            StoreInvoiceRepository::new(pool.clone()),
        );

        let checkout_data = CreateCheckoutService {
            user_uuid: user.uuid,
            amount: 1000,
            expiry: 3600,
            memo: None,
        };

        let invoice = store_service.create_invoice(&store.uuid, metadata, checkout_data, checkout_service).await.unwrap();

        assert!(&invoice.invoice.uuid.len() > &0);
        assert!(&invoice.checkout.bitcoin_address.len() > &0);
        assert!(&invoice.checkout.payment_request.len() > &0);

    }
}
