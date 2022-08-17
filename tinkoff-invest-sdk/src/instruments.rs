use tinkoff_invest_grpc::api::{self};
use tinkoff_invest_grpc::Inner;
use tinkoff_invest_grpc::api::instruments_service_client::InstrumentsServiceClient;

use crate::{service, types::Bond};
use crate::error::{TinkoffInvestError, ErrorType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentRequest {
    Figi(String),
    Ticker { id: String, class_code: String },
    Uid(String),
}

impl From<InstrumentRequest> for api::InstrumentRequest {
    fn from(req: InstrumentRequest) -> api::InstrumentRequest {
        let mut request = api::InstrumentRequest::default();        
        match req {
            InstrumentRequest::Figi(figi) =>  {
                request.set_id_type(api::InstrumentIdType::Figi);
                request.id = figi;
            },
            InstrumentRequest::Ticker { id, class_code } => {
                request.set_id_type(api::InstrumentIdType::Ticker);
                request.id = id;
                request.class_code = class_code;
            },
            InstrumentRequest::Uid(uid) =>  {
                request.set_id_type(api::InstrumentIdType::Uid);
                request.id = uid;
            },
        };
        request
    }
}

service!(InstrumentsClient, InstrumentsServiceClient<Inner>);
impl InstrumentsClient {
    pub async fn bond_by(&mut self, request: InstrumentRequest) -> crate::Result<Option<Bond>> {
        let req: api::InstrumentRequest = request.into();
        match self.internal.bond_by(req).await.map_err(Into::into) {
            Ok(response) => {
                let inner = response.into_inner();
                let bond = inner.instrument;
                Ok(bond.map(Into::into))
            },
            // Не смотря на то, что поле bond помечено как Option, если мы укажем не существующий id - нам вернётся 50002 код
            // По тому мы её обрабатываем и делаем Ok(None), а Err оставляем для всех остальных ошибок
            Err(error) => {
                let error: TinkoffInvestError = error;
                match error.error_type() {
                    ErrorType::NotFound => Ok(None),
                    _ => Err(error)
                }
            }
        }
    }

    pub async fn trading_schedules(&mut self) -> crate::Result<String> {
        let req = todo!();
        let res = self.internal.trading_schedules(req).await?;
        let data = res.into_inner();

        todo!()
    }
}
