//! Copilot Instructions for Rust (OpenIAP Client)

## Coding Conventions

- Use `snake_case` for function and variable names.
- Prefer `info!()`, `warn!()`, `error!()`, `debug!()`, `trace!()` from the `tracing` crate for logging.
- All API responses should be pattern matched using `Result<T, E>`.
- Use `tokio::spawn` for background tasks.

---

## Initialization

```rust
let client = Client::new();
client.on_event(Box::new(|event| match event {
    ClientEvent::Connecting => println!("Connecting..."),
    ClientEvent::Connected => println!("Connected"),
    ClientEvent::SignedIn => println!("Signed in"),
    ClientEvent::Disconnected(e) => println!("Disconnected: {:?}", e),
})).await;
client.connect_async("").await?;
```

---

## Observability

```rust
set_f64_observable_gauge("cpu_usage", 42.0, "CPU usage").unwrap();
set_u64_observable_gauge("users_online", 100, "Users online").unwrap();
set_i64_observable_gauge("error_count", -1, "Error count").unwrap();
disable_observable_gauge("cpu_usage");
```

---

## Authentication

```rust
let signin_result = client.signin(SigninRequest::with_userpass("guest", "password")).await;
```

---

## Database Operations

```rust
let request = QueryRequest::with_query("entities", "{}");
let response = client.query(request).await?;

let insert_one = InsertOneRequest::new("entities", "{\"name\":\"test\"}");
let insert_result = client.insert_one(insert_one).await?;

let insert_many = InsertManyRequest::new("entities", "[{\"name\":\"test1\"},{\"name\":\"test2\"}]");
let insert_many_result = client.insert_many(insert_many).await?;

let distinct = DistinctRequest {
    collectionname: "entities".into(),
    field: "_type".into(),
    ..Default::default()
};
let distinct_result = client.distinct(distinct).await?;
```

---

## File Transfer

```rust
client.upload(UploadRequest::filename("file.csv"), "file.csv").await?;
client.download(DownloadRequest::id("some_id"), None, None).await?;
```

---

## Work Items

```rust
let pop_result = client.pop_workitem(PopWorkitemRequest::bywiq("queue_name"), None).await?;

let update_result = client.update_workitem(UpdateWorkitemRequest {
    workitem: Some(mut_workitem),
    ..Default::default()
}).await?;

let push_result = client.push_workitem(PushWorkitemRequest {
    wiq: "queue_name".into(),
    name: "task_name".into(),
    ..Default::default()
}).await?;
```

---

## Messaging

```rust
client.queue_message(QueueMessageRequest::byqueuename("queue", "{\"key\":\"value\"}", true)).await?;

let rpc_result = client.rpc(QueueMessageRequest::byqueuename("queue", "{\"key\":\"value\"}", true), Duration::from_secs(2)).await?;
```

---

## Watches

```rust
let watch_id = client.watch(
    WatchRequest::new("entities", vec!["".into()]),
    Box::new(|event| println!("Watch: {:?}", event.document))
).await?;

client.unwatch(&watch_id).await?;
```

---

## Queues & Exchanges

```rust
let qname = client.register_queue(RegisterQueueRequest::byqueuename("queue"), Arc::new(|_, event| {
    println!("Queue msg: {:?}", event.data);
    Box::pin(async { Some("{\"reply\":\"ok\"}".into()) })
})).await?;

let exch = client.register_exchange(RegisterExchangeRequest::byexchangename("exchange"), Arc::new(|_, event| {
    println!("Exchange msg: {:?}", event.data);
    Box::pin(async { None })
})).await?;
```

---

## Custom Commands

```rust
let cmd = CustomCommandRequest {
    command: "getclients".into(),
    ..Default::default()
};
let response = client.custom_command(cmd, None).await?;
```

---

## Logging

Enable tracing early:

```rust
enable_tracing("openiap=info", "");
// or
enable_tracing("openiap=debug", "new");
```

Disable it later:

```rust
disable_tracing();
```

Log messages:

```rust
info!("This is an info message");
warn!("This is an warning");
error!("This is an error");
debug!("Debug messages");
trace!("tracing messages");
```