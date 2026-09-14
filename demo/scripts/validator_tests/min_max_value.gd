@tool # required for method calls in attributes
extends Node3D

@export
var min_hp: int = 0
@export
var max_hp: int = 100
@export
var min_ratio: float = 0.25
@export
var max_ratio: float = 0.75

@export_custom(PROPERTY_HINT_NONE, "min_value:min_hp; max_value:max_hp")
var hp: int = 50
@export_custom(PROPERTY_HINT_NONE, "min_value:get_min_ratio(); max_value:get_max_ratio()")
var ratio: float = 0.5
@export_custom(PROPERTY_HINT_NONE, "min_value:0; max_value:1")
var int_min_0_max_1: int = 0
@export_custom(PROPERTY_HINT_NONE, "min_value:0; max_value:1")
var float_min_0_max_1: float = 0.5
@export_custom(PROPERTY_HINT_NONE, "min_value:0; max_value:1")
var vector3_min_0_max_1: Vector3 = Vector3.ONE * 0.5


func get_min_ratio() -> float:
	return self.min_ratio


func get_max_ratio() -> float:
	return self.max_ratio
