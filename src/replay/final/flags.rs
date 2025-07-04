
// buttons
// https://www.sourcemod.net/new-api/entity_prop_stocks/__raw
// // These defines are for client button presses.
// #define IN_ATTACK                (1 << 0)
// #define IN_JUMP                  (1 << 1)
// #define IN_DUCK                  (1 << 2)
// #define IN_FORWARD               (1 << 3)
// #define IN_BACK                  (1 << 4)
// #define IN_USE                   (1 << 5)
// #define IN_CANCEL                (1 << 6)
// #define IN_LEFT                  (1 << 7)
// #define IN_RIGHT                 (1 << 8)
// #define IN_MOVELEFT              (1 << 9)
// #define IN_MOVERIGHT             (1 << 10)
// #define IN_ATTACK2               (1 << 11)
// #define IN_RUN                   (1 << 12)
// #define IN_RELOAD                (1 << 13)
// #define IN_ALT1                  (1 << 14)
// #define IN_ALT2                  (1 << 15)
// #define IN_SCORE                 (1 << 16)   /**< Used by client.dll for when scoreboard is held down */
// #define IN_SPEED                 (1 << 17)   /**< Player is holding the speed key */
// #define IN_WALK                  (1 << 18)   /**< Player holding walk key */
// #define IN_ZOOM                  (1 << 19)   /**< Zoom key for HUD zoom */
// #define IN_WEAPON1               (1 << 20)   /**< weapon defines these bits */
// #define IN_WEAPON2               (1 << 21)   /**< weapon defines these bits */
// #define IN_BULLRUSH              (1 << 22)
// #define IN_GRENADE1              (1 << 23)   /**< grenade 1 */
// #define IN_GRENADE2              (1 << 24)   /**< grenade 2 */
// #define IN_ATTACK3               (1 << 25)

use bitflags::bitflags;


bitflags! {
    /// Refer to this https://github.com/shavitush/bhoptimer/blob/14b967496211dc598c09af470fd02759da9fab9d/addons/sourcemod/scripting/shavit-hud.sp#L1889-L1893
    /// and this https://www.sourcemod.net/new-api/entity_prop_stocks/__raw
    /// to figure out what the fuck this shit means
    #[derive(Debug)]
    pub struct ButtonFlags: u32 {
        const IN_ATTACK     = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        const IN_JUMP       = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        const IN_DUCK       = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        const IN_FORWARD    = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        const IN_BACK       = 0b0000_0000_0000_0000_0000_0000_0001_0000;
        const IN_USE        = 0b0000_0000_0000_0000_0000_0000_0010_0000;
        const IN_CANCEL     = 0b0000_0000_0000_0000_0000_0000_0100_0000;
        const IN_LEFT       = 0b0000_0000_0000_0000_0000_0000_1000_0000;
        const IN_RIGHT      = 0b0000_0000_0000_0000_0000_0001_0000_0000;
        const IN_MOVELEFT   = 0b0000_0000_0000_0000_0000_0010_0000_0000;
        const IN_MOVERIGHT  = 0b0000_0000_0000_0000_0000_0100_0000_0000;
        const IN_ATTACK2    = 0b0000_0000_0000_0000_0000_1000_0000_0000;
        const IN_RUN        = 0b0000_0000_0000_0000_0001_0000_0000_0000;
        const IN_RELOAD     = 0b0000_0000_0000_0000_0010_0000_0000_0000;
        const IN_ALT1       = 0b0000_0000_0000_0000_0100_0000_0000_0000;
        const IN_ALT2       = 0b0000_0000_0000_0000_1000_0000_0000_0000;
        const IN_SCORE      = 0b0000_0000_0000_0001_0000_0000_0000_0000;
        const IN_SPEED      = 0b0000_0000_0000_0010_0000_0000_0000_0000;
        const IN_WALK       = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        const IN_ZOOM       = 0b0000_0000_0000_1000_0000_0000_0000_0000;
        const IN_WEAPON1    = 0b0000_0000_0001_0000_0000_0000_0000_0000;
        const IN_WEAPON2    = 0b0000_0000_0010_0000_0000_0000_0000_0000;
        const IN_BULLRUSH   = 0b0000_0000_0100_0000_0000_0000_0000_0000;
        const IN_GRENADE1   = 0b0000_0000_1000_0000_0000_0000_0000_0000;
        const IN_ATTACK3    = 0b0000_0001_0000_0000_0000_0000_0000_0000;
        const UNUSED1       = 0b0000_0010_0000_0000_0000_0000_0000_0000;
        const UNUSED2       = 0b0000_0100_0000_0000_0000_0000_0000_0000;
        const UNUSED3       = 0b0000_1000_0000_0000_0000_0000_0000_0000;
        const UNUSED4       = 0b0001_0000_0000_0000_0000_0000_0000_0000;
        const UNUSED5       = 0b0010_0000_0000_0000_0000_0000_0000_0000;
        const UNUSED6       = 0b0100_0000_0000_0000_0000_0000_0000_0000;
        const UNUSED7       = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    }
}

