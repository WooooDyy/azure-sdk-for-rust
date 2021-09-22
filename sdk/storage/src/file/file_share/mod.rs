pub mod requests;
pub mod responses;


use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::AccessTier;
use http::{header, HeaderMap};
use azure_core::util::HeaderMapExt;
use azure_core::headers::{
    QUOTA,REMAINING_RETENTION_DAYS,META_PREFIX,ENABLED_PROTOCOLS,ROOT_SQUASH,
};
use azure_core::incompletevector::IncompleteVector;
use xml::{Element, Xml};
use crate::parsing_xml::{cast_must, cast_optional, traverse};
use http::request::Builder;
use azure_core::{AddAsHeader, Request, HTTPHeaderError};

create_enum!(
    EnabledProtocols,
    (None,"none"),
    (SMB,"SMB"),
    (NFS,"NFS")
);
impl AddAsHeader for EnabledProtocols{

    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            EnabledProtocols::SMB => builder.header(ENABLED_PROTOCOLS,"SMB"),
            EnabledProtocols::NFS => builder.header(ENABLED_PROTOCOLS,"NFS"),
            EnabledProtocols::None => builder,
        }
    }

    fn add_as_header2(&self, request: &mut Request) -> Result<(), HTTPHeaderError> {
        let (header_name, header_value) = match self {
            EnabledProtocols::SMB => (ENABLED_PROTOCOLS, "SMB"),
            EnabledProtocols::NFS => (ENABLED_PROTOCOLS, "NFS"),
            EnabledProtocols::None => return Ok(()),
        };
        request.headers_mut().append(header_name,
                                     http::header::HeaderValue::from_str(&header_value)?,
        );
        Ok(())
    }
}
create_enum!(
    RootSquash,
    (None,"none"),
    (NoRootSquash,"NoRootSquash"),
    (AllSquash,"AllSquash")
);
// TODO implement AddAsHeader for EnabledProtocols and RootSquash
impl AddAsHeader for RootSquash{
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            RootSquash::NoRootSquash => builder.header(ROOT_SQUASH,"NoRootSquash"),
            RootSquash::AllSquash => builder.header(ROOT_SQUASH,"AllSquash"),
            RootSquash::None => builder,
        }
    }
    fn add_as_header2(&self, request: &mut Request) -> Result<(), HTTPHeaderError> {
        let(header_name, header_value) = match self
        {
            RootSquash::NoRootSquash => (ROOT_SQUASH,"NoRootSquash"),
            RootSquash::AllSquash => (ROOT_SQUASH,"AllSquash"),
            RootSquash::None => return Ok(()),
        };
        request.headers_mut().append(
            header_name,
            http::header::HeaderValue::from_str(&header_value)?,
        );
        Ok(())
    }
}

#[derive(Debug,Clone)]
pub struct FileShare {
    pub name: String,
    pub snapshot: Option<DateTime<Utc>>,
    pub last_modified: DateTime<Utc>,
    pub e_tag: String,
    pub quota: u64,
    pub deleted_time: Option<DateTime<Utc>>,
    pub remaining_retention_days: u64,
    pub access_tier: Option<AccessTier>,
    pub access_tier_change_time: Option<DateTime<Utc>>,
    pub access_tier_transition_state: String,
    pub enabled_protocols: String,
    pub metadata: HashMap<String, String>,
}

impl AsRef<str> for FileShare {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl FileShare{
    pub fn new(name: &str)->FileShare{
        FileShare{
            name:name.to_owned(),
            snapshot: None,
            last_modified: Utc::now(),
            e_tag: "".to_owned(),
            quota: 0,
            deleted_time: None,
            remaining_retention_days: 0,
            access_tier: None,
            access_tier_change_time:None,
            access_tier_transition_state: "".to_owned(),
            enabled_protocols: "".to_owned(),
            metadata: HashMap::new(),
        }
    }

