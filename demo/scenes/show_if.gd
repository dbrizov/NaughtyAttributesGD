extends Node

@export var is_weapon: bool = false
@export_custom(PROPERTY_HINT_NONE, "show_if:is_weapon") var damage: float = 10.0
