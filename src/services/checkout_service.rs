use crate::{
    models::checkout::Checkout,
    repositories::checkout_repository::{CheckoutRepository, CreateCheckout},
};
use std::fs;
use anyhow::Result;
use lightning_cluster::{
    cluster::{Cluster, ClusterAddInvoice},
    lnd::AddInvoiceResponse,
};
use tokio::try_join;

pub struct CheckoutService {
    pub cluster: Cluster,
}

#[derive(Debug, Clone)]
pub struct CreateCheckoutService {
    pub user_uuid: String,
    pub amount: i64,
    pub expiry: i64,
    pub memo: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CheckoutResponse {
    pub checkout: Checkout,
    pub qr_unified: String,
    pub qr_bitcoin: String,
    pub qr_ln: String,
}

impl CheckoutService {
    pub fn new(cluster: Cluster) -> Self {
        Self { cluster }
    }

    pub async fn create(
        &self,
        data: CreateCheckoutService,
        repo: CheckoutRepository,
    ) -> Result<CheckoutResponse> {
        let (ln_pr, bitcoin_addr) =
            try_join!(self.get_ln_pr(data.clone()), self.get_bitcoin_addr())?;

        let unified = format! {"bitcoin:{}?lightning={}", bitcoin_addr, ln_pr.payment_request};

        let (unified_qr, bitcoin_qr, ln_qr) = try_join!(
            Self::get_qr(unified.as_str()),
            Self::get_qr(bitcoin_addr.as_str()),
            Self::get_qr(ln_pr.payment_request.as_str())
        )?;

        let create_checkout = CreateCheckout {
            user_uuid: data.user_uuid,
            amount: data.amount,
            bitcoin_address: bitcoin_addr,
            payment_request: ln_pr.payment_request,
            expiry_seconds: data.expiry,
        };

        let checkout = repo.create(create_checkout).await?;

        let response = CheckoutResponse {
            checkout: checkout,
            qr_unified: unified_qr,
            qr_bitcoin: bitcoin_qr,
            qr_ln: ln_qr,
        };

        Ok(response)
    }

    async fn get_ln_pr(&self, data: CreateCheckoutService) -> Result<AddInvoiceResponse> {
        let mut memo = String::from("");
        match data.memo {
            Some(m) => memo = m,
            None => memo = dotenvy::var("APP_NAME").unwrap().to_string(),
        };

        let pr_req = ClusterAddInvoice {
            pubkey: None,
            value: data.amount,
            expiry: data.expiry,
            memo: memo,
        };

        let lightning_request = self.cluster.add_invoice(pr_req, None).await?;

        Ok(lightning_request)
    }

    async fn get_bitcoin_addr(&self) -> Result<String, anyhow::Error> {
        let addr = self.cluster.next_address(None).await?;

        Ok(addr)
    }

    async fn get_qr(url: &str) -> Result<String> {
        let qr_name = format!("/tmp/{}.png", &url[..6]);
        qrcode_gen::qr_image(url, qr_name.as_str());
        let buffer = fs::read(qr_name.as_str())?;
        let base64_image = base64::encode(&buffer);
        fs::remove_file(qr_name)?;
        Ok(format!("data:image/png;base64,{}", base64_image))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        helpers::tests::{create_test_cluster, create_test_pool, create_test_user},
        repositories::
            checkout_repository::CheckoutRepository,
        services::checkout_service::{CheckoutService, CreateCheckoutService},
    };

    #[tokio::test]
    pub async fn test_create_checkout_service() {
        let cluster = create_test_cluster().await;
        let pool = create_test_pool().await;
        let user = create_test_user().await.unwrap();

        let service = CheckoutService::new(cluster);
        let data = CreateCheckoutService {
            user_uuid: user.uuid,
            amount: 1000,
            expiry: 3600,
            memo: None,
        };
        let repo = CheckoutRepository::new(pool);
        let response = service.create(data, repo).await.unwrap();
        
        assert_eq!(response.checkout.amount, 1000);
        assert_eq!(response.checkout.expiry_seconds, 3600);
        assert!(response.checkout.bitcoin_address.len() > 0);
        assert!(response.checkout.payment_request.len() > 0);
    }
}
