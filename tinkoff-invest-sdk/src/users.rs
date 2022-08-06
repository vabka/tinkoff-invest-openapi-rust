use crate::{method, service};
use tinkoff_invest_grpc::api::{users_service_client::UsersServiceClient, GetUserTariffRequest};
use tinkoff_invest_grpc::api::{GetAccountsRequest, GetInfoRequest};
pub use tinkoff_invest_grpc::api::{
    GetAccountsResponse, GetInfoResponse, GetMarginAttributesRequest, GetMarginAttributesResponse,
    GetUserTariffResponse,
};
use tinkoff_invest_grpc::*;

service!(UsersClient, UsersServiceClient<Inner>, {
    // method!(get_accounts, GetAccountsRequest, GetAccountsResponse);
    // method!(get_user_tariff, GetUserTariffRequest, GetUserTariffResponse);
    // method!(get_info, GetInfoRequest, GetInfoResponse);
    // method!(
    //     get_margin_attributes,
    //     GetMarginAttributesRequest,
    //     GetMarginAttributesResponse,
    //     thin
    // );
});
