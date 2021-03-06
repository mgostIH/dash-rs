//! Module containing request structs for retrieving profile/level comments

use crate::request::{BaseRequest, GD_21};
use serde::Serialize;
use std::fmt::{Display, Formatter};

/// The different orderings that can be requested for level comments
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(into = "u8")]
pub enum SortMode {
    /// Sort the comments by likes, in descending order
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `1` in the boomlings API
    Liked,

    /// Sort the comments from newest to oldest
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `0` in the boomlings API
    Recent,
}

impl Into<u8> for SortMode {
    fn into(self) -> u8 {
        match self {
            SortMode::Liked => 1,
            SortMode::Recent => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct LevelCommentsRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the boomlings API
    pub total: u32,

    /// The page of users to retrieve. The first page is page `0`
    ///
    /// ## GD Internals:
    /// This field is called `page` in the boomlings API
    pub page: u32,

    /// What to sort by comments by
    ///
    /// ## GD Internals:
    /// This field is called `mode` in the boomlings API.
    #[serde(rename = "mode")]
    pub sort_mode: SortMode,

    /// The id of the level to retrieve the comments of
    ///
    /// ## GD Internals:
    /// This field is called `levelID` in the boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,

    /// The amount of comments to retrieve. Note that while in-game this can only be set to 20 or 40
    /// (via the "load more comments option), the API accepts any value. So you can set it to
    /// something ridiculously high (like u32::MAX_VALUE) and retrieve all comments at once.
    ///
    /// ## GD Internals:
    /// This field is called `count` in the boomlings API
    #[serde(rename = "count")]
    pub limit: u32,
}

impl<'a> LevelCommentsRequest<'a> {
    const_setter!(total: u32);

    const_setter!(limit: u32);

    const_setter!(page: u32);

    pub const fn new(level: u64) -> Self {
        Self::with_base(GD_21, level)
    }

    pub const fn with_base(base: BaseRequest<'a>, level: u64) -> Self {
        LevelCommentsRequest {
            level_id: level,
            base,
            page: 0,
            total: 0,
            sort_mode: SortMode::Recent,
            limit: 20,
        }
    }

    pub const fn most_liked(mut self) -> Self {
        self.sort_mode = SortMode::Liked;
        self
    }

    pub const fn most_recent(mut self) -> Self {
        self.sort_mode = SortMode::Recent;
        self
    }
}

impl Display for LevelCommentsRequest<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "LevelCommentsRequest({})", self.level_id)
    }
}

impl<'a> Into<LevelCommentsRequest<'a>> for u64 {
    fn into(self) -> LevelCommentsRequest<'a> {
        LevelCommentsRequest::new(self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct ProfileCommentsRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the boomlings API
    pub total: u32,

    /// The page of users to retrieve. The first page is page `0`
    ///
    /// ## GD Internals:
    /// This field is called `page` in the boomlings API
    pub page: u32,

    /// The account id of the user to retrieve the comments of
    ///
    /// ## GD Internals:
    /// This field is called `accountID` in the boomlings API
    #[serde(rename = "accountID")]
    pub account_id: u64,
}

impl<'a> ProfileCommentsRequest<'a> {
    const_setter!(total: u32);

    const_setter!(page: u32);

    const_setter!(account_id: u64);

    pub const fn new(account: u64) -> Self {
        Self::with_base(GD_21, account)
    }

    pub const fn with_base(base: BaseRequest<'a>, account: u64) -> Self {
        ProfileCommentsRequest {
            account_id: account,
            base,
            page: 0,
            total: 0,
        }
    }
}

impl Display for ProfileCommentsRequest<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "AccountCommentsRequest({})", self.account_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        request::comment::{LevelCommentsRequest, ProfileCommentsRequest},
        serde::RequestSerializer,
    };
    use serde::Serialize;

    #[test]
    fn serialize_level_comments() {
        if let Err(err) = env_logger::builder().is_test(true).try_init() {
            // nothing to make the tests fail over
            eprintln!("Error setting up env_logger: {:?}", err)
        }

        let request = LevelCommentsRequest::new(1234).most_liked().page(2).limit(15);
        let mut output = Vec::new();

        let mut serializer = RequestSerializer::new(&mut output);

        request.serialize(&mut serializer).unwrap();

        assert_eq!(
            std::str::from_utf8(&output),
            Ok("gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&total=0&page=2&mode=1&levelID=1234&count=15")
        );
    }

    #[test]
    fn serialize_profile_comments() {
        if let Err(err) = env_logger::builder().is_test(true).try_init() {
            // nothing to make the tests fail over
            eprintln!("Error setting up env_logger: {:?}", err)
        }

        let request = ProfileCommentsRequest::new(1710032).page(2);
        let mut output = Vec::new();

        let mut serializer = RequestSerializer::new(&mut output);

        request.serialize(&mut serializer).unwrap();

        assert_eq!(
            std::str::from_utf8(&output),
            Ok("gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&total=0&page=2&accountID=1710032")
        );
    }
}
