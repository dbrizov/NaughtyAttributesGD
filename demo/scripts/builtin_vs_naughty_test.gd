@tool # required for method calls in attributes
extends Node

@export_group("Built-in")
@export_range(0, 10, 0.1)
var slider_builtin: float = 5.0
@export_custom(PROPERTY_HINT_FLAGS, "Fire,Ice,Wind")
var flags_builtin: int = 0

@export_group("Naughty")
@export_custom(PROPERTY_HINT_NONE, "range:0,10,0.1")
var slider_naughty: float = 5.0
@export_custom(PROPERTY_HINT_NONE, "flags:Fire,Ice,Wind; on_value_changed:print_flags")
var flags_naughty: int = 0


func print_flags(old_value, new_value):
	print(old_value, " -> ", new_value)
