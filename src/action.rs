#[derive(Debug, Clone, Copy)]
pub enum Action {
    Ackpoll,
    CancelDomain,
    ChangeOwnerDomain,
    CreateDomain,
    CreateHandle,
    DeleteHandle,
    GetAuthcodeDomain,
    InfoDnsRecords,
    InfoDnsZone,
    InfoDomain,
    InfoHandle,
    ListallDomains,
    ListallHandle,
    Login,
    Logout,
    Poll,
    PriceTopleveldomain,
    TransferDomain,
    UpdateDnsRecords,
    UpdateDnsZone,
    UpdateDomain,
    UpdateHandle,
}

impl Action {
    /// The action string to specify the wanted action
    ///
    /// <https://helpcenter.netcup.com/de/wiki/general/unsere-api/#anmerkungen-zu-json-requests>
    pub fn as_str(self) -> &'static str {
        match self {
            Action::Ackpoll => "ackpoll",
            Action::CancelDomain => "cancelDomain",
            Action::ChangeOwnerDomain => "changeOwnerDomain",
            Action::CreateDomain => "createDomain",
            Action::CreateHandle => "createHandle",
            Action::DeleteHandle => "deleteHandle",
            Action::GetAuthcodeDomain => "getAuthcodeDomain",
            Action::InfoDnsRecords => "infoDnsRecords",
            Action::InfoDnsZone => "infoDnsZone",
            Action::InfoDomain => "infoDomain",
            Action::InfoHandle => "infoHandle",
            Action::ListallDomains => "listallDomains",
            Action::ListallHandle => "listallHandle",
            Action::Login => "login",
            Action::Logout => "logout",
            Action::Poll => "poll",
            Action::PriceTopleveldomain => "priceTopleveldomain",
            Action::TransferDomain => "transferDomain",
            Action::UpdateDnsRecords => "updateDnsRecords",
            Action::UpdateDnsZone => "updateDnsZone",
            Action::UpdateDomain => "updateDomain",
            Action::UpdateHandle => "updateHandle",
        }
    }
}

/// Serialize the enum with its `as_str` representation.
impl serde::Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
