use heapless::format;
use picocraft_proto::serverbound::*;

use crate::prelude::*;

impl HandlePacket for StatusRequestPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        // This should really be built with values from the server config for player
        // count
        let json = clientbound::JsonStatusResponse::builder()
            .players(SERVER_CONFIG.read().await.max_players, 0)
            // The clone here ideally shouldn't occur
            .description(SERVER_CONFIG.read().await.motd.clone())
            .build();

        let status_response = clientbound::StatusResponsePacket::<256>::builder()
            .json_response(
                format!(
                    "{{\"version\":{{\"name\":\"{}\",\"protocol\":{}}},\"players\":{{\"max\":{},\"\
                     online\":{},\"sample\":[]}},\"description\":{{\"text\":\"{}\"}},\"\
                     enforcesSecureChat\":{}}}",
                    json.version.name,
                    json.version.protocol,
                    json.players.max,
                    json.players.online,
                    json.description.text,
                    json.enforces_secure_chat
                )
                .unwrap(),
            )
            .build();

        trace!("Packet constructed: {:?}", &status_response);

        status_response
            .encode(&mut client.tx_buf)
            .await
            .inspect_err(|e| error!("{e:#?}"))
            .unwrap();

        VarInt(client.tx_buf.len() as i32)
            .encode(&mut client.socket)
            .await
            .unwrap();

        client.socket.write_all(&client.tx_buf).await.unwrap();

        client.socket.flush().await.unwrap();

        trace!("Status response sent.");

        Ok(())
    }
}

impl HandlePacket for PingRequestPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        let pong_response = clientbound::PongResponsePacket::builder()
            .timestamp(self.timestamp)
            .build();

        trace!("Packet constructed: {:?}", &pong_response);

        pong_response
            .encode(&mut client.tx_buf)
            .await
            .inspect_err(|e| error!("{e:#?}"))
            .unwrap();

        client.encode_packet_length(client.tx_buf.len()).await?;

        client.socket.write_all(&client.tx_buf).await.unwrap();

        trace!("Pong response sent.");

        client.socket.flush().await.unwrap();

        client.socket.shutdown().await?;

        info!(
            "Handled status request for client: {}",
            client.socket.socket.peer_addr().unwrap()
        );

        Err(PacketError::ConnectionClosed)
    }
}
