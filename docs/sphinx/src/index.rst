Introduction
============
NaughtyAttributes is an attributes extension for the Godot Inspector.

It expands the range of things you can say about an exported property, so that you can build powerful inspectors
without writing an ``EditorInspectorPlugin`` or an ``EditorProperty`` of your own.
Conditional visibility, value clamping, validation messages, help boxes and custom widgets are all written inline,
next to the property they describe.

It's built as a GDExtension in Rust, and it is aimed at **GDScript users**.

.. note::
    GDScript has no user-definable annotations, so attributes are written as strings in the hint text of
    ``@export_custom``. See :ref:`label-writing-attributes` before reaching for any of the attribute pages.

Contribute
----------
If you want to contribute you can visit the `GitHub Repo <https://github.com/dbrizov/NaughtyAttributesGD>`_ and give me pull requests.
The project is written in Rust, uses ``LF`` line endings, and is expected to pass ``cargo fmt --all --check`` and ``cargo clippy --workspace``.
It is a must that you respect the coding standard.

Donation
--------
I am developing the project in my free time. If you like it you can support me by donating.

- `PayPal <https://paypal.me/dbrizov>`_
- `Buy Me A Coffee <https://www.buymeacoffee.com/dbrizov>`_


.. toctree::
    :maxdepth: 1
    :caption: General
    :name: sec-general

    general/installation
    general/writing_attributes
    general/diagnostics

.. toctree::
    :maxdepth: 1
    :caption: Attributes
    :name: sec-attributes

    attributes/decorator_attributes/index
    attributes/drawer_attributes/index
    attributes/meta_attributes/index
    attributes/validator_attributes/index

.. toctree::
    :maxdepth: 1
    :caption: About
    :name: sec-about

    credits
