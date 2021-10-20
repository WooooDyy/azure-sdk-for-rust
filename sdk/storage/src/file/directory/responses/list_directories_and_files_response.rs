use azure_core::incompletevector::IncompleteVector;
use azure_core::RequestId;
use crate::file::directory::Directory;
use crate::file::file::File;

#[derive(Debug,Clone)]
pub struct ListDirectoriesAndFilesResponse{
    pub incomplete_vector_directory: IncompleteVector<Directory>,
    pub incomplete_vector_file: IncompleteVector<File>,
    pub request_id: RequestId,
}
impl ListDirectoriesAndFilesResponse{
    pub fn is_complete(&self) -> bool { self.incomplete_vector_directory.is_complete() && self.incomplete_vector_file.is_complete() }

}