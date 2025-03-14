use common_lib::{
    error::BrownFoxError,
    worker::{WorkerCommand, WorkerRequest},
};
use futures_util::StreamExt;
use redis::{Client, Commands};

async fn worker_loop(client: Client) -> Result<(), BrownFoxError> {
    let mut con = client
        .get_connection()
        .map_err(|e| BrownFoxError::RedisError(e))?;

    let (mut sink, mut stream) = client
        .get_async_pubsub()
        .await
        .map_err(|e| BrownFoxError::RedisError(e))?
        .split();

    sink.subscribe("worker")
        .await
        .map_err(|e| BrownFoxError::RedisError(e))?;

    println!("listening to 'worker'");
    loop {
        let Some(msg) = stream.next().await else {
            continue;
        };

        let payload: String = msg
            .get_payload()
            .map_err(|e| BrownFoxError::RedisError(e))?;

        println!("[RCVD] channel {}: {}", msg.get_channel_name(), payload);

        let req: WorkerRequest =
            match serde_json::from_str(&payload).map_err(|e| BrownFoxError::JsonError(e)) {
                Ok(req) => req,
                Err(e) => {
                    println!("[ERR] {}", e);
                    continue;
                }
            };

        match req.cmd {
            WorkerCommand::Echo { msg } => {
                con.rpush::<_, _, ()>(&req.id, &msg)
                    .map_err(|e| BrownFoxError::RedisError(e))?;
            }
            WorkerCommand::Stop => break,
        }
    }

    println!("worker stopped");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), BrownFoxError> {
    let client =
        redis::Client::open("redis://127.0.0.1").map_err(|e| BrownFoxError::RedisError(e))?;
    worker_loop(client).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use common_lib::worker::{WorkerCommand, WorkerRequest};
    use redis::Commands;

    use crate::worker_loop;

    #[tokio::test]
    async fn test_synchronous() {
        let client = redis::Client::open("redis://127.0.0.1").unwrap();
        let mut con: redis::Connection = client.get_connection().unwrap();

        let w = tokio::task::spawn(async {
            println!("starting worker loop");
            worker_loop(client).await.unwrap();
        });

        // sleep so the worker has some time to start
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        con.publish::<_, _, String>(
            "worker",
            &serde_json::to_string(&WorkerRequest {
                id: 12345,
                cmd: WorkerCommand::Echo {
                    msg: "this is a test".to_string(),
                },
            })
            .unwrap(),
        )
        .unwrap();

        con.publish::<_, _, String>(
            "worker",
            &serde_json::to_string(&WorkerRequest {
                id: 6789,
                cmd: WorkerCommand::Stop,
            })
            .unwrap(),
        )
        .unwrap();

        let _ = w.await.unwrap();

        let rsp: String = con.rpop(12345, None).unwrap();
        assert_eq!(rsp, "this is a test");
    }
}
