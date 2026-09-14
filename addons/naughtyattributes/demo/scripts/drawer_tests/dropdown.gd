@tool # required for method calls in attributes
extends Node3D

const LEFT: Vector2 = Vector2(-1.0, 0.0)
const RIGHT: Vector2 = Vector2(1.0, 0.0)
const UP: Vector2 = Vector2(0.0, -1.0)
const DOWN: Vector2 = Vector2(0.0, 1.0)

@export
var titles: Array[String] = ["Novice", "Adept"]

@export_custom(PROPERTY_HINT_NONE, "dropdown:titles")
var title: String = "Novice"

@export_custom(PROPERTY_HINT_NONE, "dropdown:{'Warrior': 0, 'Archer': 1, 'Cleric': 2}")
var class_type: int = 0

@export_custom(PROPERTY_HINT_NONE, "dropdown:get_directions()")
var direction: Vector2 = LEFT


func get_directions():
	return {
		"Left": LEFT,
		"Right": RIGHT,
		"Up": UP,
		"Down": DOWN,
	}
