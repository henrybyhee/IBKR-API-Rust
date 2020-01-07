use std::any::Any;
use std::convert::TryInto;
use std::io::Write;
use std::ops::Deref;
use std::string::String;
use std::sync::{Arc, RwLock};
use std::vec::Vec;

use ascii;
use ascii::AsAsciiStr;
use bytebuffer::ByteBuffer;
use byteorder::{BigEndian, ByteOrder};

use crate::core::common::{UNSET_DOUBLE, UNSET_INTEGER, UNSET_LONG};
use crate::core::decoder::Decoder;
use crate::core::wrapper::Wrapper;

trait EClientMsgSink {
    fn server_version(version: i32, time: &str);
    fn redirect(host: &str);
}

pub enum FAMessageDataTypes {
    // FA msg data types
    Groups = 1,
    Profiles = 2,
    Aliases = 3,
}

trait IncomingMessageProcessor {
    fn process(wrapper: &mut dyn Wrapper, params: &Vec<String>);
}

pub struct MessageProcessor<'a, T: Wrapper> {
    wrapper: Arc<RwLock<&'a T>>,
    //decoder: Arc<RwLock<Decoder<'a, T>>>,
}

// incoming msg id's
#[derive(FromPrimitive)]
#[repr(i32)]
pub enum IncomingMessageIds {
    TickPrice = 1,
    TickSize = 2,
    OrderStatus = 3,
    ErrMsg = 4,
    OpenOrder = 5,
    AcctValue = 6,
    PortfolioValue = 7,
    AcctUpdateTime = 8,
    NextValidId = 9,
    ContractData = 10,
    ExecutionData = 11,
    MarketDepth = 12,
    MarketDepthL2 = 13,
    NewsBulletins = 14,
    ManagedAccts = 15,
    ReceiveFa = 16,
    HistoricalData = 17,
    BondContractData = 18,
    ScannerParameters = 19,
    ScannerData = 20,
    TickOptionComputation = 21,
    TickGeneric = 45,
    TickString = 46,
    TickEfp = 47,
    CurrentTime = 49,
    RealTimeBars = 50,
    FundamentalData = 51,
    ContractDataEnd = 52,
    OpenOrderEnd = 53,
    AcctDownloadEnd = 54,
    ExecutionDataEnd = 55,
    DeltaNeutralValidation = 56,
    TickSnapshotEnd = 57,
    MarketDataType = 58,
    CommissionReport = 59,
    PositionData = 61,
    PositionEnd = 62,
    AccountSummary = 63,
    AccountSummaryEnd = 64,
    VerifyMessageApi = 65,
    VerifyCompleted = 66,
    DisplayGroupList = 67,
    DisplayGroupUpdated = 68,
    VerifyAndAuthMessageApi = 69,
    VerifyAndAuthCompleted = 70,
    PositionMulti = 71,
    PositionMultiEnd = 72,
    AccountUpdateMulti = 73,
    AccountUpdateMultiEnd = 74,
    SecurityDefinitionOptionParameter = 75,
    SecurityDefinitionOptionParameterEnd = 76,
    SoftDollarTiers = 77,
    FamilyCodes = 78,
    SymbolSamples = 79,
    MktDepthExchanges = 80,
    TickReqParams = 81,
    SmartComponents = 82,
    NewsArticle = 83,
    TickNews = 84,
    NewsProviders = 85,
    HistoricalNews = 86,
    HistoricalNewsEnd = 87,
    HeadTimestamp = 88,
    HistogramData = 89,
    HistoricalDataUpdate = 90,
    RerouteMktDataReq = 91,
    RerouteMktDepthReq = 92,
    MarketRule = 93,
    Pnl = 94,
    PnlSingle = 95,
    HistoricalTicks = 96,
    HistoricalTicksBidAsk = 97,
    HistoricalTicksLast = 98,
    TickByTick = 99,
    OrderBound = 100,
    CompletedOrder = 101,
    CompletedOrdersEnd = 102,
}

