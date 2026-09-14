extends Node3D

@export_custom(PROPERTY_HINT_NONE, "info_box:Information about this property")
var info: int = 0

@export_custom(PROPERTY_HINT_NONE, "warning_box:Hmm, something here looks off")
var warning: int = 0

@export_custom(PROPERTY_HINT_NONE, "error_box:This value breaks the build")
var error: int = 0
