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

impl<const ACTIONS: usize> core::fmt::Display for PlayerInfoUpdatePacket<ACTIONS> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PlayerInfoUpdatePacket (id: {})", Self::ID)
    }
}

impl<const ACTIONS: usize> Encode for PlayerInfoUpdatePacket<ACTIONS> {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        Self::ID.encode(&mut buffer).await?;
        self.actions.encode(&mut buffer).await?;
        self.players.encode(&mut buffer).await
    }
}

impl<const ACTIONS: usize> Decode for PlayerInfoUpdatePacket<ACTIONS> {
    async fn decode<R: embedded_io_async::Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let actions = EnumSet::decode(&mut buffer).await?;

        let array_length = *VarInt::decode(&mut buffer).await?;

        if array_length > 8 {
            log::warn!(
                "Decoded array length of PlayerInfoUpdatePacket {array_length} exceeds maximum \
                 size of 8."
            );
            return Err(DecodeError::VarIntTooBig);
        }

        let actions_length = actions.bits().count_ones();

        if actions_length > ACTIONS as u32 {
            log::warn!(
                "Decoded actions length of PlayerInfoUpdatePacket {actions_length} exceeds \
                 maximum size of {ACTIONS}."
            );
            return Err(DecodeError::VarIntTooBig);
        }

        let mut players = PrefixedArray::new();

        for _ in 0..array_length {
            let uuid = UUID::decode(&mut buffer).await?;

            let player_actions = decode_player_actions(&actions, &mut buffer).await?;
            players
                .push((uuid, player_actions))
                .expect("already validated this length");
        }

        Ok(PlayerInfoUpdatePacket { actions, players })

        // let players = PrefixedArray::decode(&mut buffer).await?;
    }
}

async fn decode_player_actions<const ACTIONS: usize, R: embedded_io_async::Read>(
    actions: &EnumSet,
    mut buffer: R,
) -> Result<Array<PlayerActions, ACTIONS>, DecodeError> {
    let mut player_actions = Array::new();

    for flag in actions.iter() {
        let action = match flag {
            EnumSet::ADD_PLAYER => PlayerActions::AddPlayer {
                name: String::<16>::decode(&mut buffer).await?,
                properties: Properties::decode(&mut buffer).await?,
            },
            EnumSet::INITIALISE_CHAT => PlayerActions::InitialiseChat(
                PrefixedOptional::<InitialiseChatData>::decode(&mut buffer).await?,
            ),
            EnumSet::UPDATE_GAME_MODE => {
                PlayerActions::UpdateGameMode(VarInt::decode(&mut buffer).await?)
            }
            EnumSet::UPDATE_LISTED => {
                PlayerActions::UpdateListed(Boolean::decode(&mut buffer).await?)
            }
            EnumSet::UPDATE_LATENCY => {
                PlayerActions::UpdateLatency(VarInt::decode(&mut buffer).await?)
            }
            EnumSet::UPDATE_DISPLAY_NAME => PlayerActions::UpdateDisplayName(
                PrefixedOptional::<TextComponent>::decode(&mut buffer).await?,
            ),
            EnumSet::UPDATE_LIST_PRIORITY => {
                PlayerActions::UpdateListPriority(VarInt::decode(&mut buffer).await?)
            }
            EnumSet::UPDATE_HAT => PlayerActions::UpdateHat(Boolean::decode(&mut buffer).await?),
            bits => {
                return Err(DecodeError::InvalidEnumSetBits(bits.bits()));
            }
        };

        player_actions
            .push(action)
            .expect("already validated this length");
    }

    Ok(player_actions)
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

/// An empty struct representing a `TextComponent`, as we cannot yet decode NBT
/// data.
#[derive(Debug, Clone, Default, Encode, Decode)]
pub struct TextComponent;
