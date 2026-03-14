## Entity format Checklist

From [minecraft.wiki/w/Entity_format](https://minecraft.wiki/w/Entity_format?section=6)


- [ ] [Short] Air: How much air the entity has, in game ticks. Decreases when unable to breathe (except suffocating in a block). Increases when it can breathe. [Short] Air being `<= -20` game ticks (while still unable to breathe) on a given game tick causes the entity to immediately lose 1 health to [drowning](https://minecraft.wiki/w/Drowning "Drowning") damage. This resets [Short] Air to 0 game ticks. Most mobs can have a maximum of 300 game ticks (15 seconds) of [Short] Air, while dolphins can reach up to 4800 game ticks (240 seconds), and axolotls have 6000 game ticks (300 seconds).
- [ ] [String] CustomName: The custom name [text component](https://minecraft.wiki/w/Text_component "Text component") of this entity. Appears in player death messages and villager trading interfaces, as well as above the entity when the player's cursor is over it. May be empty or not exist. Represents the [`minecraft:custom_name`](https://minecraft.wiki/w/Data_component_format#custom_name "Data component format") component.
- [ ] [Boolean] CustomNameVisible: `1` or `0` (`true`/`false`) - if `true`, and this entity has a custom name, the name always appears above the entity, regardless of where the cursor points. If the entity does not have a custom name, a default name is shown. May not exist.
- [x] [Double] fall\_distance: Distance the entity has fallen. Larger values cause more damage when the entity lands.
- [ ] [Short] Fire: Number of game ticks until the fire is put out. Negative values reflect how long the entity can stand in fire before burning. Default to -20 when not on fire.
- [ ] [Boolean] HasVisualFire: `1` or `0` (`true`/`false`) - if `true`, the entity visually appears on fire, even if it is not actually on fire.
- [ ] [String] id: [Namespaced ID](https://minecraft.wiki/w/Resource_location#Conversion_to_string "Resource location") of the entity's [entity type](https://minecraft.wiki/w/Java_Edition_data_values#Entities "Java Edition data values"). Does not exist for entities in the world, only stored entities (such as in [data components](https://minecraft.wiki/w/Data_components "Data components")).
- [ ] [Boolean] Invulnerable: `1` or `0` (`true`/`false`) - if `true`, the entity should not take damage. This applies to living and nonliving entities alike: mobs should not take damage from any source (including potion effects), and cannot be moved by fishing rods, attacks, explosions, or projectiles, and objects such as vehicles and item frames cannot be destroyed unless their supports are removed. Invulnerable player entities are also ignored by any hostile mobs. Note that these entities can be damaged by players in Creative mode.
- [x] [NBT List / JSON Array] Motion: List of 3 [Double] doubles describing the current `dX`, `dY`, and `dZ` velocity of the entity in meters per game tick. Only allows between 10.0 and -10.0 (inclusive), else resets to 0.
- [x] [Boolean] OnGround: `1` or `0` (`true`/`false`) - if `true`, the entity is touching the ground.
- [ ] [NBT List / JSON Array] Passengers: The data of the entities that are riding this entity.
  - [NBT Compound / JSON Object]: The same as this format (recursive).
- [ ] [Int] PortalCooldown: The number of game ticks before which the entity may be teleported back through a nether portal. Initially starts at 300 game ticks (15 seconds) after teleportation and counts down to 0.
- [x] [NBT List / JSON Array] Pos: List of 3 [Double] doubles describing the current X, Y, and Z position ([coordinates](https://minecraft.wiki/w/Coordinates "Coordinates")) of the entity.
- [x] [NBT List / JSON Array] Rotation: List of 2 [Float] floats representing the entity's server-side rotation (facing direction). This does not necessarily match with the direction that the mob is looking in on the client's side. The yaw and pitch are stored inverted for most projectile entities.
  - [Float] 0: The entity's **yaw**. Yaw is rotation about the Y axis. It is measured in degrees and ranges from -180 to +180. The yaw increases as the entity turns clockwise (*right* from their perspective). A yaw of 0 means the entity is facing south. See table of specific values here: [Rotation (yaw)](https://minecraft.wiki/w/Chunk_format/Entity/Rotation_(yaw) "Chunk format/Entity/Rotation (yaw)").
  - [Float] 1: The entity's **pitch**. Pitch is rotation about the local X axis after yaw is applied. It is measured in degrees and ranges from -90 to +90. A pitch of 0 means the entity is facing parallel to the ground or directly towards the horizon. The pitch increases as the entity looks downwards, so a pitch of 90 is facing directly down and a pitch of -90 facing directly up.
- [ ] [NBT List / JSON Array] Tags: List of [scoreboard tags](https://minecraft.wiki/w/Scoreboard#Tags "Scoreboard") of this entity. It is not preserved if it is removed.
- [x] [Int Array] UUID: This entity's [Universally Unique IDentifier](https://minecraft.wiki/w/Universally_Unique_IDentifier "Universally Unique IDentifier"). The 128-bit UUID is stored as four 32-bit integers ([Int] Ints), ordered from most to least significant.

### Mobs

[edit](https://minecraft.wiki/w/Entity_format?section=7&veaction=edit "Edit section: Mobs") | [edit source](https://minecraft.wiki/w/Entity_format?action=edit&section=7 "Edit section's source code: Mobs")]

Mobs are a subclass of Living Entity with additional tags to store their health, attacking/damaged state, potion effects, and more depending on the mob. [Players](https://minecraft.wiki/w/Player.dat_format#NBT_structure "Player.dat format") and [armor stands](https://minecraft.wiki/w/Armor_stand "Armor stand") are a subclass of living entities.

- [ ] [Float] AbsorptionAmount: number of extra health added by Absorption effect.
- [ ] [NBT List / JSON Array] active\_effects: The list of status effects on this mob. May not exist.
  - [NBT Compound / JSON Object] Each item is a [status effect](https://minecraft.wiki/w/Status_effect "Status effect")
    - [Boolean] ambient: `1` or `0` (`true`/`false`) - if `true`, this effect is provided by a Beacon and therefore should be less intrusive on screen.
    - [Byte] amplifier: The status effect level. `0` is level 1.
    - [Int] duration: The number of [game ticks](https://minecraft.wiki/w/Game_tick "Game tick") before the effect wears off. `-1` means infinite duration.
    - [NBT Compound / JSON Object] hidden\_effect: Lower amplifier effect of the same type, this replaces the above effect when it expires. (The duration of the effect still decreases in here too)
    - [String] id: The [effect name](https://minecraft.wiki/w/Status_effect#Effect_list "Status effect").
    - [Boolean] show\_icon: `1` or `0` (`true`/`false`) - if `true`, effect icon is shown; if `false`, no icon is shown.
    - [Boolean] show\_particles: `1` or `0` (`true`/`false`) - if `true`, particles are shown (affected by `ambient`); if `false`, no particles are shown.
- [ ] [NBT List / JSON Array] attributes: A list of [Attributes](https://minecraft.wiki/w/Attribute "Attribute") for this mob. These are used for many purposes in internal calculations, and can be considered a mob's "statistics". Valid attributes for a given mob are listed in the [main article](https://minecraft.wiki/w/Attribute "Attribute").
  - [NBT Compound / JSON Object] An individual attribute.
    - [String] id: The name of this attribute.
    - [Double] base: The base value of this attribute.
    - [NBT List / JSON Array] modifiers: A list of [Modifiers](https://minecraft.wiki/w/Attribute#Modifiers "Attribute") acting on this attribute. Modifiers alter the base value in internal calculations, without changing the original copy. Note that a modifier never modifies base to be higher than its maximum or lower than its minimum for a given attribute.
      - [NBT Compound / JSON Object] An individual modifier.
        - [Double] amount: The amount by which this modifier modifies the base value in calculations.
        - [String] id: A [Resource location](https://minecraft.wiki/w/Resource_location "Resource location") unique to this modifier. Used to identify the modifier so that the correct modifier can be added or removed.
        - [String] operation: `add_value`, `add_multiplied_base`, `add_multiplied_total`. Defines the operation this modifier executes on the attribute's base value.
          - `add_value`: Increment `X``Amount`.
          - `add_multiplied_base`: `Y``X * Amount`.
          - `add_multiplied_total`: Set `Y = Y * (1 + Amount)` (equivalent to Increment `Y``Y * Amount`).
      - The specified modifiers are applied to the attribute, probably whenever the attribute is modified.​[*[more information needed](https://minecraft.wiki/w/Special:TalkPage/Entity_format "Special:TalkPage/Entity format")*] To compute the effective value of the attribute, the game:
        1. Sets `X = Base`.
        2. Executes all add\_value modifiers.
        3. Sets `Y = X`.
        4. Executes all add\_multiplied\_base modifiers.
        5. Executes all add\_multiplied\_total modifiers.
        - The value Y is the final effective value of the attribute.​[*[more information needed](https://minecraft.wiki/w/Special:TalkPage/Entity_format "Special:TalkPage/Entity format")*]
- [ ] [Boolean] CanPickUpLoot: `1` or `0` (`true`/`false`) - if `true`, the mob can pick up loot (wear armor it picks up, use weapons it picks up).
- Tags common to all mobs with drops from loot tables see [Template:Nbt inherit/death\_lootable/template](https://minecraft.wiki/w/Template:Nbt_inherit/death_lootable/template "Template:Nbt inherit/death lootable/template")[show]
- [ ] [Short] DeathTime: Number of ticks the mob has been dead for. Controls death animations. 0 when alive.
- [ ] [NBT Compound / JSON Object] drop\_chances: A map between equipment slot type and chance value. Each entry specifies the chance that the item in that slot is dropped when the mob dies. If not specified or removed, chance is assumed as default (0.085f). A chance value between 0.0f and 1.0f applies a [random](https://minecraft.wiki/w/Drops#Equipped_items "Drops") damage value if dropped and it's only drops if the mob is killed by a player or a tamed wolf. For values higher than 1.0f, the item damage is preserved and it's always dropped. Equipment picked up by mobs is set to 2.0f. Each entry is also used to calculate the chance the item in dropped when swap for a more preferred one[[1]](https://minecraft.wiki/w/Entity_format#cite_note-1), but not directly. If the value is 0.0f or less, the drop chance is 0%, if the value is greater than 0.0f, the drop chance is the value +10% and the item damage is preserved.
  - [Float] head : Chance value for the head item to drop.
  - [Float] chest : Chance value for the chest item to drop.
  - [Float] legs : Chance value for the legs item to drop.
  - [Float] feet : Chance value for the feet item to drop.
  - [Float] mainhand : Chance value for the mainhand item to drop.
  - [Float] offhand : Chance value for the offhand item to drop.
  - [Float] body : Chance value for the body item to drop.
  - [Float] saddle : Chance value for the saddle item to drop.
- [ ] [NBT Compound / JSON Object] equipment: Map between equipment slot type and item stack. Does not exist if the inventory is empty.
  - [NBT Compound / JSON Object] head: The item being held in mob's head slot.
  - [NBT Compound / JSON Object] chest: The item being held in the mob's chest slot.
  - [NBT Compound / JSON Object] legs: The item being held in the mob's legs slot.
  - [NBT Compound / JSON Object] feet: The item being held in the mob's feet slot.
  - [NBT Compound / JSON Object] mainhand: The item being held in the mob's main hand.
  - [NBT Compound / JSON Object] offhand: The item being held in the mob's off hand.
  - [NBT Compound / JSON Object] body: The item being held in the mob's body slot.
  - [NBT Compound / JSON Object] saddle: The item being held in the mob's saddle slot.
    - A single item stack see [Template:Nbt inherit/itemnoslot/template](https://minecraft.wiki/w/Template:Nbt_inherit/itemnoslot/template "Template:Nbt inherit/itemnoslot/template")[show]
- [x] [Float] Health: number of health the entity has.
- [ ] [Int Array] home\_pos: The mob's "home" position. Mobs will limit their pathfinding to stay within the indicated area. Some mobs, like bats, slimes, magma cubes, phantoms and ender dragons may ignore it. Interacting with leashes or riding may change the home position of the mob. For [creakings](https://minecraft.wiki/w/Creaking "Creaking"), this is the position of their [creaking heart](https://minecraft.wiki/w/Creaking_heart "Creaking heart").
- [ ] [Int] home\_radius: Max radius of the data `home_pos`.
- [ ] [Int] HurtByTimestamp: The last time the mob was damaged, measured in the number of ticks since the mob's creation. Updates to a new value whenever the mob is damaged, then updates again 101 ticks later for reasons unknown. Can be changed with [commands](https://minecraft.wiki/w/Commands "Commands"), but the specified value does not affect natural updates in any way, and is overwritten if the mob receives damage.
- [ ] [Short] HurtTime: Number of ticks the mob turns red for after being hit. 0 when not recently hit.
- [ ] [Int Array] last\_hurt\_by\_mob: The UUID of the last mob that attacked this mob. Clears when the attacking mob dies or despawns.
- [ ] [Int Array] last\_hurt\_by\_player: The UUID of the last player that attacked this mob.
- [ ] [Int] last\_hurt\_by\_player\_memory\_time: (when [Int Array] last\_hurt\_by\_player exists and is valid) Gets set to 100 game ticks (5 seconds) when attacked by a player, and decreases by 1 for every game tick. Clears [Int Array] last\_hurt\_by\_player when the value reaches 0.
- [ ] [NBT Compound / JSON Object][Int Array] leash: Information about where this leash connects to. Does not exist if the entity is not leashed.
  - The int array form ([Int Array]) represents the block location of the fence post that the leash is attached to (3 integers representing the X, Y, and Z coordinates respectively), or a compound containing information about the entity the leash is attached to.
  - The compound form ([NBT Compound / JSON Object]) contains the UUID of the entity that the leash is attached to.
  - [Int Array] UUID: This [Universally Unique IDentifier](https://minecraft.wiki/w/Universally_Unique_IDentifier "Universally Unique IDentifier") of the entity that the leash is attached to.
- [ ] [Boolean] LeftHanded: `1` or `0` (`true`/`false`) - if `true`, the mob renders the main hand as being left.
- [ ] [NBT Compound / JSON Object] locator\_bar\_icon: The waypoint's icon visual data in the [locator bar](https://minecraft.wiki/w/Locator_bar "Locator bar").
  - [Int] color: The waypoint's color stored as 32-bit signed integer using [two's complement](https://en.wikipedia.org/wiki/two%27s_complement "wikipedia:two's complement"), assuming the color is fully opaque.
  - [String] style: The waypoint's style name from `waypoint_style` directory in a [resource pack](https://minecraft.wiki/w/Resource_pack "Resource pack").
- [ ] [Boolean] NoAI: `1` or `0` (`true`/`false`) - Setting to `true` disables the mob's AI. The mob does not and cannot move, to the extent of not falling when normally able.
- [x] [Boolean] PersistenceRequired: `1` or `0` (`true`/`false`) - if `true`, the mob must not despawn naturally.
- [ ] [Int Array] sleeping\_pos: The coordinate of where the entity is sleeping, absent if not sleeping.
- [ ] [String] Team: This tag is actually not part of the NBT data of a mob, but instead used when spawning it, so it cannot be tested for. It makes the mob instantly join the [scoreboard](https://minecraft.wiki/w/Scoreboard "Scoreboard") team with that name.
- [ ] [Int] ticks\_since\_last\_hurt\_by\_mob: (when [Int Array] last\_hurt\_by\_mob exists and is valid) The number of game ticks since the last time the mob was attacked by the mob described by [Int Array] last\_hurt\_by\_mob.

## [**Player Specific**](https://minecraft.wiki/w/Player "Player")

- [ ] [NBT Compound / JSON Object] abilities: The abilities this player has. (I feel like these can be inferred - don't need to be data driven)
  - [Byte] flying: 1 or 0 (`true`/`false`) - `true` if the player is currently flying.
  - [Float] flySpeed: The flying speed, set to `0.05`.
  - [Byte] instabuild: 1 or 0 (`true`/`false`) - If `true`, the player can place blocks without depleting them. This is `true` for Creative mode, and `false` for other game modes.
  - [Byte] invulnerable: 1 or 0 (`true`/`false`) - Behavior is not the same as the invulnerable tag on other entities. If `true`, the player is immune to all damage and harmful effects except for [void](https://minecraft.wiki/w/Void "Void") damage and `/kill`. Also, all mobs, whether hostile or not, are passive to the player. `true` when in Creative or Spectator mode, and `false` when in Survival or Adventure mode.
  - [Byte] mayBuild: 1 or 0 (`true`/`false`) - If `true`, the player can place blocks. `true` when in Creative or Survival mode, and `false` when in Spectator or Adventure mode.
  - [Byte] mayfly: 1 or 0 (`true`/`false`) - If `true`, the player can fly and doesn't take fall damage. `true` when in Creative and Spectator modes, and `false` when in Survival and Adventure modes.
  - [Float] walkSpeed: The walking speed, set to `0.1`.
- [ ] [String] Dimension: The [ID](https://minecraft.wiki/w/Resource_location "Resource location") of the dimension the player is in. Used to store the player's last known location along with `Pos`.
- [ ] [NBT List / JSON Array] EnderItems: Each compound tag in this list is an item in the player's 27-slot ender chest inventory. When empty, list type may have [unexpected value](https://minecraft.wiki/w/NBT_format#Usage "NBT format").
  - [NBT Compound / JSON Object] An item in the inventory.
    - Includes the [Byte] Slot tag - slots are numbered `0`–`26`, inclusive.
    - See [Item\_format § NBT\_structure](https://minecraft.wiki/w/Item_format#NBT_structure "Item format").
- [ ] [NBT List / JSON Array] entered\_nether\_pos: May not exist. A list of 3 doubles, describing the [Overworld](https://minecraft.wiki/w/Overworld "Overworld") position from which the player entered the [Nether](https://minecraft.wiki/w/The_Nether "The Nether"). Used by the `nether_travel` [advancement](https://minecraft.wiki/w/Advancement "Advancement") trigger. Set every time the player passes through [a portal](https://minecraft.wiki/w/Nether_portal "Nether portal") from the Overworld to the Nether. When entering a dimension other than the nether *(not by respawning)* this tag is removed. Entering the Nether without using a portal does not update this tag.
  - [Double] x: The X coordinate in the Overworld.
  - [Double] y: The Y coordinate in the Overworld.
  - [Double] z: The Z coordinate in the Overworld.
- [ ] [Float] foodExhaustionLevel: See [Hunger § Mechanics](https://minecraft.wiki/w/Hunger#Mechanics "Hunger").
- [ ] [Int] foodLevel: The value of the hunger bar. Referred to as **hunger**. See [Hunger](https://minecraft.wiki/w/Hunger "Hunger").
- [ ] [Float] foodSaturationLevel: Referred to as **saturation**. See [Hunger § Mechanics](https://minecraft.wiki/w/Hunger#Mechanics "Hunger").
- [ ] [Int] foodTickTimer: See [Hunger](https://minecraft.wiki/w/Hunger "Hunger").
- [ ] [Boolean] ignore\_fall\_damage\_from\_current\_explosion: 1 or 0 (`true`/`false`) - `true` if the current explosion should apply a fall damage reduction. Only used by explosions from [wind charges](https://minecraft.wiki/w/Wind_charges "Wind charges").
- [ ] [NBT List / JSON Array] Inventory: Each compound tag in this list is an item in the player's inventory. (Note: when empty, list type may have [unexpected value](https://minecraft.wiki/w/NBT_format#Usage "NBT format").)
  - [NBT Compound / JSON Object] An item in the inventory.
    - See [Item\_format § NBT\_structure](https://minecraft.wiki/w/Item_format#NBT_structure "Item format").
- [ ] [NBT Compound / JSON Object] LastDeathLocation: May not exist. Location of the player's last death.
  - [String] dimension: Dimension of last death.
  - [Int Array] pos: Coordinates of last death.
- [ ] [Int] playerGameType: The current game mode of the player. `0` means [Survival](https://minecraft.wiki/w/Survival "Survival"), `1` means [Creative](https://minecraft.wiki/w/Creative "Creative"), `2` means [Adventure](https://minecraft.wiki/w/Adventure "Adventure"), and `3` means [Spectator](https://minecraft.wiki/w/Spectator "Spectator").
- [ ] [Int] previousPlayerGameType: The previous game mode of the player.
- [ ] [NBT Compound / JSON Object] recipeBook: Contains a JSON object detailing recipes the player has unlocked.

  - Tags related to the recipe book see [Template:Nbt inherit/Recipe Book/template](https://minecraft.wiki/w/Template:Nbt_inherit/Recipe_Book/template "Template:Nbt inherit/Recipe Book/template")[show]
- [ ] [NBT Compound / JSON Object] RootVehicle: May not exist. The root entity that the player is riding.
  - [Int Array] Attach: The [UUID](https://minecraft.wiki/w/UUID "UUID") of the entity the player is riding, stored as four ints.
  - [NBT Compound / JSON Object] Entity: The NBT data of the root vehicle.
    - See Entity format.
- [ ] [Int] Score: The score displayed upon death.
- [ ] [Byte] seenCredits: 1 or 0 (`true`/`false`) - `true` if the player has entered the [exit portal](https://minecraft.wiki/w/Exit_portal "Exit portal") in the [End](https://minecraft.wiki/w/The_End "The End") at least once.
- [ ] [NBT Compound / JSON Object] SelectedItem: Data of the item currently being held by the player, excluding the [Slot](https://minecraft.wiki/w/Player.dat_format#Inventory_slot_numbers "Player.dat format") tag. Only exists when using the /data command, this value is not saved in the [player.dat format](https://minecraft.wiki/w/Player.dat_format "Player.dat format").
  - See [item format](https://minecraft.wiki/w/Item_format "Item format").
- [ ] [Int] SelectedItemSlot: The selected hotbar slot of the player. Values are 0-indexed, so the leftmost slot is 0 and the rightmost slot is 8.
- [ ] [NBT Compound / JSON Object] ShoulderEntityLeft: The entity that is on the player's left shoulder. Always displays as a parrot.
  - See Entity format.
- [ ] [NBT Compound / JSON Object] ShoulderEntityRight: The entity that is on the player's right shoulder. Always displays as a parrot.
  - See Entity format.
- [ ] [Short] SleepTimer: The number of [game ticks](https://minecraft.wiki/w/Game_tick "Game tick") the player had been in bed. `0` when the player is not sleeping. When in bed, increases up to 100 ticks, then stops. Skips the night after enough players in beds have reached 100 (see [Bed § Passing the night](https://minecraft.wiki/w/Bed#Passing_the_night "Bed")). When getting out of bed, instantly changes to 100 ticks and then increases for another 9 ticks (up to 109 ticks) before returning to 0 ticks.
- [ ] [NBT Compound / JSON Object] respawn: May not exist. The respawn information of the player. Removed when the player attempts to respawn with no valid bed or respawn anchor to spawn at these coordinates. They are unaffected by breaking a bed or respawn anchor at these coordinates, and are unaffected by the player's death.
  - [Int Array] pos: block position to spawn at
  - [Float] yaw: angle to spawn with (default: 0.0)
  - [String] dimension: dimension id to spawn in (default minecraft:overworld) (required)
  - [Float] pitch: pitch to spawn with. (required)
  - [Boolean] forced: true if this spawn was set through commands (default: false)
- [ ] [NBT Compound / JSON Object] warden\_spawn\_tracker: Contains data about the [warden](https://minecraft.wiki/w/Warden "Warden") spawning process for this player.
  - [Int] warning\_level: A warning level between `0`, and `3` (inclusive). The warden spawns at level 3.
  - [Int] cooldown\_ticks: The number of game ticks before the `warning_level` can be increased again. Decreases by 1 every tick. It is set to 200 game ticks (10 seconds) every time the warning level is increased.
  - [Int] ticks\_since\_last\_warning: The number of game ticks since the player was warned for warden spawning. Increases by 1 every tick. After 12000 game ticks (10 minutes) it resets to level 3, and the `warning_level` decreases by 1 level.
- [ ] [Int] XpLevel: The level shown on the [experience](https://minecraft.wiki/w/Experience "Experience") bar.
- [ ] [Float] XpP: The progress across the experience bar to the next level, stored as a percentage.[*[verify](https://minecraft.wiki/w/Special:TalkPage/Entity_format "Special:TalkPage/Entity format")*]
- [ ] [Int] XpSeed: The seed used for the next enchantment in [enchanting tables](https://minecraft.wiki/w/Enchanting_Table "Enchanting Table").
- [ ] [Int] XpTotal: The total amount of experience the player has collected over time; used for the score upon death.
