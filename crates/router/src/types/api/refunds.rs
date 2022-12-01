use serde::{Deserialize, Serialize};

use super::ConnectorCommon;
use crate::{
    services::api,
    types::{self, storage::enums},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefundRequest {
    pub refund_id: Option<String>,
    pub payment_id: String,
    pub merchant_id: Option<String>,
    pub amount: Option<i32>,
    pub reason: Option<String>,
    //FIXME: Make it refund_type instant or scheduled refund
    pub force_process: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

impl super::Router for RefundRequest {}

#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct RefundResponse {
    pub refund_id: String,
    pub payment_id: String,
    pub amount: i32,
    pub currency: String,
    pub reason: Option<String>,
    pub status: RefundStatus,
    pub metadata: Option<serde_json::Value>,
    pub error_message: Option<String>,
}

#[derive(Debug, Eq, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Succeeded,
    Failed,
    Pending,
    Review,
}

impl Default for RefundStatus {
    fn default() -> Self {
        RefundStatus::Pending
    }
}

impl From<enums::RefundStatus> for RefundStatus {
    fn from(status: enums::RefundStatus) -> Self {
        match status {
            enums::RefundStatus::Failure | enums::RefundStatus::TransactionFailure => {
                RefundStatus::Failed
            }
            enums::RefundStatus::ManualReview => RefundStatus::Review,
            enums::RefundStatus::Pending => RefundStatus::Pending,
            enums::RefundStatus::Success => RefundStatus::Succeeded,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Execute;
#[derive(Debug, Clone)]
pub struct RSync;

pub trait RefundExecute:
    api::ConnectorIntegration<Execute, types::RefundsData, types::RefundsResponseData>
{
}

pub trait RefundSync:
    api::ConnectorIntegration<RSync, types::RefundsData, types::RefundsResponseData>
{
}

pub trait Refund: ConnectorCommon + RefundExecute + RefundSync {}
