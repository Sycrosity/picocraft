use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum StructuredComponent {
    MaxStackSize(VarInt),
    MaxDamage(VarInt),
    Damage(VarInt),
    Unbreakable,
    CustomName(TextComponent),
    Rarity(Rarity),
    Enchantments(PrefixedArray<Enchantment, 8>),
    // AttributeModifiers,
    RepairCost(VarInt),
    IntangibleProjectile(NBT),
    Food {
        nutrition: VarInt,
        saturation: Float,
        can_always_eat: Boolean,
    },
    Consumable {
        consume_seconds: Float,
        animation: Animation,
        sound: IDor<16, SoundEvent>,
        has_consume_particles: Boolean,
        effects: PrefixedArray<ConsumeEffect, 1>,
    },
    // UseCooldown {

    //     seconds: Float,
    //     cooldown_group: PrefixedOptional<Identifier<16>>,
    // }
}

#[derive(Debug, Clone)]
pub struct Enchantment {
    pub type_id: VarInt,
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
