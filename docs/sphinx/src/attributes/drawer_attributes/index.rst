.. _label-drawer-attributes:

Drawer Attributes
=================
Drawer attributes replace the property's widget with one of their own.

.. note::
    A property can have only one drawer attribute. If it has more, the last one wins
    and the others are reported as errors.

.. note::
    A drawer combines freely with the other families. Decorators still draw above the property,
    metas still hide, grey out and relabel it, and validators still run on its value.

.. toctree::
    :maxdepth: 1
    :name: toc-drawer-attributes

    dropdown
    enum_flags
    min_max_slider
