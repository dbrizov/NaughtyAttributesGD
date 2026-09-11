@tool
extends Node3D

@export var floor_hp: int = 10
@export var floor_ratio: float = 0.25

@export_custom(PROPERTY_HINT_NONE, "min_value:0") var int_min_0: int = 5
@export_custom(PROPERTY_HINT_NONE, "min_value:0") var float_min_0: float = 0.5
@export_custom(PROPERTY_HINT_NONE, "min_value:0") var vector3_min_0: Vector3 = Vector3.ONE
@export_custom(PROPERTY_HINT_NONE, "min_value:floor_hp") var hp_from_field: int = 50
@export_custom(PROPERTY_HINT_NONE, "min_value:get_floor_ratio") var ratio_from_method: float = 0.5


func get_floor_ratio() -> float:
	return floor_ratio
