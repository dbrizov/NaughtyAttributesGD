@tool
extends Node3D

@export_custom(
	PROPERTY_HINT_NONE, "on_value_changed:update_power; on_value_changed:_print_value_change"
)
var strength: int = 10

@export_custom(
	PROPERTY_HINT_NONE, "on_value_changed:update_power; on_value_changed:_print_value_change"
)
var dexterity: int = 7

@export_custom(PROPERTY_HINT_NONE, "read_only") var power: int = 0


func update_power(_old_value, _new_value):
	self.power = self.strength * 2 + self.dexterity


func _print_value_change(old_value, new_value):
	print(old_value, " -> ", new_value)
