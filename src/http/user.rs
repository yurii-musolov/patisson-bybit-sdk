use serde::Deserialize;

use crate::VipLevel;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct APIKeyInformation {
    /// Unique ID. Internal use
    pub id: String,
    /// The remark
    pub note: String,
    /// Api key
    pub api_key: String,
    /// 0：Read and Write. 1：Read only
    pub read_only: i64,
    /// Always ""
    pub secret: String,
    /// The types of permission
    pub permissions: APIKeyPermissions,
    /// IP bound
    pub ips: Vec<String>,
    /// The type of api key. 1：personal, 2：connected to the third-party app
    #[serde(rename = "type")]
    pub key_type: i64,
    /// The remaining valid days of api key. Only for those api key with no IP bound or the password has been changed
    pub deadline_day: u64,
    /// The expiry day of the api key. Only for those api key with no IP bound or the password has been changed
    pub expired_at: String,
    /// The create day of the api key
    pub created_at: String,
    /// Whether the account to which the account upgrade to unified trade account. 0：regular account; 1：unified trade account
    pub uta: i64,
    /// User ID
    #[serde(rename = "userID")]
    pub user_id: i64,
    /// Inviter ID (the UID of the account which invited this account to the platform)
    #[serde(rename = "inviterID")]
    pub inviter_id: i64,
    /// VIP Level
    pub vip_level: VipLevel,
    /// Market maker level
    pub mkt_maker_level: String,
    /// Affiliate Id. 0 represents that there is no binding relationship.
    #[serde(rename = "affiliateID")]
    pub affiliate_id: i64,
    /// Rsa public key
    pub rsa_public_key: String,
    /// If this api key belongs to master account or not
    pub is_master: bool,
    /// The main account uid. Returns "0" when the endpoint is called by main account
    pub parent_uid: String,
    /// Personal account kyc level. LEVEL_DEFAULT, LEVEL_1， LEVEL_2
    pub kyc_level: String,
    /// Personal account kyc region
    pub kyc_region: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct APIKeyPermissions {
    /// Permission of contract trade Order, Position
    pub contract_trade: Vec<String>,
    /// Permission of spot SpotTrade
    pub spot: Vec<String>,
    /// Permission of wallet AccountTransfer, SubMemberTransfer(master account), SubMemberTransferList(sub account), Withdraw(master account)
    pub wallet: Vec<String>,
    /// Permission of USDC Contract. It supports trade option and USDC perpetual. OptionsTrade
    pub options: Vec<String>,
    /// Unified account has this permission by default DerivativesTrade
    /// For classic account, it is always []
    pub derivatives: Vec<String>,
    /// Permission of convert ExchangeHistory
    pub exchange: Vec<String>,
    /// Permission of earn product Earn
    #[serde(default)]
    pub earn: Vec<String>,
    /// Permission of blocktrade. Not applicable to subaccount, always []
    pub block_trade: Vec<String>,
    /// Permission of Affiliate. Only affiliate can have this permission, otherwise always []
    pub affiliate: Vec<String>,
    /// Always [] as Master Trader account just use ContractTrade to start CopyTrading
    pub copy_trading: Vec<String>,
}
