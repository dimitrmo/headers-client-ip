# headers-client-ip

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
