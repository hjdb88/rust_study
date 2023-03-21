use internal_files::internal_files_server::{InternalFiles, InternalFilesServer};
use internal_files::{
    DownloadRequest, DownloadResponse, ExistsRequest, ExistsResponse, FileHead, UploadRequest,
    UploadResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod internal_files {
    include!("proto/internalfiles.rs");
}

#[derive(Debug, Default)]
pub struct InternalFilesService {}

#[tonic::async_trait]
impl InternalFiles for InternalFilesService {
    async fn upload(
        &self,
        request: Request<UploadRequest>,
    ) -> Result<Response<UploadResponse>, Status> {
        println!("request upload: {:?}", request);

        let mut file_heads: Vec<FileHead> = Vec::new();
        let file_head = FileHead {
            success: false,
            hash: "hash".into(),
            name: "name".into(),
            size: 0,
        };
        file_heads.push(file_head);

        Ok(Response::new(UploadResponse { files: file_heads }))
    }

    async fn exists(
        &self,
        request: Request<ExistsRequest>,
    ) -> Result<Response<ExistsResponse>, Status> {
        println!("request upload: {:?}", request);

        Ok(Response::new(ExistsResponse { exists: true }))
    }

    async fn download(
        &self,
        request: Request<DownloadRequest>,
    ) -> Result<Response<DownloadResponse>, Status> {
        println!("request upload: {:?}", request);

        let mut data: Vec<u8> = Vec::new();
        data.push(1);
        data.push(2);
        data.push(3);
        data.push(4);
        data.push(5);
        data.push(6);
        Ok(Response::new(DownloadResponse { content: data }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8888".parse()?;
    let service = InternalFilesService::default();

    Server::builder()
        .add_service(InternalFilesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
