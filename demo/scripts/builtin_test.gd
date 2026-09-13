extends Node

@export var speed: float = 1.0
@export_range(0, 10, 0.1) var slider: float = 5.0
@export_custom(PROPERTY_HINT_FLAGS, "Fire,Ice,Wind") var flags: int = 0
