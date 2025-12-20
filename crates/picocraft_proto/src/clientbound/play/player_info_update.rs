use crate::prelude::*;

#[derive(Debug)]
pub struct PlayerInfoUpdatePacket<const ACTIONS: usize> {
    pub actions: EnumSet,
    pub players: PrefixedArray<(UUID, Array<PlayerActions, ACTIONS>), 8>,
}

impl<const ACTIONS: usize> Packet for PlayerInfoUpdatePacket<ACTIONS> {
    const ID: VarInt = VarInt(0x44);

    const STATE: State = State::Play;
}

impl<const ACTIONS: usize> Encode for PlayerInfoUpdatePacket<ACTIONS> {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        Self::ID.encode(&mut buffer).await?;
        self.actions.encode(&mut buffer).await?;
        self.players.encode(&mut buffer).await
    }
}

impl<const ACTIONS: usize> core::fmt::Display for PlayerInfoUpdatePacket<ACTIONS> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PlayerInfoUpdatePacket (id: {})", Self::ID)
    }
}

impl<const ACTIONS: usize> Decode for PlayerInfoUpdatePacket<ACTIONS> {
    async fn decode<R: embedded_io_async::Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let actions = EnumSet::decode(&mut buffer).await?;

        let mut action_bitset = actions.0;

        let array_length = *VarInt::decode(&mut buffer).await?;

        if array_length > 8 {
            log::warn!(
                "Decoded array length of PlayerInfoUpdatePacket {array_length} exceeds maximum \
                 size of 8."
            );
            return Err(DecodeError::VarIntTooBig);
        }

        let actions_length = action_bitset.count_ones();

        if actions_length > ACTIONS as u32 {
            log::warn!(
                "Decoded actions length of PlayerInfoUpdatePacket {actions_length} exceeds \
                 maximum size of {ACTIONS}."
            );
            return Err(DecodeError::VarIntTooBig);
        }

        let mut players = PrefixedArray::new();

        for _ in 0..array_length {
            players
                .push({
                    let uuid = UUID::decode(&mut buffer).await?;

                    let mut player_actions = Array::<PlayerActions, ACTIONS>::new();

                    for _ in 0..actions_length {
                        player_actions
                            .push(if (action_bitset & 0x01) != 0 {
                                action_bitset -= 0x01;
                                PlayerActions::AddPlayer {
                                    name: String::<16>::decode(&mut buffer).await?,
                                    properties: Properties::decode(&mut buffer).await?,
                                }
                            } else if (action_bitset & 0x02) != 0 {
                                action_bitset -= 0x02;
                                PlayerActions::InitialiseChat(
                                    PrefixedOptional::<InitialiseChatData>::decode(&mut buffer)
                                        .await?,
                                )
                            } else if (action_bitset & 0x04) != 0 {
                                action_bitset -= 0x04;
                                PlayerActions::UpdateGameMode(VarInt::decode(&mut buffer).await?)
                            } else if (action_bitset & 0x08) != 0 {
                                action_bitset -= 0x08;
                                PlayerActions::UpdateListed(Boolean::decode(&mut buffer).await?)
                            } else if (action_bitset & 0x10) != 0 {
                                action_bitset -= 0x10;
                                PlayerActions::UpdateLatency(VarInt::decode(&mut buffer).await?)
                            } else if (action_bitset & 0x20) != 0 {
                                action_bitset -= 0x20;
                                PlayerActions::UpdateDisplayName(
                                    PrefixedOptional::<TextComponent>::decode(&mut buffer).await?,
                                )
                            } else if (action_bitset & 0x40) != 0 {
                                action_bitset -= 0x40;
                                PlayerActions::UpdateListPriority(
                                    VarInt::decode(&mut buffer).await?,
                                )
                            } else if (action_bitset & 0x80) != 0 {
                                action_bitset -= 0x80;
                                PlayerActions::UpdateHat(Boolean::decode(&mut buffer).await?)
                            } else {
                                return Err(DecodeError::InvalidEnumValue);
                            })
                            .expect("already validated this length");
                    }

                    (uuid, player_actions)
                })
                .expect("already validated this length");
        }

        Ok(PlayerInfoUpdatePacket { actions, players })

        // let players = PrefixedArray::decode(&mut buffer).await?;
    }
}

#[derive(Debug)]
pub enum PlayerActions {
    AddPlayer {
        name: String<16>,
        properties: Properties,
    },
    InitialiseChat(PrefixedOptional<InitialiseChatData>),
    UpdateGameMode(VarInt),
    UpdateListed(Boolean),
    UpdateLatency(VarInt),
    UpdateDisplayName(PrefixedOptional<TextComponent>),
    UpdateListPriority(VarInt),
    UpdateHat(Boolean),
}

impl Encode for PlayerActions {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        match self {
            PlayerActions::AddPlayer { name, properties } => {
                name.encode(&mut buffer).await?;
                properties.encode(&mut buffer).await
            }
            PlayerActions::InitialiseChat(data) => data.encode(&mut buffer).await,
            PlayerActions::UpdateGameMode(mode) => mode.encode(&mut buffer).await,
            PlayerActions::UpdateListed(listed) => listed.encode(&mut buffer).await,
            PlayerActions::UpdateLatency(latency) => latency.encode(&mut buffer).await,
            PlayerActions::UpdateDisplayName(name) => name.encode(&mut buffer).await,
            PlayerActions::UpdateListPriority(priority) => priority.encode(&mut buffer).await,
            PlayerActions::UpdateHat(hat) => hat.encode(&mut buffer).await,
        }
    }
}

/// An empty struct representing data to initialise secure chat, as they use up
/// a lot of memory.
#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct InitialiseChatData;

/// An empty struct representing a TextComponent, as we cannot yet decode NBT
/// data.
#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct TextComponent;
