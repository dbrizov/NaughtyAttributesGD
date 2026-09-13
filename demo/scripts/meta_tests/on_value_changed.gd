@tool
extends Node3D

@export_custom(PROPERTY_HINT_NONE, "on_value_changed:print_value_change") var strength: int = 10

@export_custom(PROPERTY_HINT_NONE, "on_value_changed:print_value_change") var dexterity: int = 7

@export_custom(PROPERTY_HINT_NONE, "read_only") var power: int:
	get:
		return strength * 2 + dexterity


func print_value_change(old_value, new_value):
	print(old_value, " -> ", new_value)
