use azure_core::incompletevector::IncompleteVector;
use azure_core::RequestId;
use crate::file::directory::Directory;

#[derive(Debug,Clone)]
pub struct ListDirectoriesAndFilesResponse{
    pub incomplete_vector: IncompleteVector<Directory>,
    // pub incomplete_vector: IncompleteVector<File>,
    pub request_id: RequestId,
}
impl ListDirectoriesAndFilesResponse{
    pub fn is_complete(&self) -> bool { self.incomplete_vector.is_complete() }

}