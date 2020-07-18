.. raw:: html

    <script async defer src="https://buttons.github.io/buttons.js"></script>

.. role:: raw-html(raw)
   :format: html

My Projects and Collaborations
==============================

Most of my projects live on Github, where I have the user name
:raw-html:`<a class="github-button" href="https://github.com/mgeier" data-show-count="true" aria-label="Follow @mgeier on GitHub">@mgeier</a>`

Created and Maintained by Me
----------------------------

``sounddevice`` Module for Python
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Play and record sound with Python.
The ``sounddevice`` module provides bindings for the PortAudio library
and a few convenience functions to play and record
NumPy arrays containing audio signals.

https://python-sounddevice.readthedocs.io/

:raw-html:`<a class="github-button" href="https://github.com/spatialaudio/python-sounddevice" data-icon="octicon-star" data-show-count="true" aria-label="Star spatialaudio/python-sounddevice on GitHub">spatialaudio/python-sounddevice</a>`


``rtmixer`` Module for Python
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Reliable low-latency audio playback and recording with Python,
using PortAudio via the ``sounddevice`` module.

https://python-rtmixer.readthedocs.io/

:raw-html:`<a class="github-button" href="https://github.com/spatialaudio/python-rtmixer" data-icon="octicon-star" data-show-count="true" aria-label="Star spatialaudio/python-rtmixer on GitHub">spatialaudio/python-rtmixer</a>`

The ``rtmixer`` module uses a Python wrapper for PortAudio's ring buffer,
which I've published as a separate project:

:raw-html:`<a class="github-button" href="https://github.com/spatialaudio/python-pa-ringbuffer" data-icon="octicon-star" data-show-count="true" aria-label="Star spatialaudio/python-pa-ringbuffer on GitHub">spatialaudio/python-pa-ringbuffer</a>`

``jack`` Module for Python
^^^^^^^^^^^^^^^^^^^^^^^^^^

Python bindings for the JACK Audio Connection Kit (JACK_).

.. _JACK: https://jackaudio.org/

https://jackclient-python.readthedocs.io/

:raw-html:`<a class="github-button" href="https://github.com/spatialaudio/jackclient-python" data-icon="octicon-star" data-show-count="true" aria-label="Star spatialaudio/jackclient-python on GitHub">spatialaudio/jackclient-python</a>`

``nbsphinx`` Extension for Sphinx
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

``nbsphinx`` is a Sphinx_ extension that provides a source parser for
``*.ipynb`` files.
Custom Sphinx directives are used to show `Jupyter Notebook`_ code cells (and of
course their results) in both HTML and LaTeX output.
Un-evaluated notebooks -- i.e. notebooks without stored output cells -- will be
automatically executed during the Sphinx build process.

.. _Sphinx: https://www.sphinx-doc.org/
.. _Jupyter Notebook: https://jupyter.org/

https://nbsphinx.readthedocs.io/

:raw-html:`<a class="github-button" href="https://github.com/spatialaudio/nbsphinx" data-icon="octicon-star" data-show-count="true" aria-label="Star spatialaudio/nbsphinx on GitHub">spatialaudio/nbsphinx</a>`

``sphinx_last_updated_by_git`` Extension for Sphinx
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

As the name suggests,
this Sphinx_ extension uses Git to find out the date/time when each source file
was last updated (which is typically displayed in the footer of each page).

:raw-html:`<a class="github-button" href="https://github.com/mgeier/sphinx-last-updated-by-git" data-icon="octicon-star" data-show-count="true" aria-label="Star mgeier/sphinx-last-updated-by-git on GitHub">mgeier/sphinx-last-updated-by-git</a>`


Jupyter Notebooks About Python & Audio
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A collection of some more and some less finished Jupyter notebooks
about signal processing in Python (and more).

https://nbviewer.jupyter.org/github/mgeier/python-audio/blob/master/index.ipynb

