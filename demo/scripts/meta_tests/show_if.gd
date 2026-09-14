@tool # required for method calls in attributes
extends Node3D

enum WeaponType { MELEE, RANGED, MAGIC }

@export
var is_immortal: bool = false

@export
var weapon_type: WeaponType = WeaponType.MELEE

@export_custom(PROPERTY_HINT_NONE, "show_if:weapon_type == WeaponType.MELEE")
var melee_class: String = "class_name"

@export_custom(PROPERTY_HINT_NONE, "show_if:!is_immortal")
var max_health: int = 100

@export_custom(PROPERTY_HINT_NONE, "show_if:!is_immortal")
var health: int = 100

@export_custom(PROPERTY_HINT_NONE, "show_if:is_at_max_health()")
var at_max_health: String = ""


func is_at_max_health():
	return self.health >= self.max_health
