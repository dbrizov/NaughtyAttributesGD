extends Node3D

@export_custom(PROPERTY_HINT_NONE, "horizontal_line")
var max_hp = 100
@export
var hp = 100
@export
var level: int = 1

@export_custom(PROPERTY_HINT_NONE, "horizontal_line")
var strength: int = 10
@export
var dexterity: int = 7
@export
var intelect: int = -1

@export_custom(PROPERTY_HINT_NONE, "horizontal_line")
var melee_weapon: String = "Katana"
@export
var ranged_weapon: String = "Kunai"
