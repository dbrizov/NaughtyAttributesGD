extends Node

@export var plain_float: float = 5.0

@export_custom(PROPERTY_HINT_NONE, "range:0,10,0.1")
var ranged: float = 5.0

@export_custom(PROPERTY_HINT_NONE, "enum:Melee,Ranged,Magic")
var kind: int = 0

@export_custom(PROPERTY_HINT_NONE, "multiline:")
var notes: String = ""

@export_custom(PROPERTY_HINT_NONE, "shwo_if:plain_float")
var typo: float = 0.0

@export var tags: Array[int] = []
