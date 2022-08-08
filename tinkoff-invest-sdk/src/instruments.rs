use tinkoff_invest_grpc::api::{self, InstrumentIdType};
use tinkoff_invest_grpc::Inner;
use tinkoff_invest_grpc::api::instruments_service_client::InstrumentsServiceClient;

use crate::{service, types::Bond};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentRequest {
    Figi(String),
    Ticker { id: String, class_code: String },
    Uid(String),
}

impl Into<api::InstrumentRequest> for InstrumentRequest {
    fn into(self) -> api::InstrumentRequest {
        match self {
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
        }
    }
}

service!(InstrumentsClient, InstrumentsServiceClient<Inner>);
impl InstrumentsClient {
    pub async fn bond_by(&mut self, request: InstrumentRequest) -> crate::Result<Option<Bond>> {
        let req: api::InstrumentRequest = request.into();
        let response = self.internal.bond_by(req).await?;
        let inner = response.into_inner();
        let bond = inner.instrument;
        Ok(bond.map(Into::into))
    }
}
