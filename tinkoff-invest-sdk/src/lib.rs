mod generated;
pub mod instruments;
pub mod types;
pub mod users;

use users::*;
pub use generated::errors as error;

use error::TinkoffInvestError;
use instruments::InstrumentsClient;
pub use chrono;
pub use tinkoff_invest_grpc::decimal;
pub struct TinkoffInvestClient {
    internal: tinkoff_invest_grpc::TinkoffInvestClient,
}

pub type Result<T> = core::result::Result<T, TinkoffInvestError>;
macro_rules! service {
    ($name:ident, $internal:ty $(, {$($impl: item)*$(;)*})?) => {
        pub struct $name {
            internal: $internal
        }

        impl $name {
            pub(crate) fn new(internal: $internal) -> Self {
                Self {internal}
            }

            $(
                $(
                $impl
            )*
        )?
        }
    }
}
pub(crate) use service;
macro_rules! method {
    ($method: ident, $req: ident, $ret: ident $(,empty)?) => {
        #[inline]
        pub async fn $method(&mut self) -> $crate::Result<$ret> {
            let req = $req {};
            let res = self.internal.$method(req).await;
            let data = res?.into_inner().into();

            $crate::Result::Ok(data)
        }
    };
    ($method: ident, $req: ident, $ret: ident, thin) => {
        #[inline]
        pub async fn $method(&mut self, request: $req) -> $crate::Result<$ret> {
            let res = self.internal.$method(request).await;
            let data = res?.into_inner().into();

            $crate::Result::Ok(data)
        }
    };
    ($method: ident, $req: ident, $ret: ident, into) => {
        #[inline]
        pub async fn $method(&mut self, request: $req) -> $crate::Result<$ret> {
            let res = self.internal.$method(request.into()).await;
            let data = res?.into_inner().into();

            $crate::Result::Ok(data)
        }
    };
}
pub(crate) use method;

macro_rules! service_getter {
    ($name: ident, $service: ident) => {
        pub fn $name(&self) -> $service {
            $service::new(self.internal.$name())
        }
    };
}

impl TinkoffInvestClient {
    pub async fn connect(token: &str) -> core::result::Result<Self, Box<dyn std::error::Error>> {
        let internal = tinkoff_invest_grpc::TinkoffInvestClient::connect(token).await?;
        Ok(Self { internal })
    }
    service_getter!(users, UsersClient);
    service_getter!(instruments, InstrumentsClient);
    // market_data
    // market_data_stream
    // operations
    // operations_stream
    // orders
    // orders_stream
    // sandbox
    // stop_orders
}