// Outgoing msg id's
#[derive(FromPrimitive)]
#[repr(i32)]
pub enum OutgoingMessageIds {
    ReqMktData = 1,
    CancelMktData = 2,
    PlaceOrder = 3,
    CancelOrder = 4,
    ReqOpenOrders = 5,
    ReqAcctData = 6,
    ReqExecutions = 7,
    ReqIds = 8,
    ReqContractData = 9,
    ReqMktDepth = 10,
    CancelMktDepth = 11,
    ReqNewsBulletins = 12,
    CancelNewsBulletins = 13,
    SetServerLoglevel = 14,
    ReqAutoOpenOrders = 15,
    ReqAllOpenOrders = 16,
    ReqManagedAccts = 17,
    ReqFa = 18,
    ReplaceFa = 19,
    ReqHistoricalData = 20,
    ExerciseOptions = 21,
    ReqScannerSubscription = 22,
    CancelScannerSubscription = 23,
    ReqScannerParameters = 24,
    CancelHistoricalData = 25,
    ReqCurrentTime = 49,
    ReqRealTimeBars = 50,
    CancelRealTimeBars = 51,
    ReqFundamentalData = 52,
    CancelFundamentalData = 53,
    ReqCalcImpliedVolat = 54,
    ReqCalcOptionPrice = 55,
    CancelCalcImpliedVolat = 56,
    CancelCalcOptionPrice = 57,
    ReqGlobalCancel = 58,
    ReqMarketDataType = 59,
    ReqPositions = 61,
    ReqAccountSummary = 62,
    CancelAccountSummary = 63,
    CancelPositions = 64,
    VerifyRequest = 65,
    VerifyMessage = 66,
    QueryDisplayGroups = 67,
    SubscribeToGroupEvents = 68,
    UpdateDisplayGroup = 69,
    UnsubscribeFromGroupEvents = 70,
    StartApi = 71,
    VerifyAndAuthRequest = 72,
    VerifyAndAuthMessage = 73,
    ReqPositionsMulti = 74,
    CancelPositionsMulti = 75,
    ReqAccountUpdatesMulti = 76,
    CancelAccountUpdatesMulti = 77,
    ReqSecDefOptParams = 78,
    ReqSoftDollarTiers = 79,
    ReqFamilyCodes = 80,
    ReqMatchingSymbols = 81,
    ReqMktDepthExchanges = 82,
    ReqSmartComponents = 83,
    ReqNewsArticle = 84,
    ReqNewsProviders = 85,
    ReqHistoricalNews = 86,
    ReqHeadTimestamp = 87,
    ReqHistogramData = 88,
    CancelHistogramData = 89,
    CancelHeadTimestamp = 90,
    ReqMarketRule = 91,
    ReqPnl = 92,
    CancelPnl = 93,
    ReqPnlSingle = 94,
    CancelPnlSingle = 95,
    ReqHistoricalTicks = 96,
    ReqTickByTickData = 97,
    CancelTickByTickData = 98,
    ReqCompletedOrders = 99,
}

pub struct EMessage {
    buffer: ByteBuffer,
}

trait NewEmessageFrom<T> {
    fn new(buf: T) -> EMessage;
}

impl NewEmessageFrom<&[u8]> for EMessage {
    fn new(buf: &[u8]) -> EMessage {
        let mut msg = EMessage::new();
        msg.buffer.write(buf);
        msg
    }
}

//impl NewEmessageFrom<&Builder> for EMessage {
//    fn new(builder: &Builder) -> EMessage {
//        let mut msg = EMessage::new();
//        builder.write_out(&mut msg.buffer);
//        msg
//    }
//}

impl EMessage {
    pub fn new() -> Self {
        EMessage {
            buffer: ByteBuffer::new(),
        }
    }
    pub fn get_stream(&self) -> &ByteBuffer {
        &self.buffer
    }
    pub fn get_raw_data(&self) -> Vec<u8> {
        self.buffer.to_bytes()
    }
}

pub fn make_message(msg: &str) -> Vec<u8> {
    //let mut buffer = ByteBuffer::new();
    let mut buffer: Vec<u8> = Vec::new();

    buffer.extend_from_slice(&i32::to_be_bytes(msg.len() as i32));

    buffer.write(msg.as_ascii_str().unwrap().as_bytes());
    let tmp = buffer.clone();
    //debug!("Message after create: {:?}", buffer);

    let (size, msg, buf) = read_msg(tmp.as_slice());
    //debug!("Message read: size:{}, msg:{}, bytes: {:?}", size, msg, buf);

    tmp
}

pub fn read_msg<'a>(buf: &[u8]) -> (usize, String, Vec<u8>) {
    // first the size prefix and then the corresponding msg payload ""
    let mut text = String::new();
    if buf.len() < 4 {
        error!("read_msg:  buffer too small!! {:?}", buf.len());
        return (0, String::new(), buf.to_vec());
    }

    let size = i32::from_be_bytes(buf[0..4].try_into().unwrap()) as usize;
    //debug!("read_msg: Message size: {:?}", size);

    if buf.len() - 4 >= size {
        text = String::from_utf8(buf[4..4 + size].to_vec()).unwrap();
        //debug!("read_msg: text in read message: {:?}", text);
        (size, text, buf[4 + size..].to_vec())
    } else {
        (size, String::new(), buf.to_vec())
    }
}

pub fn read_fields(buf: &str) -> Vec<String> {
    //msg payload is made of fields terminated/separated by NULL chars """
    let a = '\u{0}';
    let mut fields: Vec<&str> = buf.split(a).collect::<Vec<&str>>();
    //debug!("fields.len() in read_fields: {}", fields.len());
    //last one is empty
    fields.remove(fields.len() - 1);

    fields
        .iter()
        .map(|x| String::from(*x))
        .collect::<Vec<String>>()
}

pub fn make_field(val: &dyn Any) -> String {
    // adds the NULL string terminator

    // bool type is encoded as int
    if let Some(boolval) = val.downcast_ref::<bool>() {
        format!("{}\0", *boolval as i32)
    } else if let Some(stringval) = val.downcast_ref::<String>() {
        format!("{}\0", stringval)
    } else if let Some(stringval) = val.downcast_ref::<&str>() {
        format!("{}\0", stringval)
    } else if let Some(stringval) = val.downcast_ref::<f64>() {
        format!("{}\0", stringval)
    } else if let Some(stringval) = val.downcast_ref::<i32>() {
        format!("{}\0", stringval)
    } else {
        "".to_string()
    }
}

pub fn make_field_handle_empty(val: &dyn Any) -> String {
    make_field(val)
}