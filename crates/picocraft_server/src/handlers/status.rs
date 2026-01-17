use heapless::format;
use picocraft_proto::serverbound::status::*;

use crate::prelude::*;

impl HandlePacket for StatusRequestPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        // let server_config = client.server_config().lock().await.borrow();

        // This should really be built with values from the server config for player
        // count
        let json = clientbound::JsonStatusResponse::builder()
            .players(client.server_config().read().await.max_players, 0)
            //TODO the clone here ideally shouldn't occur
            .description(client.server_config().read().await.motd.clone())
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
                .expect("this string should be less than or equal to 256 bytes"),
            )
            .build();

        trace!("Packet constructed: {:?}", &status_response);

        client.encode_packet(&status_response).await?;

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

        client.encode_packet(&pong_response).await?;

        debug!(
            "Handled status request for client: {}",
            client
                .socket
                .remote_endpoint()
                .expect("socket should be open")
        );

        Err(PacketError::ConnectionClosed)
    }
}
