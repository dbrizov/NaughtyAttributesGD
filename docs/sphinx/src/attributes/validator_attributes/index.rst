.. _label-validator-attributes:

Validator Attributes
====================
Used for validating properties. A property can have any number of validator attributes.

A validator has two shapes over one job. It either **corrects** the value, as :ref:`label-min-max-value` does,
or it **rejects** it and draws a message beside the property, as :ref:`label-require` does.

.. note::
    Validators run before a property's widget is built, and again across every visible property on every
    inspector edit, so a property whose bound reads another property re-validates when that one changes.
    They do not run on a property hidden by :ref:`label-show-hide-if`.

.. toctree::
    :maxdepth: 1
    :name: toc-validator-attributes

    min_max_value
    require
