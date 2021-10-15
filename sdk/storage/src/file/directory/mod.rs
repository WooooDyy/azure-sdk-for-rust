use azure_core::prelude::{FilePermissionKey, FileAttributes};
use chrono::{DateTime, Utc};
use http::{HeaderMap, header};
use azure_core::headers::{FILE_PERMISSION_KEY, FILE_ATTRIBUTES, FILE_CREATION_TIME, FILE_LAST_WRITE_TIME, FILE_CHANGE_TIME, FILE_ID, FILE_PARENT_ID};

pub mod requests;
pub mod responses;
pub mod prelude;


#[derive(Debug,Clone)]
pub struct Directory {
    pub name: String,
    pub last_modified: DateTime<Utc>,
    pub e_tag: String,
    pub file_permission_key: String,
    pub file_attributes: String,
    pub file_creation_time:DateTime<Utc>,
    pub file_last_write_time: DateTime<Utc>,
    pub file_change_time: DateTime<Utc>,
    pub file_id: String,
    pub file_parent_id: String,
}

impl AsRef<str> for Directory{
    fn as_ref(&self) -> &str {
        &self.name
    }
}
impl Directory{
    pub fn new(name: &str) -> Directory{
        Directory{
            name:name.to_owned(),
            last_modified: Utc::now(),
            e_tag: "".to_owned(),
            file_permission_key: "".to_owned(),
            file_attributes: "".to_owned(),
            file_creation_time:Utc::now(),
            file_last_write_time: Utc::now(),
            file_change_time: Utc::now(),
            file_id: "".to_owned(),
            file_parent_id: "".to_owned(),
        }
    }
    pub(crate) fn from_response<NAME>(
        name: NAME,
        headers: &HeaderMap,
    ) -> Result<Directory,crate::Error>
    where
        NAME: Into<String>,
    {
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


        let file_permission_key = match headers.get(FILE_PERMISSION_KEY){
            Some(file_permission_key) => file_permission_key.to_str().unwrap().to_string(),
            None => return Err(crate::Error::MissingHeaderError(FILE_PERMISSION_KEY.to_owned())),
        };
        // let file_permission_key = FilePermissionKey::from(file_permission_key);


        let file_attributes = match headers.get(FILE_ATTRIBUTES){
            Some(file_attributes) => file_attributes.to_str().unwrap().to_string(),
            None => return Err(crate::Error::MissingHeaderError(FILE_ATTRIBUTES.to_owned())),
        };
        // let file_attributes = FileAttributes::from(file_attributes);


        let file_creation_time = match headers.get(FILE_CREATION_TIME) {
            Some(file_creation_time) => file_creation_time.to_str()?,
            None => {
                return Err(crate::Error::MissingHeaderError(FILE_CREATION_TIME.to_owned()));
            }
        };
        let file_creation_time = DateTime::parse_from_rfc3339(file_creation_time)?;
        let file_creation_time = DateTime::from_utc(file_creation_time.naive_utc(), Utc);


        let file_last_write_time = match headers.get(FILE_LAST_WRITE_TIME) {
            Some(file_last_write_time) => file_last_write_time.to_str()?,
            None => {
                return Err(crate::Error::MissingHeaderError(FILE_LAST_WRITE_TIME.to_owned()));
            }
        };
        let file_last_write_time = DateTime::parse_from_rfc3339(file_last_write_time)?;
        let file_last_write_time = DateTime::from_utc(file_last_write_time.naive_utc(), Utc);


        let file_change_time = match headers.get(FILE_CHANGE_TIME) {
            Some(file_change_time) => file_change_time.to_str()?,
            None => {
                return Err(crate::Error::MissingHeaderError(FILE_CHANGE_TIME.to_owned()));
            }
        };
        let file_change_time = DateTime::parse_from_rfc3339(file_change_time)?;
        let file_change_time = DateTime::from_utc(file_change_time.naive_utc(), Utc);


        let file_id = match headers.get(FILE_ID){
            Some(file_id) => file_id.to_str().unwrap().to_string(),
            None => return Err(crate::Error::MissingHeaderError(FILE_ID.to_owned())),
        };


        let file_parent_id = match headers.get(FILE_PARENT_ID){
            Some(file_parent_id) => file_parent_id.to_str().unwrap().to_string(),
            None => return Err(crate::Error::MissingHeaderError(FILE_PARENT_ID.to_owned())),
        };

        Ok(Directory{
            name:name.into(),
            last_modified,
            e_tag,
            file_permission_key,
            file_attributes,
            file_creation_time,
            file_last_write_time,
            file_change_time,
            file_id,
            file_parent_id,
        })
    }
}

