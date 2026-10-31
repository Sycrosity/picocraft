use crate::prelude::*;

impl Encode for Slot {
    //TODO - needs validation for number of components
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        self.item_count.encode(&mut buffer).await?;
        if *self.item_count == 0 {
            return Ok(());
        }

        self.item_id.encode(&mut buffer).await?;

        let components_to_add = self
            .number_of_components_to_add
            .ok_or(EncodeError::UnsupportedOperation)?;
        let components_to_remove = self
            .number_of_components_to_remove
            .ok_or(EncodeError::UnsupportedOperation)?;

        if *components_to_add != 0 || *components_to_remove != 0 {
            return Err(EncodeError::UnsupportedOperation);
        }

        self.number_of_components_to_add.encode(&mut buffer).await?;
        self.number_of_components_to_remove
            .encode(&mut buffer)
            .await?;

        // self.components_to_add.encode(&mut buffer).await?;
        // self.components_to_remove.encode(&mut buffer).await?;

        Ok(())
    }
}

impl Decode for Slot {
    async fn decode<R: embedded_io_async::Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let item_count = VarInt::decode(&mut buffer).await?;
        if *item_count == 0 {
            return Ok(Slot {
                item_count,
                item_id: None,
                number_of_components_to_add: None,
                number_of_components_to_remove: None,
            });
        }

        let item_id = VarInt::decode(&mut buffer).await?;
        let number_of_components_to_add = VarInt::decode(&mut buffer).await?;
        let number_of_components_to_remove = VarInt::decode(&mut buffer).await?;

        Ok(Slot {
            item_count,
            item_id: Some(item_id),
            number_of_components_to_add: Some(number_of_components_to_add),
            number_of_components_to_remove: Some(number_of_components_to_remove),
        })
    }
}

#[derive(Debug, Clone)]
pub enum StructuredComponent {
    MaxStackSize(VarInt),
    MaxDamage(VarInt),
    Damage(VarInt),
    Unbreakable,
    CustomName(TextComponent),
    Rarity(Rarity),
    //TODO - should be optimised given that we don't need infinite possible enchants - disabled for now Enchantments(PrefixedArray<Enchantment, 4>),
    RepairCost(VarInt),
    IntangibleProjectile(NBT),
    Food {
        nutrition: VarInt,
        saturation: Float,
        can_always_eat: Boolean,
    },
    //TODO - This enum option is 120 bytes, drastically inflating the size of a slot. A different solution or realistically not having non food consumables is necessary.
    // Consumable {
    //     consume_seconds: Float,
    //     animation: Animation,
    //     sound: IDor<16, SoundEvent>,
    //     has_consume_particles: Boolean,
    //     effects: PrefixedArray<ConsumeEffect, 1>,
    // },

    //TODO - implementation required
    // UseCooldown {

    //     seconds: Float,
    //     cooldown_group: PrefixedOptional<Identifier<16>>,
    // }
    // AttributeModifiers,
}

/// Realistically the list of enchantments would be as such:
/// + Unbreaking
/// + Sharpness
/// + Silk touch
/// + Fortune
/// + Efficiency
/// + Protection
/// + depth strider
/// + feather falling
/// + looting
/// + knockback?
/// + infinity
/// + power
/// + punch?
/// + respiration
/// + sweeping edge
/// + flame?
/// + fire aspect?
///
/// from this, the max enchants on one item is likely 5/6.
#[derive(Debug, Clone)]
pub struct Enchantment {
    pub type_id: VarInt,
    // could easily just be a u8.
    pub level: VarInt,
}

#[derive(Debug, Clone)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

#[derive(Debug, Clone)]
pub enum Animation {
    None,
    Eat,
    Drink,
    Block,
    Bow,
    Spear,
    Crossbow,
    Spyglass,
    TootHorn,
    Brush,
}

//very much a stretch goal - unlikely to be used
#[derive(Debug, Clone)]
pub enum ConsumeEffect {
    ApplyEffects {
        effects: PrefixedArray<PotionEffect, 1>,
        probability: Float,
    },
    // what would this be used for?
    RemoveEffects(IDSet<16, 1>),
    ClearAllEffects,
    TeleportRandomly(Float),
    PlaySound(SoundEvent),
}

//very much a stretch goal - unlikely to be used
#[derive(Debug, Clone)]
pub struct PotionEffect {
    pub amplifier: VarInt,
    pub duration: VarInt,
    pub ambient: Boolean,
    pub show_particles: Boolean,
    pub show_icon: Boolean,
    ///Apparently this field is unused by the client entirely, so can just
    /// always be a false value.
    pub hidden_effect: bool,
    // hidden_effect: PrefixedOptional<Detail>,
}
