// consts for binary operations
pub const SEEN: i32 = 1;
pub const VISIBLE: i32 = 2;
pub const TARGETED: i32 = 4;

pub const WAITING: i32 = 0;
pub const FOV_CHANGE: i32 = 1;
pub const CONTENTS_CHANGE: i32 = 2;
pub const INVENTORY_CHANGE: i32 = 4;
pub const ARMOUR_CHANGE: i32 = 8;
pub const EXIT_MAP: i32 = 16;
pub const GAME_OVER: i32 = 32;

// key codes
// sections
pub const KEY_ESC: u32 = 27;
pub const KEY_I: u32 = 73;
pub const KEY_A: u32 = 65;
// movement
pub const KEY_LEFT: u32 = 37;
pub const KEY_UP: u32 = 38;
pub const KEY_RIGHT: u32 = 39;
pub const KEY_DOWN: u32 = 40;
pub const KEY_Y: u32 = 89;
pub const KEY_U: u32 = 85;
pub const KEY_B: u32 = 66;
pub const KEY_N: u32 = 78;
pub const KEY_G: u32 = 71;
// actions
pub const KEY_D: u32 = 68;
pub const KEY_R: u32 = 82;
pub const KEY_ENTER: u32 = 13;
// stairs
pub const KEY_GT: u32 = 190;
pub const KEY_LT: u32 = 188;

// MONSTERS 10 - 999
pub const MOB_WHITE_CENTIPEDE: i32 = 10;
pub const MOB_RED_ANT: i32 = 11;
pub const MOB_GHOST: i32 = 12;
pub const MOB_GREY_MOULD: i32 = 13;
pub const MOB_KOBOLD: i32 = 14;
pub const MOB_THIEF: i32 = 15;

// OTHER
pub const OTHER_DOOR: i32 = 1900;

// ITEMS
// Regular 2000 - 2099
pub const ITEM_HEALTH_POTION: i32 = 2000;
// Targetable 2100 - 2199
pub const ITEM_MAGIC_MISSILE: i32 = 2100; 
pub const ITEM_DRAGON_BREATH: i32 = 2101;
pub const ITEM_ACID_RAIN: i32 = 2102;
pub const ITEM_CONFUSION_SCROLL: i32 = 2103;

// WEAPONS
// Melee 3000 - 3099
pub const WEAP_DAGGER: i32 = 3000;
pub const WEAP_RUSTY_SWORD: i32 = 3001;
pub const WEAP_LONG_SWORD: i32 = 3002;
// Shield 3100 - 3199
pub const ARMOUR_SHIELD: i32 = 3100;
// Head 3200 - 3299
// Body 3300 - 3399
pub const ARMOUR_TUNIC: i32 = 3300;
// Legs 3400 - 3499
pub const ARMOUR_PANTS: i32 = 3400;
// Feet 3500 - 3599
pub const ARMOUR_OLD_BOOTS: i32 = 3500;
// Hands 3600 - 3699
// Neck 3700 - 3799
// Fingers 3800 - 3899

// TRAPS 5000 - 5099
pub const TRAP_CHASM: i32 = 5000;
pub const TRAP_LAVA: i32 = 5001;

// PARTICLES
// Particles can be from either the following list
// OR an Item code
pub const PARTICLE_ATTACK: i32 = 0;
pub const PARTICLE_DEFEND: i32 = 1;
pub const PARTICLE_EFFECT: i32 = 2;
