// use crate::trade::WebsocketResponse;
// use std::net::TcpStream;
// use tungstenite::stream::MaybeTlsStream;
//
// impl<R: serde::de::DeserializeOwned> WebsocketResponse<R>
//     for tungstenite::WebSocket<MaybeTlsStream<TcpStream>>
// {
//     fn read_stream_single(&mut self) -> BianResult<R> {
//         let msg = self
//             .read_message()
//             .map_err(|e| APIError::WSClientError(e.to_string()))?;
//         match msg {
//             tungstenite::Message::Text(text) => {
//                 let resp = serde_json::from_str(&text)
//                     .map_err(|e| APIError::DecodeError(e.to_string()))?;
//                 Ok(resp)
//             }
//             tungstenite::Message::Ping(_) => {
//                 let pong = tungstenite::Message::Pong(vec![]);
//                 self.write_message(pong)
//                     .map_err(|e| APIError::WSClientError(e.to_string()))?;
//                 self.read_stream_single()
//             }
//             _ => unreachable!(),
//         }
//     }
//
//     fn read_stream_multi(&mut self) -> BianResult<R> {
//         let msg = self
//             .read_message()
//             .map_err(|e| APIError::WSClientError(e.to_string()))?;
//         match msg {
//             tungstenite::Message::Text(text) => {
//                 let wrapped_resp: MultiResponse<R> = serde_json::from_str(&text)
//                     .map_err(|e| APIError::DecodeError(e.to_string()))?;
//                 Ok(wrapped_resp.data)
//             }
//             tungstenite::Message::Ping(_) => {
//                 let pong = tungstenite::Message::Pong(vec![]);
//                 self.write_message(pong)
//                     .map_err(|e| APIError::WSClientError(e.to_string()))?;
//                 self.read_stream_multi()
//             }
//             _ => unreachable!(),
//         }
//     }
//
//     fn close_stream(&mut self) {
//         self.close(None).unwrap();
//     }
// }
