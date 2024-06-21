use crossbeam::queue::SegQueue;
use dashmap::DashMap;
use futures::future::Future;
use futures::task::Poll;
use std::sync::Arc;
use std::task::Waker;
use std::thread;
use std::time::Duration;

use super::super::ws::msg::Response;
use actix::prelude::Recipient;
type Socket = Recipient<Response>;

use super::client::ClientResponse;
use super::server::ServerCommand;

// 实现了在ws通路上，服务器向客户端发送命令，并等待客户端回复反馈的机制
#[derive(Clone)]
pub struct WebSocketCommand {
    pub event_id: String,
    pub queue: Arc<DashMap<String, Option<ClientResponse>>>,
    pub wake_mgt: WakerManager,
}

impl WebSocketCommand {
    pub async fn send(
        socket: &Socket,
        command: &ServerCommand,
        queue: &Arc<DashMap<String, Option<ClientResponse>>>,
        wake_mgt: &WakerManager,
        timeout_seconds: u64,
    ) -> anyhow::Result<ClientResponse> {
        let event_id = command.event_id.clone();
        queue.insert(event_id.clone(), None);

        use serde::Serialize;
        let mut data = Vec::new();
        command
            .serialize(&mut rmp_serde::Serializer::new(&mut data))
            .unwrap();
        let command_resp = Response {
            status: true,
            binary: Some(data),
            ..Default::default()
        };
        let send_status = socket.send(command_resp).await;
        if send_status.is_err() {
            queue.remove(&event_id);
            return Err(anyhow::anyhow!(
                "send socket data to client with error: {:?}",
                send_status.err()
            ));
        }

        let client = WebSocketCommand {
            event_id: event_id.clone(),
            queue: queue.clone(),
            wake_mgt: wake_mgt.clone(),
        };

        let resp =
            tokio::time::timeout(tokio::time::Duration::from_secs(timeout_seconds), client).await;
        queue.remove(&event_id);
        match resp {
            Ok(resp) => return anyhow::Ok(resp),
            Err(_) => {
                return Err(anyhow::anyhow!("timeout for waiting for client response"));
            }
        }
    }
}

impl Future for WebSocketCommand {
    type Output = ClientResponse;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        let Self {
            event_id,
            queue,
            wake_mgt,
        } = &mut *self;

        if queue.contains_key(&*event_id) {
            let value = queue.get(&*event_id).unwrap();
            if value.is_some() {
                return Poll::Ready(value.clone().unwrap());
            }
        }
        wake_mgt.wakers.push(cx.waker().clone());
        return Poll::Pending;
    }
}

#[derive(Default, Clone)]
pub struct WakerManager {
    pub wakers: Arc<SegQueue<Waker>>,
}

impl WakerManager {
    pub fn start(&self, dur: Duration) {
        let copied_manager = self.clone();
        thread::spawn(move || {
            let manager = copied_manager;
            let dur = dur;
            loop {
                std::thread::sleep(dur);
                let queue_length = manager.wakers.len();
                for _i in 0..queue_length {
                    if let Some(item) = manager.wakers.pop() {
                        item.wake();
                    }
                }
            }
        });
    }
}
