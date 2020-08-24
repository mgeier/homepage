Using Latexmk
=============

If you use cross-references, you often have to run LaTeX more than once, if you
use BibTeX for your bibliography or if you want to have a glossary you even need
to run external programs in-between.

To avoid all this hassle, you should simply use Latexmk_!

Latexmk_ is a Perl_ script which you just have to run once and it does
everything else for you ... completely automagically.

And the nice thing is: you probably have it already installed on your computer,
because it is part of MacTeX_ and MikTeX_ and it is bundled with many Linux
Distributions.

.. _Latexmk: http://www.phys.psu.edu/~collins/software/latexmk-jcc/
.. _Perl: http://www.perl.org/
.. _MacTeX: http://www.tug.org/mactex/
.. _MikTeX: http://miktex.org/

Installation
------------

On *Linux*:

   * Perl_ should be already installed.
   * You may have to install a package called ``latexmk`` or similar.

On *macOS* with MacTeX_:

   * It's probably already installed.
   * If not, open "TeX Live Utility", search for "latexmk" and install it.
   * If you prefer using the Terminal::

        sudo tlmgr install latexmk

On *Windows* with MikTeX_:

   * You probably have to install Perl_,
     e.g. from here: http://strawberryperl.com/.

   * If it's not installed already, open the MikTeX Package Manager and install
     the ``latexmk`` package.

Running Latexmk
---------------

.. highlight:: sh

Latexmk is a command line application, see `below <batch_>`__ for how to use it
with batch files.

In the simplest case you just have to type ::

   latexmk

This runs LaTeX on all ``.tex`` files in the current directory using the output
format specified by the `configuration files`_.

If you want to make sure to get a ``.pdf`` file as output, just mention it::

   latexmk -pdf

If you want to use ``latex`` instead of ``pdflatex`` but still want a ``.pdf``
file in the end, use ::

   latexmk -pdfps

If you want to compile only one specific ``.tex`` file in the current directory,
just provide the file name::

   latexmk myfile.tex

If you want to preview the resulting output file(s), just use ::

   latexmk -pv

And now the *Killer Feature*:
If you want Latexmk to continuously check all input files for changes and
re-compile the whole thing if needed and always display the result, type ::

   latexmk -pvc

Then, whenever you change something in *any* of your source files and save your
changes, the preview is automagically updated.
*But:* This doesn't work with all viewers, especially not with *Adobe Reader*.
See the section about `configuration files`_ below for setting a suitable viewer
application.

Of course, options can be combined, e.g. ::

   latexmk -pdf -pv myfile.tex

Cleaning Up
-----------

After running LaTeX, the current directory is contaminated with a myriad of
temporary files; you can get rid of them with ::

   latexmk -c

This doesn't delete the final ``.pdf``/``.ps``/``.dvi`` files.
If you want to delete those too, use ::

   latexmk -C

.. _batch:

Running Latexmk with Batch Files
--------------------------------

.. highlight:: bat

If you are working on *Windows*, you may not be used to typing things at the
command line. You may prefer clicking on things.

No problem, just create a file (in the same folder as your ``.tex`` files)
with the extension ``.bat`` containing ::

   latexmk
   @pause

and double-click on it.

You can of course use all the abovementioned options, don't forget the
especially useful ones ``-c`` and ``-pvc``.

Configuration Files
-------------------

.. highlight:: perl

On *Linux*, you can put your configurations into ``$HOME/.latexmkrc``,
which could contain something like this::

   $dvi_previewer = 'start xdvi -watchfile 1.5';
   $ps_previewer  = 'start gv --watch';
   $pdf_previewer = 'start evince';

On *macOS*, you can also use ``$HOME/.latexmkrc``, e.g. with this content::

   $pdf_previewer = 'open -a Skim';
   $pdflatex = 'pdflatex -synctex=1 -interaction=nonstopmode';
   @generated_exts = (@generated_exts, 'synctex.gz');

This uses Skim_ as preview application, which can be set up to automatically
update its display when the PDF file changes by selecting
"Preferences" -- "Sync" -- "Check for file changes".
While you are at it, you should also activates the *SyncTeX* feature by
selecting you editor right below in the "PDF-TeX Sync support" section.
With this selected and with ``-synctex=1`` in your LaTeX call, you can
Shift-⌘-click in the preview window and jump directly to the corresponding
source text in your editor!

.. _Skim: http://skim-app.sourceforge.net/

On *Windows*, you can use the system-wide config file ``C:\latexmk\LatexMk``
(if the file doesn't exist yet, just create a new text file with this name).
To choose a PDF viewer, use something like this::

   $pdf_previewer = 'start gsview32';

You'll need *GSview* and *Ghostscript* for that,
see http://pages.cs.wisc.edu/~ghost/gsview/.

Some previewers use different methods for updating the viewed PDF file.
You can change that with ``$pdf_update_method``, like in this example::

   $pdf_update_method = 4;
   $pdf_update_command = 'bla bla bla';

Full documentation is available in the manpage_.

.. _manpage: http://personal.psu.edu/~jcc8/software/latexmk/latexmk-469a.txt

Local Configuration Files
-------------------------

You can also put a configuration file in the current directory for settings
which only influence files in the current directory.
Such a configuration file has to be named ``latexmkrc`` or ``.latexmkrc`` and
may contain some of the following lines.

To specify if you want PDF or PS output, choose one of those::

   $pdf_mode = 1;        # tex -> pdf
   $pdf_mode = 2;        # tex -> ps -> pdf
   $postscript_mode = 1; # tex -> ps

If you have your work split up into several parts, you have to specify the main
file like this::

   @default_files = ('main.tex');

Or maybe you want to process several files::

   @default_files = ('file-one.tex', 'file-two.tex');

.. note:: If you don't specify ``@default_files``, all ``.tex`` files in the
   current directory will be used.

Advanced Options
----------------

Latexmk can also do more crazy stuff.

For example, it can create a nomenclature (you'll have to use the *nomencl*
package) like this::

   @cus_dep_list = (@cus_dep_list, "glo gls 0 makenomenclature");
   sub makenomenclature {
      system("makeindex $_[0].glo -s nomencl.ist -o $_[0].gls"); }
   @generated_exts = (@generated_exts, 'glo');

Or, if you are creating your figures in EPS format but you need them in PDF, you
can tell Latexmk to convert them for you::

   @cus_dep_list = (@cus_dep_list, "eps pdf 0 eps2pdf");
   sub eps2pdf {
      system("epstopdf $_[0].eps"); }

If you need to enable shell escape for ``\write18``
(e.g. for on-the-fly figure generation)::

   $latex = 'latex -interaction=nonstopmode -shell-escape';
   $pdflatex = 'pdflatex -interaction=nonstopmode -shell-escape';

And finally, if ``latexmk -c`` refuses to remove certain files, you can specify
their extensions and next time they'll be gone::

   $clean_ext = "bbl nav out snm";

Have fun!
