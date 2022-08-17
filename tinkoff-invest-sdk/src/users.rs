use crate::{service, types};
use tinkoff_invest_grpc::api::users_service_client::UsersServiceClient;

pub use tinkoff_invest_grpc::api::{
    GetAccountsResponse, GetInfoResponse, GetMarginAttributesRequest, GetMarginAttributesResponse,
    GetUserTariffResponse,
};
use tinkoff_invest_grpc::*;

service!(UsersClient, UsersServiceClient<Inner>);

impl UsersClient {
    /// Получить все счета пользователя
    pub async fn get_accounts(&mut self) -> crate::Result<Vec<types::Account>> {
        let request = api::GetAccountsRequest {};
        let response = self.internal.get_accounts(request).await?;
        let data = response.into_inner();
        Ok(data
            .accounts
            .into_iter()
            .map(types::Account::from)
            .collect())
    }

    pub async fn get_user_tariff(&mut self) -> crate::Result<types::UserTariff> {
        let request = api::GetUserTariffRequest {};
        let response = self.internal.get_user_tariff(request).await?;
        let tariff = response.into_inner();
        Ok(tariff.into())
    }

    pub async fn get_info(&mut self) -> crate::Result<types::Info> {
        let request = api::GetInfoRequest {};
        let response = self.internal.get_info(request).await?;
        let data = response.into_inner();
        Ok(types::Info::from(data))
    }

    pub async fn get_margin_attributes(
        &mut self,
        account_id: impl Into<String>,
    ) -> crate::Result<types::MarginAttributes> {
        let request = api::GetMarginAttributesRequest {
            account_id: account_id.into(),
        };
        let response = self.internal.get_margin_attributes(request).await?;
        let data = response.into_inner();
        Ok(data.into())
    }
}
