@tool # required for method calls in attributes
extends Node3D

@export
var min_size: int = 4

@export_custom(PROPERTY_HINT_NONE, "require:size >= min_size")
var size: int = 2

@export_custom(PROPERTY_HINT_NONE, "resource_type:Texture2D; require:icon")
var icon: Texture2D

@export_custom(PROPERTY_HINT_NONE, "require:is_even(even_number)")
var even_number: int = 3


func is_even(number: int):
	return number % 2 == 0
