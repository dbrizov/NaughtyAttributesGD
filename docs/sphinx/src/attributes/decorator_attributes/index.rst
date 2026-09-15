.. _label-decorator-attributes:

Decorator Attributes
====================
Decorator attributes draw something extra above a property, instead of replacing the way the property itself is drawn.

.. note::
    Unlike :ref:`label-drawer-attributes`, a property can have any number of decorator attributes,
    and they can be combined with a drawer attribute. They stack in written order.

.. note::
    A decorator hides together with the property it decorates, so a property hidden by :ref:`label-show-hide-if`
    takes its decorators with it. A property merely disabled by :ref:`label-enable-disable-if` keeps them lit.

.. toctree::
    :maxdepth: 1
    :name: toc-decorator-attributes

    horizontal_line
    message_box
