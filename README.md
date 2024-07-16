# rust learn

> tcp Server demo!

```rust
use std::sync::Arc;

use tcp_server::tcp_server::TcpServer;

fn main() {
    let server = Arc::new(TcpServer::new("0.0.0.0".to_string(), 8080));
    server.start(4);
}
```