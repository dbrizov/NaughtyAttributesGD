Installation
============
.. note::
    NaughtyAttributes requires **Godot 4.6** or later versions.

The addon ships prebuilt binaries for Windows x86_64, Linux x86_64 and macOS (universal),
so there is nothing to compile and no Rust toolchain to install.

1. The addon is available in the `Asset Store <https://store.godotengine.org/asset/denis-rizov/naughtyattributes/>`_.
2. Alternatively, download the latest zip from the `Releases page <https://github.com/dbrizov/NaughtyAttributesGD/releases>`_
   and copy ``addons/naughtyattributes/`` into your project.

Restart the editor afterwards.

.. note::
    **There is no checkbox in Project Settings → Plugins, and that is not a bug.**
    GDExtension classes register themselves when the library loads, so the addon is active as soon as the editor restarts.
    There is nothing to enable.
