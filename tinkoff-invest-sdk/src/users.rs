use crate::{service, types};
use tinkoff_invest_grpc::api::users_service_client::UsersServiceClient;

pub use tinkoff_invest_grpc::api::{
    GetAccountsResponse, GetInfoResponse, GetMarginAttributesRequest, GetMarginAttributesResponse,
    GetUserTariffResponse,
};
use tinkoff_invest_grpc::*;

service!(UsersClient, UsersServiceClient<Inner>, {
    // method!(get_user_tariff, GetUserTariffRequest, GetUserTariffResponse);
    // method!(get_info, GetInfoRequest, GetInfoResponse);
    // method!(
    //     get_margin_attributes,
    //     GetMarginAttributesRequest,
    //     GetMarginAttributesResponse,
    //     thin
    // );
});

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
}
