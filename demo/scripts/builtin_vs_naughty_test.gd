extends Node

@export_range(0, 10, 0.1) var slider_builtin: float = 5.0
@export_multiline var multiline_builtin: String = ""

@export_custom(PROPERTY_HINT_NONE, "range:0,10,0.1") var slider_naughty: float = 5.0
@export_custom(PROPERTY_HINT_NONE, "multiline:") var multiline_naughty: String = ""
