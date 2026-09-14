extends Node3D

enum DamageType { FIRE = 1, ICE = 2, WIND = 4 }

# Takes the enum itself, so the names and values follow the declaration.
@export_custom(PROPERTY_HINT_NONE, "enum_flags:DamageType")
var naughty_damage_type: int = DamageType.FIRE | DamageType.ICE

# The built-in annotation accepts only string literals, which a rename leaves stale.
@export_flags("FIRE:1", "ICE:2", "WIND:4")
var builtin_damage_type: int = DamageType.FIRE | DamageType.ICE
