# NaughtyAttributesGD
[![Godot 4.6+](https://img.shields.io/badge/godot-4.6%2B-blue.svg)](https://godotengine.org/download)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://github.com/dbrizov/NaughtyAttributesGD/blob/master/LICENSE)

NaughtyAttributes is an attributes extension for the Godot Inspector.

It expands the range of things you can say about an exported property, so that you can build powerful inspectors without writing an `EditorInspectorPlugin` or an `EditorProperty` of your own. Conditional visibility, value clamping, validation messages, help boxes and custom widgets are all written inline, next to the property they describe.

It is a port of [NaughtyAttributes for Unity](https://github.com/dbrizov/NaughtyAttributes), built as a GDExtension in Rust, and it is aimed at **GDScript users**.

## System Requirements

Godot **4.6** or later. The addon ships prebuilt binaries for Windows x86_64, Linux x86_64 and macOS (universal) - no compilation and no Rust toolchain needed.

## Installation

1. (TODO) From the editor's **AssetLib** tab, search for NaughtyAttributes, download and install.
2. Or download the latest zip from [Releases](https://github.com/dbrizov/NaughtyAttributesGD/releases) and copy `addons/naughtyattributes/` into your project.

Restart the editor afterwards.

**There is no checkbox in Project Settings → Plugins, and that is not a bug.** GDExtension classes register themselves when the library loads, so the addon is active as soon as the editor restarts. There is nothing to enable.

## How attributes are written

GDScript has no user-definable annotations, so attributes are written as strings in the hint text of `@export_custom`, separated by `;`

```gdscript
@export_custom(PROPERTY_HINT_NONE, "min_value:0; max_value:100")
var health: int = 50
```

An attribute's arguments follow a `:`, and multi-argument attributes separate them with `,`

```gdscript
@export_custom(PROPERTY_HINT_NONE, "min_max_slider:0,100")
var damage_range: Vector2
```

Everything after the first `:` belongs to the attribute, so a condition can contain colons and commas of its own.

### Escaping

`;` separates attributes and `,` separates arguments. Escape them as `\;` and `\\,` - the double backslash is GDScript's, which rejects `\,` inside a string literal. Attributes that take their text whole (`info_box`, `warning_box`, `error_box`, `label`) need no comma escaping at all.

### `@tool`

Conditions are expressions evaluated against the edited object. Reading properties and comparing enum values works on an ordinary script:

```gdscript
extends Node

@export var is_immortal: bool

@export_custom(PROPERTY_HINT_NONE, "show_if:!is_immortal")
var health: int = 100
```

**Calling a method requires `@tool` on your script.** Without it the editor gives the script a placeholder instance, which holds values but has no methods:

```gdscript
@tool # required for method calls in attributes
extends Node

@export_custom(PROPERTY_HINT_NONE, "show_if:is_at_max_health()")
var at_max_health: String = ""


func is_at_max_health():
	return self.health >= self.max_health
```

### Godot's own hints still work

Built-in hints are re-encoded rather than reimplemented, and handed to Godot's native editor untouched. That lets a built-in and an attribute share the one hint text slot:

```gdscript
@export_custom(PROPERTY_HINT_NONE, "range:0,10,0.1; show_if:is_ranged")
var spread: float
```

`range`, `enum`, `flags`, `multiline`, `resource_type`, `node_type`, `dir`, `file` and the rest behave exactly as their `@export_*` equivalents.

This also matters for **reference properties**. A plain `@export var icon: Texture2D` already carries a hint, so it cannot carry attributes. Re-encode the picker and the attribute rides along:

```gdscript
@export_custom(PROPERTY_HINT_NONE, "resource_type:Texture2D; require:icon")
var icon: Texture2D
```

# Overview

## Decorator Attributes

Draw something extra above a property. A property can have any number of decorators; they stack in written order and hide together with the property they decorate.

### horizontal_line

Draws a separator above the property. Takes an optional hex color, with or without alpha; the default follows the editor theme.

```gdscript
extends Node3D

@export_custom(PROPERTY_HINT_NONE, "horizontal_line")
var default: int = 0

@export_custom(PROPERTY_HINT_NONE, "horizontal_line:#e06c75")
var red: int = 0

@export_custom(PROPERTY_HINT_NONE, "horizontal_line:#98c379")
var green: int = 0

@export_custom(PROPERTY_HINT_NONE, "horizontal_line:#61afef")
var blue: int = 0
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/horizontal_line.png)

### info_box / warning_box / error_box

Used for providing additional information.

```gdscript
extends Node3D

@export_custom(PROPERTY_HINT_NONE, "info_box:Information about this property")
var info: int = 0

@export_custom(PROPERTY_HINT_NONE, "warning_box:Hmm, something here looks off")
var warning: int = 0

@export_custom(PROPERTY_HINT_NONE, "error_box:This value breaks the build")
var error: int = 0
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/message_box.png)

The message takes the rest of the entry whole, so prose commas need no escaping.

## Drawer Attributes

Replace the property's widget. A property can have only one drawer - if it has more, the last one wins and the others are reported as errors.

### dropdown

Provides an interface for dropdown value selection. The option list is an expression, so it can name a property, a dictionary literal, or a method call - and it re-evaluates, so a list that reads another property follows it.

```gdscript
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
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/dropdown.gif)

An `Array` uses each element as both label and value; a `Dictionary` uses the key as the label and the value as the value. A stored value that matches no option is shown as `{value} (not an option)` until you pick a real one - the widget never writes on your behalf.

### enum_flags

Provides a checkbox interface for setting enum flags. Unlike the built-in `@export_flags`, it takes the enum itself, so the names and values follow the declaration and a rename cannot leave it stale.

```gdscript
extends Node3D

enum DamageType { FIRE = 1, ICE = 2, WIND = 4 }

# Takes the enum itself, so the names and values follow the declaration.
@export_custom(PROPERTY_HINT_NONE, "enum_flags:DamageType")
var naughty_damage_type: int = DamageType.FIRE | DamageType.ICE

# The built-in annotation accepts only string literals, which a rename leaves stale.
@export_flags("FIRE:1", "ICE:2", "WIND:4")
var builtin_damage_type: int = DamageType.FIRE | DamageType.ICE
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/enum_flags.png)

### min_max_slider

A double slider. The **min value** is saved to the **x** component and the **max value** to the **y** component of a `Vector2` or `Vector2i` property. Both bounds are expressions.

```gdscript
@tool # required for method calls in attributes
extends Node3D

@export
var max_hp: int = 100
@export
var max_stamina: int = 50

@export_custom(PROPERTY_HINT_NONE, "min_max_slider:0,max_hp")
var hp: Vector2 = Vector2(10, 50)
@export_custom(PROPERTY_HINT_NONE, "min_max_slider:0,get_max_stamina(50)")
var stamina: Vector2 = Vector2(10, 40)
@export_custom(PROPERTY_HINT_NONE, "min_max_slider:0,10")
var damage: Vector2 = Vector2(2, 8)
@export_custom(PROPERTY_HINT_NONE, "min_max_slider:1,60")
var level: Vector2i = Vector2i(20, 40)


func get_max_stamina(bonus_stamina: int):
	return self.max_stamina + bonus_stamina
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/min_max_slider.gif)

## Meta Attributes

Give properties metadata. A property can have more than one meta attribute.

### enable_if / disable_if

Greys the property out and refuses input. Decorators stay lit above a disabled property, and validators keep running - read-only is a drawing state, not a storage state.

```gdscript
@tool # required for method calls in attributes
extends Node3D

enum WeaponType { MELEE, RANGED, MAGIC }

@export
var is_immortal: bool = false

@export
var weapon_type: WeaponType = WeaponType.MELEE

@export_custom(PROPERTY_HINT_NONE, "enable_if:weapon_type == WeaponType.MELEE")
var melee_class: String = "class_name"

@export_custom(PROPERTY_HINT_NONE, "enable_if:!is_immortal")
var max_health: int = 100

@export_custom(PROPERTY_HINT_NONE, "disable_if:!is_at_max_health()")
var at_max_health: String = ""


func is_at_max_health():
	return self.health >= self.max_health
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/enable_disable_if.gif)

Conditions are full expressions, so `&&`, `||`, `!`, parentheses, comparisons and enum comparisons all work, and several conditions on one property are ANDed.

### label

Override the default property label.

```gdscript
extends Node3D

@export_custom(PROPERTY_HINT_NONE, "label:Maximum Health")
var max_hp: int = 100

@export_custom(PROPERTY_HINT_NONE, "label:Health")
var hp: int = 100

@export_custom(PROPERTY_HINT_NONE, "label:Damage")
var dmg: int = 15
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/label.png)

### on_value_changed

Detects a value change and executes a callback. The callback takes the old and the new value. Keep in mind that the event fires only when the value is changed **from the inspector** - undo and redo restore values silently.

```gdscript
@tool # required for method calls in attributes
extends Node3D

@export_custom(PROPERTY_HINT_NONE, "on_value_changed:print_value_change")
var strength: int = 10

@export_custom(PROPERTY_HINT_NONE, "on_value_changed:print_value_change")
var dexterity: int = 7

@export_custom(PROPERTY_HINT_NONE, "read_only")
var power: int:
	get:
		return strength * 2 + dexterity


func print_value_change(old_value, new_value):
	print(old_value, " -> ", new_value)
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/on_value_changed.gif)<br>
![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/on_value_changed_output.gif)

