@tool
extends Node3D

@export var min_hp: int = 10
@export var min_ratio: float = 0.25

@export_custom(PROPERTY_HINT_NONE, "min_value:min_hp") var hp: int = 50
@export_custom(PROPERTY_HINT_NONE, "min_value:get_min_ratio()") var ratio: float = 0.5
@export_custom(PROPERTY_HINT_NONE, "min_value:0") var int_min_0: int = 5
@export_custom(PROPERTY_HINT_NONE, "min_value:0") var float_min_0: float = 0.5
@export_custom(PROPERTY_HINT_NONE, "min_value:0") var vector3_min_0: Vector3 = Vector3.ONE


func get_min_ratio() -> float:
	return min_ratio
