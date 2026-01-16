use crate::prelude::*;

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

pub struct Enchantment {
    type_id: VarInt,
    level: VarInt,
}

pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

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

pub struct PotionEffect {
    amplifier: VarInt,
    duration: VarInt,
    ambient: Boolean,
    show_particles: Boolean,
    show_icon: Boolean,
    ///Apparently this field is unused by the client entirely, so can just
    /// always be a false value.
    hidden_effect: False,
    // hidden_effect: PrefixedOptional<Detail>,
}
