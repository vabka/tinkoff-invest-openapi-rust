#[derive(Debug)]
pub struct TinkoffInvestError {
    formatted_message: String,
    error_type: ErrorType,
    description: String,
    code: String
}

impl std::error::Error for TinkoffInvestError {}

impl std::fmt::Display for TinkoffInvestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.code, self.error_type, self.formatted_message)
    }
}

impl TinkoffInvestError {
    pub fn formatted_message(&self) -> &str {
        self.formatted_message.as_ref()
    }
    
    pub fn error_type(&self) -> &ErrorType {
        &self.error_type
    }
    
    pub fn description(&self) -> &str {
        self.description.as_ref()
    }
    
    pub fn code(&self) -> &str {
        self.code.as_ref()
    }
}

impl From<tonic::Status> for TinkoffInvestError {
    fn from(status: tonic::Status) -> TinkoffInvestError {
        let code = status.message();
        let err = ErrorType::from_code(code);
        let (message, description) = get_description_by_code(code);
        TinkoffInvestError {
            code: code.to_string(),
            error_type: err,
            description: description,
            formatted_message: message
        }
    }
}

impl ErrorType {
    pub fn from_code(code: &str) -> Self {
        match code {
            "50007" => Self::NotFound,
            "30064" => Self::InvalidArgument,
            "30060" => Self::InvalidArgument,
            "70001" => Self::Internal,
            "30021" => Self::InvalidArgument,
            "30047" => Self::InvalidArgument,
            "30054" => Self::InvalidArgument,
            "30062" => Self::InvalidArgument,
            "30013" => Self::InvalidArgument,
            "30011" => Self::InvalidArgument,
            "30028" => Self::InvalidArgument,
            "30044" => Self::InvalidArgument,
            "30085" => Self::InvalidArgument,
            "70003" => Self::Internal,
            "30004" => Self::InvalidArgument,
            "30014" => Self::InvalidArgument,
            "80001" => Self::ResourceExhausted,
            "50008" => Self::NotFound,
            "30031" => Self::InvalidArgument,
            "30059" => Self::InvalidArgument,
            "30079" => Self::InvalidArgument,
            "30003" => Self::InvalidArgument,
            "30037" => Self::InvalidArgument,
            "30043" => Self::InvalidArgument,
            "30089" => Self::InvalidArgument,
            "30034" => Self::InvalidArgument,
            "30026" => Self::InvalidArgument,
            "30029" => Self::InvalidArgument,
            "30032" => Self::InvalidArgument,
            "30018" => Self::InvalidArgument,
            "30040" => Self::InvalidArgument,
            "30020" => Self::InvalidArgument,
            "30041" => Self::InvalidArgument,
            "30056" => Self::InvalidArgument,
            "30061" => Self::InvalidArgument,
            "30080" => Self::InvalidArgument,
            "30084" => Self::InvalidArgument,
            "30063" => Self::InvalidArgument,
            "30068" => Self::InvalidArgument,
            "30081" => Self::InvalidArgument,
            "30086" => Self::InvalidArgument,
            "30091" => Self::InvalidArgument,
            "40003" => Self::Unauthenticated,
            "40004" => Self::PermissionDenied,
            "30001" => Self::InvalidArgument,
            "30083" => Self::InvalidArgument,
            "30027" => Self::InvalidArgument,
            "30065" => Self::InvalidArgument,
            "30070" => Self::InvalidArgument,
            "30002" => Self::InvalidArgument,
            "30022" => Self::InvalidArgument,
            "30033" => Self::InvalidArgument,
            "80003" => Self::ResourceExhausted,
            "30049" => Self::InvalidArgument,
            "50002" => Self::NotFound,
            "30048" => Self::InvalidArgument,
            "30019" => Self::InvalidArgument,
            "30012" => Self::InvalidArgument,
            "30005" => Self::InvalidArgument,
            "30036" => Self::InvalidArgument,
            "30051" => Self::InvalidArgument,
            "30053" => Self::InvalidArgument,
            "30058" => Self::InvalidArgument,
            "30092" => Self::InvalidArgument,
            "50001" => Self::NotFound,
            "50010" => Self::NotFound,
            "70002" => Self::Internal,
            "30066" => Self::InvalidArgument,
            "12001" => Self::Unimplemented,
            "80002" => Self::ResourceExhausted,
            "50006" => Self::NotFound,
            "30008" => Self::InvalidArgument,
            "30015" => Self::InvalidArgument,
            "30016" => Self::InvalidArgument,
            "30038" => Self::InvalidArgument,
            "30042" => Self::InvalidArgument,
            "30082" => Self::InvalidArgument,
            "50004" => Self::NotFound,
            "90001" => Self::FailedPrecondition,
            "90002" => Self::FailedPrecondition,
            "30078" => Self::InvalidArgument,
            "30067" => Self::InvalidArgument,
            "50005" => Self::NotFound,
            "50009" => Self::NotFound,
            "30017" => Self::InvalidArgument,
            "30006" => Self::InvalidArgument,
            "30007" => Self::InvalidArgument,
            "30050" => Self::InvalidArgument,
            "30023" => Self::InvalidArgument,
            "30045" => Self::InvalidArgument,
            "30052" => Self::InvalidArgument,
            "30055" => Self::InvalidArgument,
            "30009" => Self::InvalidArgument,
            "30010" => Self::InvalidArgument,
            "30057" => Self::InvalidArgument,
            "30069" => Self::InvalidArgument,
            "40002" => Self::PermissionDenied,
            "30090" => Self::InvalidArgument,
            "30077" => Self::InvalidArgument,
            "30087" => Self::InvalidArgument,
            "30025" => Self::InvalidArgument,
            "30030" => Self::InvalidArgument,
            "30088" => Self::InvalidArgument,
            _ => Self::Unknown,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum ErrorType {
    FailedPrecondition,
    InvalidArgument,
    NotFound,
    Unauthenticated,
    Unimplemented,
    ResourceExhausted,
    PermissionDenied,
    Internal,
    Unknown,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedPrecondition => write!(f, "FAILED_PRECONDITION"),
            Self::InvalidArgument => write!(f, "INVALID_ARGUMENT"),
            Self::NotFound => write!(f, "NOT_FOUND"),
            Self::Unauthenticated => write!(f, "UNAUTHENTICATED"),
            Self::Unimplemented => write!(f, "UNIMPLEMENTED"),
            Self::ResourceExhausted => write!(f, "RESOURCE_EXHAUSTED"),
            Self::PermissionDenied => write!(f, "PERMISSION_DENIED"),
            Self::Internal => write!(f, "INTERNAL"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}

fn get_description_by_code(code: &str) -> (String, String) {
    match code {
        "50007" => (r"task not found".to_string(),
                    r##"Задача не найдена."##
                    .to_string()),
        "30064" => (r"the required period should not exceed 31 days".to_string(),
                    r##"Запрошенный период не может превышать 31 дня.
Укажите корректный период."##
                    .to_string()),
        "30060" => (r"cancel stop-order error: %s".to_string(),
                    r##"Ошибка метода отмены стоп-заявки.
Подробнее в тексте ошибки."##
                    .to_string()),
        "70001" => (r"internal error".to_string(),
                    r##"Внутренняя ошибка сервиса.
Если ошибка повторяется, обратитесь в службу технической поддержки."##
                    .to_string()),
        "30021" => (r"missing parameter: 'account_id'".to_string(),
                    r##"Входной параметр *account_id* является обязательным.
Укажите корректный параметр *account_id*."##
                    .to_string()),
        "30047" => (r"price currency does not match the settlement currency".to_string(),
                    r##"Валюта цены не совпадает с валютой расчётов по инструменту.
Укажите корректную валюту цены."##
                    .to_string()),
        "30054" => (r"instrument type is not a share or etf".to_string(),
                    r##"Тип инструмента не инвестиционный фонд или акция"##
                    .to_string()),
        "30062" => (r"'to' value out of range".to_string(),
                    r##"Входной параметр *to* имеет некорректное значение.
Укажите значение в пределах от 1970-01-01T00:00:00Z до 2099-12-31T23:59:59.999999999Z включительно."##
                    .to_string()),
        "30013" => (r"'class_code' require for 'id_type' = 'ticker'".to_string(),
                    r##"Входной параметр *class_code* не может быть пустым при поиске по тикеру.
Укажите корректный параметр *class_code*.
Подробнее: https://tinkoff.github.io/investAPI/faq_identification/."##
                    .to_string()),
        "30011" => (r"interval is invalid".to_string(),
                    r##"Входной параметр *interval* имеет некорректное значение.
Список доступных значений: https://tinkoff.github.io/investAPI/marketdata#subscriptioninterval."##
                    .to_string()),
        "30028" => (r"'order id' is invalid".to_string(),
                    r##"Входной параметр *order_id* имеет некорректное значение.
Укажите корректный параметр *order_id*."##
                    .to_string()),
        "30044" => (r"'expiration_type' is invalid".to_string(),
                    r##"Входной параметр *expiration_type* имеет некорректное значение.
Список доступных значений: https://tinkoff.github.io/investAPI/stoporders/#stoporderexpirationtype."##
                    .to_string()),
        "30085" => (r"price in points is available only for futures and bonds".to_string(),
                    r##"Тип цены в пунктах доступен только для фьючерсов и облигаций."##
                    .to_string()),
        "70003" => (r"internal error, please try again later".to_string(),
                    r##"Внутренняя ошибка сервиса, попробуйте выполнить запрос позднее.
Если ошибка повторяется, обратитесь в службу технической поддержки."##
                    .to_string()),
        "30004" => (r"missing parameter: 'to'".to_string(),
                    r##"Входной параметр *to* является обязательным.
Укажите корректный параметр *to*."##
                    .to_string()),
        "30014" => (r"the maximum request period for the given candle interval has been exceeded".to_string(),
                    r##"Превышен максимальный период запроса для данного интервала свечи.
Укажите корректный интервал."##
                    .to_string()),
        "80001" => (r"limit of open streams exceeded".to_string(),
                    r##"Превышен лимит одновременных открытых stream-соединений. Подробнее: https://tinkoff.github.io/investAPI/limits/"##
                    .to_string()),
        "50008" => (r"no orderbook provider".to_string(),
                    r##"Отсутствует источник данных по стаканам."##
                    .to_string()),
        "30031" => (r"missing parameter: 'depth'".to_string(),
                    r##"Входной параметр *depth* является обязательным.
Укажите корректный параметр *depth*."##
                    .to_string()),
        "30059" => (r"cancel order error: %s".to_string(),
                    r##"Ошибка метода отмены заявки.
Подробнее в тексте ошибки."##
                    .to_string()),
        "30079" => (r"instrument is not available for trading".to_string(),
                    r##"Инструмент недоступен для торгов."##
                    .to_string()),
        "30003" => (r"'from' can't be less than the current date".to_string(),
                    r##"Входной параметр *from* не может быть меньше текущей даты.
Укажите корректный параметр *from*."##
                    .to_string()),
        "30037" => (r"missing parameter: 'stop_order_type'".to_string(),
                    r##"Входной параметр *stop_order_type* является обязательным.
Укажите корректный параметр *stop_order_type*."##
                    .to_string()),
        "30043" => (r"missing parameter: 'expiration_type'".to_string(),
                    r##"Входной параметр *expiration_type* является обязательным.
Укажите корректный параметр *expiration_type*."##
                    .to_string()),
        "30089" => (r"'to' must not be later than %s".to_string(),
                    r##"Поле to не должно быть позднее даты, указанной в тексте ошибки.
Дата указана в формате ISO 8601."##
                    .to_string()),
        "30034" => (r"not enough balance".to_string(),
                    r##"Недостаточно средств для совершения сделки (ошибка песочницы).
Пополните баланс нужной валюты."##
                    .to_string()),
        "30026" => (r"invalid parameter: 'order_type'".to_string(),
                    r##"Входной параметр *order_type* имеет некорректное значение.
Укажите корректный параметр *order_type*."##
                    .to_string()),
        "30029" => (r"missing parameter: 'idempotency_key'".to_string(),
                    r##"Входной параметр *idempotency_key* является обязательным.
Укажите корректный параметр *idempotency_key*."##
                    .to_string()),
        "30032" => (r"depth is invalid".to_string(),
                    r##"Входной параметр *depth* имеет некорректное значение.
Укажите корректный параметр *depth*."##
                    .to_string()),
        "30018" => (r"price is invalid".to_string(),
                    r##"Входной параметр *price* имеет некорректное значение.
Укажите корректный параметр *price*."##
                    .to_string()),
        "30040" => (r"'expire_date' is invalid".to_string(),
                    r##"Входной параметр *expire_date* имеет некорректное значение.
Укажите корректный параметр *expire_date*."##
                    .to_string()),
        "30020" => (r"'direction' is invalid".to_string(),
                    r##"Входной параметр *direction* имеет некорректное значение."##
                    .to_string()),
        "30041" => (r"the method is available only for futures".to_string(),
                    r##"Метод предназначен только для работы с фьючерсами.
Передайте во входные параметры метода идентификатор фьючерса."##
                    .to_string()),
        "30056" => (r"stop order settlement currency is not supported".to_string(),
                    r##"Валюта выставления стоп-заявки не поддерживается"##
                    .to_string()),
        "30061" => (r"'from' value out of range".to_string(),
                    r##"Входной параметр *from* имеет некорректное значение.
Укажите значение в пределах от 1970-01-01T00:00:00Z до 2099-12-31T23:59:59.999999999Z включительно."##
                    .to_string()),
        "30080" => (r"quantity must be positive".to_string(),
                    r##"Количество лотов должно быть положительным числом."##
                    .to_string()),
        "30084" => (r"maximum request period has been exceeded".to_string(),
                    r##"Превышен лимит запрашиваемого периода."##
                    .to_string()),
        "30063" => (r"'expire_date' value out of range".to_string(),
                    r##"Входной параметр *expire_date* имеет некорректное значение.
Дата должна быть не ранее текущей и не позднее 2099-12-31T23:59:59.999999999Z."##
                    .to_string()),
        "30068" => (r"Only limit order is allowed".to_string(),
                    r##"В настоящий момент возможно выставление только лимитного торгового поручения."##
                    .to_string()),
        "30081" => (r"account status is closed".to_string(),
                    r##"Аккаунт закрыт."##
                    .to_string()),
        "30086" => (r"year is invalid".to_string(),
                    r##"Некорректный год."##
                    .to_string()),
        "30091" => (r"quantity of instruments can't be more than 100".to_string(),
                    r##"Количество инструментов не может превышать 100."##
                    .to_string()),
        "40003" => (r"authentication token is missing or invalid".to_string(),
                    r##"Токен доступа не найден или не активен.
Вы можете выпустить новый токен по ссылке https://www.tinkoff.ru/invest/settings/"##
                    .to_string()),
        "40004" => (r"Working with orders is not available with this account".to_string(),
                    r##"Выставление заявок недоступно с текущего аккаунта."##
                    .to_string()),
        "30001" => (r"missing parameter: 'from'".to_string(),
                    r##"Входной параметр *from* является обязательным.
Укажите корректный параметр *from*."##
                    .to_string()),
        "30083" => (r"order_type is invalid".to_string(),
                    r##"Некорректный тип заявки."##
                    .to_string()),
        "30027" => (r"missing parameter: 'order_id'".to_string(),
                    r##"Входной параметр *order_id* является обязательным.
Укажите корректный параметр *order_id*."##
                    .to_string()),
        "30065" => (r"missing parameter: 'task_id'".to_string(),
                    r##"Входной параметр *task_id* является обязательным.
Укажите корректный параметр *task_id*."##
                    .to_string()),
        "30070" => (r"'from' can't be more than the current date".to_string(),
                    r##"Входной параметр *from* имеет некорректное значение.
Укажите корректный параметр."##
                    .to_string()),
        "30002" => (r"the required period should not exceed 7 days".to_string(),
                    r##"Запрошенный период не может превышать 7 дней.
Укажите корректный период."##
                    .to_string()),
        "30022" => (r"missing parameter: 'state'".to_string(),
                    r##"Входной параметр *state* является обязательным.
Укажите корректный параметр *state*.
Список доступных значений: https://tinkoff.github.io/investAPI/operations/#operationstate."##
                    .to_string()),
        "30033" => (r"missing parameter: 'trade_clearing_account' or 'class_code'".to_string(),
                    r##"Параметр *trade_clearing_account* или *class_code* не может быть пустым."##
                    .to_string()),
        "80003" => (r"The limit on SMS sending has been exceeded, try again in a minute".to_string(),
                    r##"Превышен лимит на отправку СМС, попробуйте через минуту."##
                    .to_string()),
        "30049" => (r"post order error: %s".to_string(),
                    r##"Ошибка метода выставления торгового поручения.
Подробнее в тексте ошибки."##
                    .to_string()),
        "50002" => (r"instrument not found".to_string(),
                    r##"Инструмент не найден.
Укажите корректный идентификатор инструмента."##
                    .to_string()),
        "30048" => (r"instrument type is not bond".to_string(),
                    r##"Метод предназначен только для запроса информации по облигации.
Передайте во входные параметры метода идентификатор облигации."##
                    .to_string()),
        "30019" => (r"missing parameter: 'direction'".to_string(),
                    r##"Входной параметр *direction* является обязательным.
Укажите корректный параметр *direction*."##
                    .to_string()),
        "30012" => (r"'to' can't be less than 'from'".to_string(),
                    r##"Входной параметр *to* не может быть меньше параметра *from*.
Укажите корректные параметры *from* и *to*."##
                    .to_string()),
        "30005" => (r"'id_type' is invalid".to_string(),
                    r##"Входной параметр *id_type* имеет некорректное значение.
Список доступных значений: https://tinkoff.github.io/investAPI/instruments#instrumentidtype."##
                    .to_string()),
        "30036" => (r"missing parameter: 'stop_price'".to_string(),
                    r##"Входной параметр *stop_price* является обязательным.
Укажите корректный параметр *stop_price*."##
                    .to_string()),
        "30051" => (r"account margin status is disabled".to_string(),
                    r##"Для данного договора недоступна маржинальная торговля."##
                    .to_string()),
        "30053" => (r"post stop_order error: %s".to_string(),
                    r##"Ошибка метода выставления стоп-заявки.
Подробнее в тексте ошибки."##
                    .to_string()),
        "30058" => (r"task not completed yet, please try again later".to_string(),
                    r##"Выполнение задачи еще не завершено, попробуйте позже."##
                    .to_string()),
        "30092" => (r"trading unavailable on weekends".to_string(),
                    r##"Торги недоступны по нерабочим дням."##
                    .to_string()),
        "50001" => (r"exchange not found".to_string(),
                    r##"Биржа не найдена по переданному *exchange_id*.
Укажите корректный *exchange_id*."##
                    .to_string()),
        "50010" => (r"brand not found".to_string(),
                    r##"Бренд не найден.
Укажите корректный идентификатор бренда."##
                    .to_string()),
        "70002" => (r"internal network error".to_string(),
                    r##"Неизвестная сетевая ошибка, попробуйте выполнить запрос позднее.
Если ошибка повторяется, обратитесь в службу технической поддержки."##
                    .to_string()),
        "30066" => (r"missing parameter: 'payload'".to_string(),
                    r##"Входной параметр *payload* является обязательным.
Укажите корректный параметр *payload*."##
                    .to_string()),
        "12001" => (r"Method is unimplemented".to_string(),
                    r##"Метод не реализован."##
                    .to_string()),
        "80002" => (r"request limit exceeded".to_string(),
                    r##"Превышен лимит запросов в минуту. Подробнее: https://tinkoff.github.io/investAPI/limits/"##
                    .to_string()),
        "50006" => (r"stop-order not found".to_string(),
                    r##"Стоп-заявка не найдена по переданному *stop_order_id*.
Укажите корректный *stop_order_id*."##
                    .to_string()),
        "30008" => (r"missing parameter: 'figi'".to_string(),
                    r##"Входной параметр *figi* является обязательным.
Укажите корректный параметр *figi*."##
                    .to_string()),
        "30015" => (r"missing parameter: 'quantity'".to_string(),
                    r##"Входной параметр *quantity* является обязательным.
Укажите корректный параметр *quantity*."##
                    .to_string()),
        "30016" => (r"'quantity' is invalid".to_string(),
                    r##"Входной параметр *quantity* имеет некорректное значение.
Укажите корректный параметр *quantity*."##
                    .to_string()),
        "30038" => (r"'stop_order_type' is invalid".to_string(),
                    r##"Входной параметр *stop_order_type* имеет некорректное значение.
Список доступных значений: https://tinkoff.github.io/investAPI/stoporders/#stopordertype."##
                    .to_string()),
        "30042" => (r"not enough assets for a margin trade".to_string(),
                    r##"Недостаточно активов для маржинальной сделки.
Проверьте маржинальные показатели счёта."##
                    .to_string()),
        "30082" => (r"account status is blocked".to_string(),
                    r##"Аккаунт заблокирован."##
                    .to_string()),
        "50004" => (r"account not found".to_string(),
                    r##"Счёт не найден по переданному *account_id*.
Укажите корректный *account_id*."##
                    .to_string()),
        "90001" => (r"need confirmation: %s".to_string(),
                    r##"Требуется подтверждение операции.
Подробнее в тексте ошибки."##
                    .to_string()),
        "90002" => (r"only for qualified investors".to_string(),
                    r##"Торговля этим инструментом доступна только квалифицированным инвесторам."##
                    .to_string()),
        "30078" => (r"Incorrect minimum price increment".to_string(),
                    r##"Некорректный шаг изменения цены."##
                    .to_string()),
        "30067" => (r"'action_type' is invalid".to_string(),
                    r##"Некорректное значение *action_type*.
Выберите допустимое значение."##
                    .to_string()),
        "50005" => (r"order not found".to_string(),
                    r##"Торговое поручение не найдено по переданному *order_id*.
Укажите корректный *order_id*."##
                    .to_string()),
        "50009" => (r"asset not found".to_string(),
                    r##"Актив не найден.
Укажите корректный идентификатор актива."##
                    .to_string()),
        "30017" => (r"missing parameter: 'price'".to_string(),
                    r##"Входной параметр *price* является обязательным.
Укажите корректный параметр *price*."##
                    .to_string()),
        "30006" => (r"missing parameter: 'id_type'".to_string(),
                    r##"Входной параметр *id_type* является обязательным.
Укажите корректный параметр *id_type*."##
                    .to_string()),
        "30007" => (r"missing parameter: 'id'".to_string(),
                    r##"Входной параметр *id* является обязательным.
Укажите корректный параметр *id*."##
                    .to_string()),
        "30050" => (r"'instrument_status' is invalid".to_string(),
                    r##"Входной параметр *instrument_status* имеет некорректное значение.
Список доступных значений: https://tinkoff.github.io/investAPI/instruments#instrumentstatus."##
                    .to_string()),
        "30023" => (r"invalid parameter: 'state'".to_string(),
                    r##"Входной параметр *state* имеет некорректное значение.
Укажите корректный параметр *state*.
Список доступных значений: https://tinkoff.github.io/investAPI/operations/#operationstate."##
                    .to_string()),
        "30045" => (r"missing parameter: 'ticker'".to_string(),
                    r##"Входной параметр *ticker* является обязательным.
Укажите корректный параметр *ticker*."##
                    .to_string()),
        "30052" => (r"instrument forbidden for trading by API".to_string(),
                    r##"Для данного инструмента недоступна торговля через API."##
                    .to_string()),
        "30055" => (r"order_id cannot be longer than 36 characters".to_string(),
                    r##"order_id не может быть длиннее 36 символов"##
                    .to_string()),
        "30009" => (r"'from' is invalid".to_string(),
                    r##"Входной параметр *from* имеет некорректное значение.
Укажите корректный параметр *from*."##
                    .to_string()),
        "30010" => (r"'to' is invalid".to_string(),
                    r##"Входной параметр *to* имеет некорректное значение.
Укажите корректный параметр *to*."##
                    .to_string()),
        "30057" => (r"the order is a duplicate, but the order report was not found".to_string(),
                    r##"Заявка является дублем, но отчет по заявке не найден"##
                    .to_string()),
        "30069" => (r"Invalid parameter: 'limit'".to_string(),
                    r##"Входной параметр *limit* имеет некорректное значение.
Значение должно быть меньше или равно 1000."##
                    .to_string()),
        "40002" => (r"insufficient privileges".to_string(),
                    r##"Недостаточно прав для совершения операции.
Токен доступа имеет уровень прав read-only.
Подробнее: https://tinkoff.github.io/investAPI/index#_2"##
                    .to_string()),
        "30090" => (r"missing parameter: 'siebel_id'".to_string(),
                    r##"Входной параметр *siebel_id* является обязательным.
Укажите корректный параметр *siebel_id*."##
                    .to_string()),
        "30077" => (r"not available for OTC instruments".to_string(),
                    r##"Метод недоступен для внебиржевых инструментов."##
                    .to_string()),
        "30087" => (r"missing parameter: query".to_string(),
                    r##"Входной параметр *query* является обязательным.
Укажите не пустой параметр *query*."##
                    .to_string()),
        "30025" => (r"missing parameter: 'order_type'".to_string(),
                    r##"Входной параметр *order_type* является обязательным.
Укажите корректный параметр *order_type*."##
                    .to_string()),
        "30030" => (r"'idempotency_key' is invalid".to_string(),
                    r##"Входной параметр *idempotency_key* имеет некорректное значение.
Укажите корректный параметр *idempotency_key*."##
                    .to_string()),
        "30088" => (r"'from' and 'to' must have the same year".to_string(),
                    r##"Запрашиваемые даты должны быть в рамках одного года."##
                    .to_string()),
        _ => ("N/A".to_string(), "N/A".to_string()),
    }
}
