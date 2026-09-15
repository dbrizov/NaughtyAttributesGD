Installation
============
.. note::
    NaughtyAttributes requires **Godot 4.6** or later versions.

The addon ships prebuilt binaries for Windows x86_64, Linux x86_64 and macOS (universal),
so there is nothing to compile and no Rust toolchain to install.

1. Download the latest zip from the `Releases page <https://github.com/dbrizov/NaughtyAttributesGD/releases>`_
   and copy ``addons/naughtyattributes/`` into your project.

2. The addon is also being submitted to the editor's **Asset Store** tab, where it can be searched for,
   downloaded and installed without leaving Godot.

Restart the editor afterwards.

.. note::
    **There is no checkbox in Project Settings → Plugins, and that is not a bug.**
    GDExtension classes register themselves when the library loads, so the addon is active as soon as the editor restarts.
    There is nothing to enable.
