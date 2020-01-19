# Actori net [![Build Status](https://travis-ci.org/actori/actori-net.svg?branch=master)](https://travis-ci.org/actori/actori-net) [![codecov](https://codecov.io/gh/actori/actori-net/branch/master/graph/badge.svg)](https://codecov.io/gh/actori/actori-net) [![Join the chat at https://gitter.im/actori/actori](https://badges.gitter.im/actori/actori.svg)](https://gitter.im/actori/actori?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Actori net - framework for composable network services

## Documentation & community resources

* [Chat on gitter](https://gitter.im/actori/actori)
* Minimum supported Rust version: 1.39 or later

## Example

```rust
fn main() -> io::Result<()> {
    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("./examples/key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("./examples/cert.pem").unwrap();
    let acceptor = builder.build();

    let num = Arc::new(AtomicUsize::new(0));

    // bind socket address and start workers. By default server uses number of
    // available logical cpu as threads count. actori net start separate
    // instances of service pipeline in each worker.
    Server::build()
        .bind(
            // configure service pipeline
            "basic", "0.0.0.0:8443",
            move || {
                let num = num.clone();
                let acceptor = acceptor.clone();

                // construct transformation pipeline
                pipeline(
                    // service for converting incoming TcpStream to a SslStream<TcpStream>
                    fn_service(move |stream: actix_rt::net::TcpStream| async move {
                        SslAcceptorExt::accept_async(&acceptor, stream.into_parts().0).await
                            .map_err(|e| println!("Openssl error: {}", e))
                    }))
                // .and_then() combinator chains result of previos service call to argument
                /// for next service calll. in this case, on success we chain
                /// ssl stream to the `logger` service.
                .and_then(fn_service(logger))
                // Next service counts number of connections
                .and_then(move |_| {
                    let num = num.fetch_add(1, Ordering::Relaxed);
                    println!("got ssl connection {:?}", num);
                    future::ok(())
                })
            },
        )?
        .run()
}
```

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## Code of Conduct

Contribution to the actori-net crate is organized under the terms of the
Contributor Covenant, the maintainer of actori-net, @fafhrd91, promises to
intervene to uphold that code of conduct.
