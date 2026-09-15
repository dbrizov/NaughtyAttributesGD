.. _label-diagnostics:

Diagnostics
===========
GDScript has no compile-time checking for string attributes, so mistakes are reported loudly instead.
Everything goes to the Output panel, prefixed ``[NaughtyAttributes]``, in the form::

    {script_path}.{property} - {key}: {message}

For example::

    [NaughtyAttributes] res://scripts/player.gd.health - min_value: unknown identifier "floor_hp"

A typo names the script, the property and the attribute rather than silently doing nothing.

What is reported
----------------
- An unknown attribute key. It does not claim the property, which is left to Godot.
- An attribute whose arguments do not parse, or whose expression does not compile. The attribute is dropped.
- An expression that fails at evaluation time. A failing condition counts as satisfied, so a property stays
  visible and enabled, and a validator skips its correction.
- A second drawer attribute on one property, or a second built-in hint in one hint text. The last one wins
  and the displaced one is reported.
- A callback named by :ref:`label-on-value-changed` that the object cannot call.

.. note::
    A message drawn beside a property by :ref:`label-require` is not an error and is never logged.
    It is the attribute working as written over wrong data.

.. note::
    If a failing expression contains a call, the message ends with a hint: without ``@tool`` on the script,
    that the call needs it; with ``@tool``, to check the method's name and arguments.
    See :ref:`label-writing-attributes`.
