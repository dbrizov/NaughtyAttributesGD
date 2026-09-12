extends Node3D

@export_group("Stats", "stats_")
## Hit points. Shown as a tooltip by the inspector.
@export_custom(PROPERTY_HINT_NONE, "min_value:0") var stats_hp: int = 100
@export_custom(PROPERTY_HINT_NONE, "min_value:0") var stats_armor: int = 10
@export_subgroup("Regen", "stats_regen_")
@export_custom(PROPERTY_HINT_NONE, "min_value:0") var stats_regen_rate: float = 1.5
@export var stats_regen_delay: float = 2.0
@export_custom(PROPERTY_HINT_NONE, "range:0,100,1") var stats_luck: int = 7
@export_custom(PROPERTY_HINT_NONE, "info_box:No stats_ prefix so this leaves the Stats group")
var speed: float = 5.0

@export_group("Loot")
@export_custom(PROPERTY_HINT_NONE, "range:0,1,0.01") var drop_chance: float = 0.25
@export var drop_table: Array[int] = []

@export_group("")
@export var trailing: bool = false

@export_category("Movement")
@export_custom(PROPERTY_HINT_NONE, "min_value:0") var walk_speed: Vector3 = Vector3.ONE
@export_multiline var movement_notes: String = ""
