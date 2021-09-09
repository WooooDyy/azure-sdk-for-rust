
use azure_core::incompletevector::IncompleteVector;
use azure_core::RequestId;
use crate::file::file_share::FileShare;


#[derive(Debug,Clone)]
pub struct ListFileShareResponse{
    pub incomplete_vector: IncompleteVector<FileShare>,
    pub request_id: RequestId,
}
impl ListFileShareResponse{
    pub fn is_complete(&self) -> bool { self.incomplete_vector.is_complete() }
}
