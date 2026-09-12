extends Node3D

@export_custom(PROPERTY_HINT_NONE, "info_box:Information about this property") var info: int = 0
@export_custom(PROPERTY_HINT_NONE, "info_box:Something here looks off,warning") var warning: int = 0
@export_custom(PROPERTY_HINT_NONE, "info_box:This value breaks the build,error") var error: int = 0
