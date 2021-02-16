Creating a Python Module
========================

.. note::

  This is, much like quantum physics, and the universe, work-in-progress.

Here are some very subjective recommendations how to create and publish a Python
module.

Coding Style
------------

Have a look at `PEP 8 <https://www.python.org/dev/peps/pep-0008/>`_ and
`PEP 257 <https://www.python.org/dev/peps/pep-0257/>`_ and
probably also at the `Google Python Style Guide
<https://google.github.io/styleguide/pyguide.html>`_.

Docstrings
----------

Use the `NumPy Docstring Standard <https://github.com/numpy/numpy/blob/master/doc/HOWTO_DOCUMENT.rst.txt>`_.

Testing
-------

py.test

Coverage
--------

.. todo:: coverage tool, probably py.test extension

Online Documentation
--------------------

Use `Sphinx <https://www.sphinx-doc.org/>`_ and https://readthedocs.org/.

Use `sphinx.ext.autodoc <https://www.sphinx-doc.org/en/master/usage/extensions/autodoc.html>`_
and `sphinx.ext.napoleon <https://www.sphinx-doc.org/en/master/usage/extensions/napoleon.html>`_.

Installer
---------

`setuptools <https://setuptools.readthedocs.io/>`_

https://python-packaging-user-guide.readthedocs.io/

https://web.archive.org/web/20140921105456/https://caremad.io/blog/setup-vs-requirement/

https://web.archive.org/web/20201214202421/https://manikos.github.io/a-tour-on-python-packaging

https://jwodder.github.io/kbits/posts/pypkg-mistakes/


License
-------

There are many possibilities, one of them is the
`MIT license <https://opensource.org/licenses/mit-license.php>`_::

  Copyright (c) <year> <copyright holders>
  
  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:
  
  The above copyright notice and this permission notice shall be included in
  all copies or substantial portions of the Software.
  
  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
  THE SOFTWARE.

If your module consists of only one source file, this can be included in the top
as a comment.
This makes it easy for others to take just the one file and put it into their
own project while still keeping the copyright notice.

If the module consists of several source files, this is not necessary.
It's enough to put the copyright notice and the license text into a file
``LICENSE`` in the main directory (you should have this file in any case).

Further Reading
---------------

https://opensource.guide/

.. vim:textwidth=80