    pub(crate) fn from_response<NAME>(
        name: NAME,
        headers: &HeaderMap,
    ) -> Result<FileShare, crate::Error>
    where
        NAME: Into<String>,
    {
        // TODO: Retrieve the snapshot time from
        // the headers
        let snapshot = None;

        let last_modified = match headers.get(header::LAST_MODIFIED) {
            Some(last_modified) => last_modified.to_str()?,
            None => {
                static LM: header::HeaderName = header::LAST_MODIFIED;
                return Err(crate::Error::MissingHeaderError(LM.as_str().to_owned()));
            }
        };
        let last_modified = DateTime::parse_from_rfc2822(last_modified)?;
        let last_modified = DateTime::from_utc(last_modified.naive_utc(), Utc);

        let e_tag = match headers.get(header::ETAG) {
            Some(e_tag) => e_tag.to_str()?.to_owned(),
            None => {
                return Err(crate::Error::MissingHeaderError(
                    header::ETAG.as_str().to_owned(),
                ));
            }
        };
        // TODO: Retriebe the quota from the headers
        let quota = match headers.get_as_u64(QUOTA){
            Some(quota) => quota,
            None =>{
                return Err(crate::Error::MissingHeaderError(
                    QUOTA.to_owned(),
                ));
            }
        };

        // TODO
        let remaining_retention_days = 0;

        // let remaining_retention_days = match headers.get_as_u64(REMAINING_RETENTION_DAYS){
        //     Some(remaining_retention_days) => remaining_retention_days,
        //     None =>{
        //         return Err(crate::Error::MissingHeaderError(
        //             REMAINING_RETENTION_DAYS.to_owned(),
        //         ))
        //     }
        // };

        // TODO
        let access_tier = None;
        let access_tier_change_time = None;
        let access_tier_transition_state =  "".to_owned();
        let enabled_protocols =  "".to_owned();

        let mut metadata: HashMap<String, String> = HashMap::new();
        for (key, value) in headers {
            if key.as_str().starts_with(META_PREFIX) {
                metadata.insert(key.as_str().to_owned(), value.to_str()?.to_owned());
            }
        }
        Ok(FileShare{
            name:name.into(),
            snapshot,
            last_modified,
            e_tag,
            quota,
            deleted_time: None,                        // TODO
            remaining_retention_days,
            access_tier,
            access_tier_change_time,
            access_tier_transition_state,
            enabled_protocols,
            metadata,
        })

    }

    // TODO parse
    fn parse(elem: &Element) -> Result<FileShare, crate::Error> {
        let name = cast_must::<String>(elem,&["Name"])?;

        // TODO: Retrieve the snapshot time from
        // the headers
        let snapshot = None;

        let last_modified = cast_must::<DateTime<Utc>>(elem,&["Properties", "Last-Modified"])?;
        let e_tag = cast_must::<String>(elem, &["Properties", "Etag"])?;
        let quota =cast_must::<u64>(elem, &["Properties", "Quota"])?;
        // TODO
        let deleted_time = None;
        // TODO
        let remaining_retention_days = 0;
        // TODO
        let access_tier = None;
        // TODO
        let access_tier_change_time = None;
        // TODO
        let access_tier_transition_state = "".to_owned();
        // TODO
        let enabled_protocols = "".to_owned();

        let metadata = {
            let mut hm = HashMap::new();
            let metadata = traverse(elem, &["Metadata"], true)?;

            for m in metadata {
                for key in &m.children {
                    let elem = match key {
                        Xml::ElementNode(elem) => elem,
                        _ => {
                            return Err(crate::Error::UnexpectedXMLError(String::from(
                                "Metadata should contain an ElementNode",
                            )));
                        }
                    };

                    let key = elem.name.to_owned();

                    if elem.children.is_empty() {
                        return Err(crate::Error::UnexpectedXMLError(String::from(
                            "Metadata node should not be empty",
                        )));
                    }

                    let content = {
                        match elem.children[0] {
                            Xml::CharacterNode(ref content) => content.to_owned(),
                            _ => {
                                return Err(crate::Error::UnexpectedXMLError(String::from(
                                    "Metadata node should contain a CharacterNode with metadata value",
                                )));
                            }
                        }
                    };

                    hm.insert(key, content);
                }
            }

            hm
        };

        Ok(
            FileShare{
                name,
                snapshot,
                last_modified,
                e_tag,
                quota,
                deleted_time,
                remaining_retention_days,
                access_tier,
                access_tier_change_time,
                access_tier_transition_state,
                enabled_protocols,
                metadata
            }
        )
    }
}

pub(crate) fn incomplete_vector_from_fileshare_response(
    body: &str,
) -> Result<IncompleteVector<FileShare>,crate::Error>{
    let elem: Element = body.parse()?;

    let mut v = Vec::new();
    for fileshare in traverse(&elem, &["Shares", "Share"], true)? {
        v.push(FileShare::parse(fileshare)?);
    }

    let next_marker = match cast_optional::<String>(&elem, &["NextMarker"])? {
        Some(ref nm) if nm.is_empty() => None,
        Some(nm) => Some(nm.into()),
        None => None,
    };

    Ok(IncompleteVector::new(next_marker, v))
}
