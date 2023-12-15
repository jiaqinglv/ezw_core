use std::net::SocketAddr;

use axum::Router;

use super::Server;

pub struct AxumServer {
    // server: Option<Builder<hyper::AddrIncoming, Exec>>
    // pub server: Option<hyper::server::Builder<hyper::server::conn::AddrIncoming>>,
    addr: SocketAddr
}

impl Server for AxumServer {
    fn bind(addr: SocketAddr) -> Result<AxumServer, Box<dyn std::error::Error>> {
        Ok(AxumServer {
            addr,
        })
    }
}

impl AxumServer {
    // 服务器监听运行
    pub async fn listen(self, router: Router) -> Result<(), Box<dyn std::error::Error>> {
        let listener = tokio::net::TcpListener::bind(self.addr).await.unwrap();

        axum::serve(listener, router).await?;

        Ok(())
    }
}