bitflags! {
    /// refer to this to figure out what these mean https://www.sourcemod.net/new-api/entity_prop_stocks/__raw
    /// used here in bhoptimer, so i believe its entity flags
    /// https://github.com/shavitush/bhoptimer/blob/466bcf62dfc4fd509b47f3968cf3a1d8c1751297/addons/sourcemod/scripting/shavit-checkpoints.sp#L1854C1-L1854C41
    #[derive(Debug)]
    pub struct EntityFlags: u32 {
        const FL_ONGROUND               = 0b0000_0000_0000_0000_0000_0000_0000_0001;
        const FL_DUCKING                = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        const FL_WATERJUMP              = 0b0000_0000_0000_0000_0000_0000_0000_0100;
        const FL_ONTRAIN                = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        const FL_INRAIN                 = 0b0000_0000_0000_0000_0000_0000_0001_0000;
        const FL_FROZEN                 = 0b0000_0000_0000_0000_0000_0000_0010_0000;
        const FL_ATCONTROLS             = 0b0000_0000_0000_0000_0000_0000_0100_0000;
        const FL_CLIENT                 = 0b0000_0000_0000_0000_0000_0000_1000_0000;
        const FL_FAKECLIENT             = 0b0000_0000_0000_0000_0000_0001_0000_0000;

        const FL_INWATER                = 0b0000_0000_0000_0000_0000_0010_0000_0000;
        const FL_FLY                    = 0b0000_0000_0000_0000_0000_0100_0000_0000;
        const FL_SWIM                   = 0b0000_0000_0000_0000_0000_1000_0000_0000;
        const FL_CONVEYOR               = 0b0000_0000_0000_0000_0001_0000_0000_0000;
        const FL_NPC                    = 0b0000_0000_0000_0000_0010_0000_0000_0000;
        const FL_GODMODE                = 0b0000_0000_0000_0000_0100_0000_0000_0000;
        const FL_NOTARGET               = 0b0000_0000_0000_0000_1000_0000_0000_0000;
        const FL_AIMTARGET              = 0b0000_0000_0000_0001_0000_0000_0000_0000;
        const FL_PARTIALGROUND          = 0b0000_0000_0000_0010_0000_0000_0000_0000;
        const FL_STATICPROP             = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        const FL_GRAPHED                = 0b0000_0000_0000_1000_0000_0000_0000_0000;
        const FL_STEPMOVEMENT           = 0b0000_0000_0001_0000_0000_0000_0000_0000;
        const FL_DONTTOUCH              = 0b0000_0000_0010_0000_0000_0000_0000_0000;
        const FL_BASEVELOCITY           = 0b0000_0000_0100_0000_0000_0000_0000_0000;
        const FL_WORLDBRUSH             = 0b0000_0000_1000_0000_0000_0000_0000_0000;
        const FL_OBJECT                 = 0b0000_0001_0000_0000_0000_0000_0000_0000;
        const FL_KILLME                 = 0b0000_0010_0000_0000_0000_0000_0000_0000;
        const FL_ONFIRE                 = 0b0000_0100_0000_0000_0000_0000_0000_0000;
        const FL_DISSOLVING             = 0b0000_1000_0000_0000_0000_0000_0000_0000;
        const FL_TRANSRAGDOLL           = 0b0001_0000_0000_0000_0000_0000_0000_0000;
        const FL_UNBLOCKABLE_BY_PLAYER  = 0b0010_0000_0000_0000_0000_0000_0000_0000;
        const FL_FREEZING               = 0b0100_0000_0000_0000_0000_0000_0000_0000;
        const FL_EP2V_UNKNOWN1          = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    }
}

/// View meanings here: https://www.sourcemod.net/new-api/entity_prop_stocks/MoveType
/// 
/// enum descriptions taken from above sourcemod link
#[derive(Debug)]
pub enum MoveType {
    /// Never moves
    None, 
    /// For players
    Isometric,
    /// Player only - moving on the ground
    Walk,
    /// Gravity, special edge handling -- monsters use this
    Step,
    /// No gravity, but still collides with stuff
    Fly,
    /// Flies through the air + is affected by gravity
    FlyGravity,
    /// uses VPHYSICS for simulation
    VPhysics,
    /// No clip to world, push and crush
    Push,
    /// No gravity, no collisions, still do velocity/avelocity
    Noclip,
    /// Used by players only when going onto a ladder
    Ladder,
    /// Observer movement, depends on player's observer mode
    Observer,
    /// Allows the entity to describe its own physics
    Custom,
}