### read_only

Make a property read only.

```gdscript
extends Node3D

@export_custom(PROPERTY_HINT_NONE, "read_only")
var read_only: int = 0

@export
var not_read_only: int = 0
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/read_only.png)

### show_if / hide_if

Shows or hides the property, along with any decorators above it.

```gdscript
@tool # required for method calls in attributes
extends Node3D

enum WeaponType { MELEE, RANGED, MAGIC }

@export
var is_immortal: bool = false

@export
var weapon_type: WeaponType = WeaponType.MELEE

@export_custom(PROPERTY_HINT_NONE, "show_if:weapon_type == WeaponType.MELEE")
var melee_class: String = "class_name"

@export_custom(PROPERTY_HINT_NONE, "show_if:!is_immortal")
var health: int = 100

@export_custom(PROPERTY_HINT_NONE, "hide_if:!is_at_max_health()")
var at_max_health: String = ""


func is_at_max_health():
	return self.health >= self.max_health
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/show_hide_if.gif)

## Validator Attributes

Used for validating properties. A property can have any number of validator attributes.

### min_value / max_value

Clamps numeric properties. Both bounds are expressions, so they accept a literal, another property, or a method call, and a dependent re-clamps when its bound changes. Works on `int`, `float` and the six vector types.

```gdscript
@tool # required for method calls in attributes
extends Node3D

@export
var min_hp: int = 0
@export
var max_hp: int = 100
@export
var min_ratio: float = 0.25
@export
var max_ratio: float = 0.75

@export_custom(PROPERTY_HINT_NONE, "min_value:min_hp; max_value:max_hp")
var hp: int = 50
@export_custom(PROPERTY_HINT_NONE, "min_value:get_min_ratio(); max_value:get_max_ratio()")
var ratio: float = 0.5
@export_custom(PROPERTY_HINT_NONE, "min_value:0; max_value:1")
var int_min_0_max_1: int = 0
@export_custom(PROPERTY_HINT_NONE, "min_value:0; max_value:1")
var vector3_min_0_max_1: Vector3 = Vector3.ONE * 0.5


func get_min_ratio() -> float:
	return self.min_ratio


func get_max_ratio() -> float:
	return self.max_ratio
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/min_max_value.gif)

### require

The most powerful validator. It takes a boolean expression and draws a message beside the property when the expression is false. It corrects nothing and writes nothing, so it never enters the undo history.

```gdscript
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
```

![inspector](https://raw.githubusercontent.com/dbrizov/NaughtyAttributesGD/master/docs/markup/media/require.gif)

`require:icon` is a null check through Godot truthiness, `require:hp > 0` a bound, and `require:is_even(size)` a call with the argument written out.

## Diagnostics

GDScript has no compile-time checking for string attributes, so mistakes are reported loudly instead. Everything goes to the Output panel, prefixed `[NaughtyAttributes]`, in the form:

```
res://scripts/player.gd.health - min_value: unknown identifier "floor_hp"
```

A typo names the script, the property and the attribute rather than silently doing nothing.
