use tinkoff_invest_grpc::api::instruments_service_client::InstrumentsServiceClient;
use tinkoff_invest_grpc::api::{self, InstrumentIdType};
pub use tinkoff_invest_grpc::api::{
    BondResponse, BondsResponse, TradingSchedulesRequest, TradingSchedulesResponse,
};
use tinkoff_invest_grpc::tonic;
use tinkoff_invest_grpc::tonic::IntoRequest;
use tinkoff_invest_grpc::Inner;

use crate::{method, service};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentRequest {
    Figi(String),
    Ticker { id: String, class_code: String },
    Uid(String),
}

pub struct InstrumentsRequest {}

impl IntoRequest<api::InstrumentRequest> for InstrumentRequest {
    fn into_request(self) -> tonic::Request<api::InstrumentRequest> {
        let req = match self {
            InstrumentRequest::Figi(figi) => api::InstrumentRequest {
                class_code: "".to_string(),
                id_type: InstrumentIdType::Figi as i32,
                id: figi,
            },
            InstrumentRequest::Ticker { id, class_code } => api::InstrumentRequest {
                id_type: InstrumentIdType::Ticker as i32,
                class_code,
                id,
            },
            InstrumentRequest::Uid(uid) => api::InstrumentRequest {
                id_type: InstrumentIdType::Uid as i32,
                class_code: "".to_string(),
                id: uid,
            },
        };
        req.into_request()
    }
}

service!(InstrumentsClient, InstrumentsServiceClient<Inner>, {
    method!(bond_by, InstrumentRequest, BondResponse, thin);
});
