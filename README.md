# headers-client-ip

![Build Status](https://github.com/dimitrmo/headers-client-ip/actions/workflows/ci.yml/badge.svg) ![crates.io](https://img.shields.io/crates/v/headers-client-ip.svg)

## Installation

Add this to your `Cargo.toml`

```toml
[dependencies]
headers-client-ip = "0.1.0"
```

## Use with axum

```rust
let app = Router::new().route("/ws", get(ws_handler));

async fn ws_handler(
    ws: WebSocketUpgrade,
    ip: Option<TypedHeader<XRealIP>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_ip)) = ip {
        println!("`{}` connected", user_ip);
    }

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    //
}
```
