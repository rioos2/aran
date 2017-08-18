// Copyright (c) 2017 RioCorp Inc.


pub trait Pageable {
    fn get_range(&self) -> [u64; 2];

    fn limit(&self) -> i64 {
        (self.get_range()[1] - self.get_range()[0] + 1) as i64
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct AccountInvitationListResponse {
    // message fields
    account_id: u64,
    invitations: Vec<OriginInvitation>,
}


impl AccountInvitationListResponse {
    pub fn new() -> AccountInvitationListResponse {
        ::std::default::Default::default()
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct OriginCreate {
    // message fields
    name: ::std::string::String,
    owner_id: u64,
    owner_name: String,
}

impl OriginCreate {
    pub fn new() -> OriginCreate {
        ::std::default::Default::default()
    }
}


#[derive(PartialEq, Clone, Default)]
pub struct Origin {
    id: u64,
    name: ::std::string::String,
    owner_id: u64,
    private_key_name: String,
}


impl Origin {
    pub fn new() -> Origin {
        ::std::default::Default::default()
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct OriginInvitation {
    id: u64,
    account_id: u64,
    account_name: String,
    origin_id: u64,
    origin_name: String,
    owner_id: u64,
}

impl OriginInvitation {
    pub fn new() -> OriginInvitation {
        ::std::default::Default::default()
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct OriginChannel {
    // message fields
    id: u64,
    origin_id: u64,
    name: String,
    owner_id: u64,
}

impl OriginChannel {
    pub fn new() -> OriginChannel {
        ::std::default::Default::default()
    }
}


#[derive(PartialEq, Clone, Default)]
pub struct OriginChannelIdent {
    origin: String,
    name: String,
}

impl OriginChannelIdent {
    pub fn new() -> OriginChannelIdent {
        ::std::default::Default::default()
    }
}


#[derive(PartialEq, Clone, Default)]
pub struct OriginChannelListResponse {
    origin_id: u64,
    channels: Vec<OriginChannel>,
}

impl OriginChannelListResponse {
    pub fn new() -> OriginChannelListResponse {
        ::std::default::Default::default()
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct OriginChannelPackageListRequest {
    name: String,
    start: u64,
    stop: u64,
}

impl OriginChannelPackageListRequest {
    pub fn new() -> OriginChannelPackageListRequest {
        ::std::default::Default::default()
    }

    pub fn set_start(&mut self, v: u64) {
        self.start = v;
    }
    pub fn get_start(&self) -> u64 {
        self.start
    }

    pub fn set_stop(&mut self, v: u64) {
        self.stop = v;
    }
    pub fn get_stop(&self) -> u64 {
        self.stop
    }
}

impl Pageable for OriginChannelPackageListRequest {
    fn get_range(&self) -> [u64; 2] {
        [self.get_start(), self.get_stop()]
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct OriginInvitationListResponse {
    origin_id: u64,
    invitations: Vec<OriginInvitation>,
}

impl OriginInvitationListResponse {
    pub fn new() -> OriginInvitationListResponse {
        ::std::default::Default::default()
    }
}
