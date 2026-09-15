.. _label-writing-attributes:

Writing Attributes
==================
GDScript has no user-definable annotations, so attributes are written as strings in the hint text of
``@export_custom``, separated by ``;``::

    @export_custom(PROPERTY_HINT_NONE, "min_value:0; max_value:100")
    var health: int = 50

An attribute's arguments follow a ``:``, and multi-argument attributes separate them with ``,``::

    @export_custom(PROPERTY_HINT_NONE, "min_max_slider:0,100")
    var damage_range: Vector2

Everything after the first ``:`` belongs to the attribute, so a condition can contain colons and commas of its own.

.. note::
    The first argument of ``@export_custom`` must be ``PROPERTY_HINT_NONE``.
    A hint text is parsed only when the property carries no built-in hint;
    any other hint is passed through to Godot untouched.

Escaping
--------
``;`` separates attributes and ``,`` separates arguments. Escape them as ``\;`` and ``\\,`` --
the double backslash is GDScript's, which rejects ``\,`` inside a string literal.

Attributes that take their text whole (:ref:`label-message-box` and :ref:`label-label`) need no comma escaping at all.

@tool
-----
Conditions are expressions evaluated against the edited object.
Reading properties and comparing enum values works on an ordinary script::

    extends Node

    @export
    var is_immortal: bool

    @export_custom(PROPERTY_HINT_NONE, "show_if:!is_immortal")
    var health: int = 100

**Calling a method requires** ``@tool`` **on your script.**
Without it the editor gives the script a placeholder instance, which holds values but has no methods::

    @tool # required for method calls in attributes
    extends Node

    @export_custom(PROPERTY_HINT_NONE, "show_if:is_at_max_health()")
    var at_max_health: String = ""


    func is_at_max_health():
        return self.health >= self.max_health

Expressions are Godot's own ``Expression`` class, so comparisons, ``!``, parentheses, ``and``/``or``
(and their ``&&``/``||`` spellings), constants and enum values all work::

    @tool # required for method calls in attributes
    extends Node

    @export
    var is_immortal: bool
    @export
    var max_health: int = 100
    @export
    var health: int = 100

    @export_custom(PROPERTY_HINT_NONE, "show_if:is_at_max_health() || is_immortal")
    var unbreakable: String = ""


    func is_at_max_health():
        return self.health >= self.max_health

Godot's own hints still work
----------------------------
Built-in hints are re-encoded rather than reimplemented, and handed to Godot's native editor untouched.
That lets a built-in and an attribute share the one hint text slot::

    @export_custom(PROPERTY_HINT_NONE, "range:0,10,0.1; show_if:is_ranged")
    var spread: float

``range``, ``enum``, ``flags``, ``multiline``, ``resource_type``, ``node_type``, ``dir``, ``file``
and the rest behave exactly as their ``@export_*`` equivalents.

This also matters for **reference properties**. A plain ``@export var icon: Texture2D`` already carries a hint,
so it cannot carry attributes. Re-encode the picker and the attribute rides along::

    @export_custom(PROPERTY_HINT_NONE, "resource_type:Texture2D; require:icon")
    var icon: Texture2D
