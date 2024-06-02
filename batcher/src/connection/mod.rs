// use std::{future, sync::Arc};

// use futures_util::stream::{SplitSink, SplitStream};
// use futures_util::SinkExt;
// use futures_util::TryStreamExt;
// use log::info;
// use tokio::{
//     net::TcpStream,
//     sync::{broadcast::Receiver, Mutex},
// };
// use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

// use crate::Batcher;

// /// A struct representing a websocket connection state.
// ///
// /// `rx` is the receiver part of a broadcast channel. It listens for messages to be
// /// responded to the connected clients. It is set to None until some data was included to the
// /// batch. It is reseted to None once the batch merkle root information is sent to the
// /// connected clients.
// /// `received_msgs` manages the count of received messages from this connection.
// /// `responded_msgs` manages the count of data added to a batch, what will mean that
// /// the message was responded.
// pub(crate) struct ConnectionState {
//     pub(crate) rx: Mutex<Option<Receiver<Message>>>,
//     pub(crate) received_msgs: Mutex<usize>,
//     pub(crate) responded_msgs: Mutex<usize>,
// }

// impl ConnectionState {
//     /// Creates a new ConnectionState with an empty state.
//     pub(crate) fn new() -> Self {
//         Self {
//             rx: Mutex::new(None),
//             received_msgs: Mutex::new(0),
//             responded_msgs: Mutex::new(0),
//         }
//     }

//     /// Updates the received message count of the connection by one
//     pub(crate) async fn update_received_msg_count(&self) {
//         *self.received_msgs.lock().await += 1
//     }

//     /// Updates the responded message count of the connection by one
//     pub(crate) async fn update_responded_msg_count(&self) {
//         *self.responded_msgs.lock().await += 1
//     }

//     /// When some data is added to the current batch, the websocket connection is
//     /// subscribed to receive data from the finalized batch, such like the merkle root.
//     /// If some data was already inserted in the current batch by this connection, then
//     /// there is no need to re-subscribe.
//     pub(crate) async fn maybe_subscribe(&self, batcher_state: Arc<Batcher>) {
//         // The only entity that can lock `rx` is the Future returned by the `send_response`
//         // method. If it is locked, then it means that it is already subscribed, so no need
//         // to do anything.
//         if let Ok(mut rx) = self.rx.try_lock() {
//             if rx.is_none() {
//                 *rx = Some(batcher_state.broadcast_tx.lock().await.subscribe())
//             }
//         }
//     }

//     /// Checks if all messages from this connection have been responded
//     pub(crate) async fn all_messages_responded(&self) -> bool {
//         *self.responded_msgs.lock().await == *self.received_msgs.lock().await
//     }

//     /// Method for awaiting for new messages from the connection and process them.
//     pub(crate) async fn process_new_messages(
//         &self,
//         batcher_state: Arc<Batcher>,
//         incoming: SplitStream<WebSocketStream<TcpStream>>,
//     ) {
//         incoming
//             .try_filter(|msg| future::ready(msg.is_text()))
//             .try_for_each(|msg| batcher_state.clone().handle_message(msg, self))
//             .await
//             .unwrap();
//     }

//     /// Method for awaiting processed batch information and forward it to the
//     /// connected client
//     pub(crate) async fn send_response(
//         &self,
//         outgoing: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
//     ) {
//         loop {
//             let mut rx_lock = self.rx.lock().await;
//             if let Some(rx) = &mut (*rx_lock) {
//                 let msg = rx.recv().await.unwrap();

//                 info!("Sending response...");
//                 outgoing.send(msg).await.unwrap();

//                 // reset the receiver state of the connection so that
//                 // it can subscribe to new batch information.
//                 *rx_lock = None;

//                 // if all messages have not been responded, this means
//                 // that the connection should not be closed, since there
//                 // will be more responses from other processed batches where
//                 // the client included data
//                 if self.all_messages_responded().await {
//                     outgoing.close().await.unwrap();
//                 }
//             }
//         }
//     }
// }
