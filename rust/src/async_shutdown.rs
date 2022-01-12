use std::sync::{Arc, Mutex};

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let actual = Arc::new(Mutex::new(Vec::new()));
            let shutdown = async_shutdown::Shutdown::new();

            tokio::spawn({
                let actual = actual.clone();
                let shutdown = shutdown.clone();
                async move {
                    if let Err(_) = tokio::signal::ctrl_c().await {
                        actual.lock().unwrap().push(-1);
                        println!("{:?}", actual.lock().unwrap());
                        std::process::exit(1);
                    } else {
                        actual.lock().unwrap().push(0);
                        shutdown.shutdown();
                    }
                }
            });
            let fut1 = tokio::spawn({
                let actual = actual.clone();
                let shutdown = shutdown.clone();
                shutdown.wrap_cancel(async move {
                    loop {
                        actual.lock().unwrap().push(1);
                        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    }
                })
            });
            let fut2 = tokio::spawn({
                let actual = actual.clone();
                let shutdown = shutdown.clone();
                shutdown.wrap_cancel(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    loop {
                        actual.lock().unwrap().push(2);
                        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    }
                })
            });
            fut1.await.unwrap();
            actual.lock().unwrap().push(11);
            fut2.await.unwrap();
            actual.lock().unwrap().push(12);
            shutdown.wait_shutdown_complete().await;
            actual.lock().unwrap().push(13);
            println!("{:?}", actual.lock().unwrap());
        })
}
