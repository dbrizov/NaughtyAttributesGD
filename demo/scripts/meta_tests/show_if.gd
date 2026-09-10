@tool
extends Node3D

enum WeaponKind { MELEE, RANGED, MAGIC }

@export var is_immortal: bool = false

@export var weapon_kind: WeaponKind = WeaponKind.MELEE

@export_custom(PROPERTY_HINT_NONE, "show_if:weapon_kind == WeaponKind.MELEE")
var melee_class: String = "class_name"

@export_custom(PROPERTY_HINT_NONE, "show_if:!is_immortal") var max_health: int = 100

@export_custom(PROPERTY_HINT_NONE, "show_if:!is_immortal") var health: int = max_health

@export_custom(PROPERTY_HINT_NONE, "show_if:is_at_max_health()") var at_max_health: bool = true


func is_at_max_health():
	return self.health == self.max_health
