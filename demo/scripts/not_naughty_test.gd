extends Node

@export var tags: Array[int] = []
@export var speed: float = 1.0
@export_range(0, 10, 0.1) var ratio: float = 0.5
@export_multiline var notes: String = ""