:raw-html:`<a class="github-button" href="https://github.com/mgeier/python-audio" data-icon="octicon-star" data-show-count="true" aria-label="Star mgeier/python-audio on GitHub">mgeier/python-audio</a>`


This "Homepage"
^^^^^^^^^^^^^^^

A collection of notes about different topics.

https://mg.readthedocs.io/

:raw-html:`<a class="github-button" href="https://github.com/mgeier/homepage" data-icon="octicon-star" data-show-count="true" aria-label="Star mgeier/homepage on GitHub">mgeier/homepage</a>`

``jupyter_format`` module for Python
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An experimental new serialization format for Jupyter notebooks
(as replacement for the ``.ipynb`` format).

https://jupyter-format.readthedocs.io/en/latest/

:raw-html:`<a class="github-button" href="https://github.com/mgeier/jupyter-format" data-icon="octicon-star" data-show-count="true" aria-label="Star mgeier/jupyter-format on GitHub">mgeier/jupyter-format</a>`

Collaborations
--------------

The SoundScape Renderer
^^^^^^^^^^^^^^^^^^^^^^^

The SoundScape Renderer (SSR) is a tool for
real-time spatial audio reproduction providing a variety of rendering algorithms,
e.g. Wave Field Synthesis, Higher-Order Ambisonics and binaural techniques.

http://spatialaudio.net/ssr/

:raw-html:`<a class="github-button" href="https://github.com/SoundScapeRenderer/ssr" data-icon="octicon-star" data-show-count="true" aria-label="Star SoundScapeRenderer/ssr on GitHub">SoundScapeRenderer/ssr</a>`

The multi-threaded signal-processing core of the SSR is provided separately
as a set of re-usable C++ libraries called "Audio Processing Framework":

:raw-html:`<a class="github-button" href="https://github.com/AudioProcessingFramework/apf" data-icon="octicon-star" data-show-count="true" aria-label="Star AudioProcessingFramework/apf on GitHub">AudioProcessingFramework/apf</a>`

``sfs`` Module for Python
^^^^^^^^^^^^^^^^^^^^^^^^^

A Python library for creating numercial simulations of
*sound field synthesis* methods like Wave Field Synthesis (WFS) or
Near-Field Compensated Higher Order Ambisonics (NFC-HOA).

https://sfs-python.readthedocs.io/

:raw-html:`<a class="github-button" href="https://github.com/sfstoolbox/sfs-python" data-icon="octicon-star" data-show-count="true" aria-label="Star sfstoolbox/sfs-python on GitHub">sfstoolbox/sfs-python</a>`

``soundfile`` Module for Python
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The ``soundfile`` module can read and write sound files.
File reading/writing is supported through libsndfile_ via CFFI_.

.. _libsndfile: http://www.mega-nerd.com/libsndfile/
.. _CFFI: https://cffi.readthedocs.io/

https://python-soundfile.readthedocs.io/


:raw-html:`<a class="github-button" href="https://github.com/bastibe/SoundFile" data-icon="octicon-star" data-show-count="true" aria-label="Star bastibe/SoundFile on GitHub">bastibe/SoundFile</a>`

Exercises for "Communication Acoustics" Lecture
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Course material
(provided as Jupyter notebooks)
for the exercises accompanying the lecture "Acoustics for Communications"
(in German "Kommunikationsakustik") at
`Institute of Communications Engineering/Faculty of Computer Science
and Electrical Engineering/University of Rostock`__.

__ http://www.int.uni-rostock.de/

On ``nbviewer``:
https://nbviewer.jupyter.org/github/spatialaudio/communication-acoustics-exercises/blob/master/index.ipynb

:raw-html:`<a class="github-button" href="https://github.com/spatialaudio/communication-acoustics-exercises" data-icon="octicon-star" data-show-count="true" aria-label="Star spatialaudio/communication-acoustics-exercises on GitHub">spatialaudio/communication-acoustics-exercises</a>`


