Importing Local Python Modules from Jupyter Notebooks
=====================================================

If you re-use local modules a lot,
you should consider turning them into proper Python packages
which can be installed with Python's package manager ``pip``.

The following sections are created from Jupyter notebooks
which show multiple ways to import local Python modules,
even if they are located in sub-directories.

The file :download:`module-subdirectory/mymodule.py` is used as a
dummy example module.

If you know other (reasonable) methods to use local modules,
please create an issue or a pull request!

.. toctree::

   symbolic-link/symlink
   sys-path-in-notebook/path-notebook
   sys-path-in-helper-module/path-helper
