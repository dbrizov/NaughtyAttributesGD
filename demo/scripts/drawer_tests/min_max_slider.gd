@tool
extends Node3D

@export var max_hp: int = 100
@export var max_stamina: int = 50

@export_custom(PROPERTY_HINT_NONE, "min_max_slider:0,max_hp") var hp: Vector2 = Vector2(10, 50)
@export_custom(PROPERTY_HINT_NONE, "min_max_slider:0,get_max_stamina(50)")
var stamina: Vector2 = Vector2(10, 40)
@export_custom(PROPERTY_HINT_NONE, "min_max_slider:0,10") var damage: Vector2 = Vector2(2, 8)
@export_custom(PROPERTY_HINT_NONE, "min_max_slider:1,60") var level: Vector2i = Vector2i(20, 40)


func get_max_stamina(bonus_stamin: int):
	return self.max_stamina + bonus_stamin