Minor Contributions
-------------------

The following links show some projects I have contributed to,
with links to my "pull requests" and the issues I created for each project.


Audio-Related
^^^^^^^^^^^^^

* https://github.com/hoene/libmysofa
  -- `pull requests <https://github.com/hoene/libmysofa/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/hoene/libmysofa/issues?q=is:issue+author:mgeier>`__


Sphinx and Related
^^^^^^^^^^^^^^^^^^

* https://github.com/sphinx-doc/sphinx
  -- `pull requests <https://github.com/sphinx-doc/sphinx/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/sphinx-doc/sphinx/issues?q=is:issue+author:mgeier>`__
* https://github.com/bashtage/sphinx-material
  -- `pull requests <https://github.com/bashtage/sphinx-material/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/bashtage/sphinx-material/issues?q=is:issue+author:mgeier>`__
* https://github.com/guzzle/guzzle_sphinx_theme
  -- `pull requests <https://github.com/guzzle/guzzle_sphinx_theme/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/guzzle/guzzle_sphinx_theme/issues?q=is:issue+author:mgeier>`__
* https://github.com/readthedocs/sphinx_rtd_theme
  -- `pull requests <https://github.com/readthedocs/sphinx_rtd_theme/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/readthedocs/sphinx_rtd_theme/issues?q=is:issue+author:mgeier>`__
* https://github.com/readthedocs/readthedocs.org
  -- `pull requests <https://github.com/readthedocs/readthedocs.org/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/readthedocs/readthedocs.org/issues?q=is:issue+author:mgeier>`__
* https://github.com/readthedocs/recommonmark
  -- `pull requests <https://github.com/readthedocs/recommonmark/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/readthedocs/recommonmark/issues?q=is:issue+author:mgeier>`__
* https://github.com/executablebooks/sphinx-copybutton
  -- `pull requests <https://github.com/executablebooks/sphinx-copybutton/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/executablebooks/sphinx-copybutton/issues?q=is:issue+author:mgeier>`__


Jupyter Ecosystem
^^^^^^^^^^^^^^^^^

* https://github.com/jupyter/notebook
  -- `pull requests <https://github.com/jupyter/notebook/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/jupyter/notebook/issues?q=is:issue+author:mgeier>`__
* https://github.com/jupyterlab/jupyterlab
  -- `pull requests <https://github.com/jupyterlab/jupyterlab/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/jupyterlab/jupyterlab/issues?q=is:issue+author:mgeier>`__
* https://github.com/ipython/ipython
  -- `pull requests <https://github.com/ipython/ipython/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/ipython/ipython/issues?q=is:issue+author:mgeier>`__
* https://github.com/jupyter/nbconvert
  -- `pull requests <https://github.com/jupyter/nbconvert/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/jupyter/nbconvert/issues?q=is:issue+author:mgeier>`__
* https://github.com/jupyter-widgets/ipywidgets
  -- `pull requests <https://github.com/jupyter-widgets/ipywidgets/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/jupyter-widgets/ipywidgets/issues?q=is:issue+author:mgeier>`__
* https://github.com/jupyter/nbclient
  -- `pull requests <https://github.com/jupyter/nbclient/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/jupyter/nbclient/issues?q=is:issue+author:mgeier>`__


Scientific Python Fundamentals
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

* https://github.com/numpy/numpy
  -- `pull requests <https://github.com/numpy/numpy/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/numpy/numpy/issues?q=is:issue+author:mgeier>`__
* https://github.com/sympy/sympy
  -- `pull requests <https://github.com/sympy/sympy/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/sympy/sympy/issues?q=is:issue+author:mgeier>`__
* https://github.com/matplotlib/matplotlib
  -- `pull requests <https://github.com/matplotlib/matplotlib/pulls?q=is:pr+author:mgeier>`__
  -- `issues <https://github.com/matplotlib/matplotlib/issues?q=is:issue+author:mgeier>`__
