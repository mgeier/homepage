Reproducible Research
=====================

.. note::

  This is, much like research itself, and the art of eating spaghetti without
  soiling yourself, work-in-progress.

This page is not as general as it should be.
It is biased towards audio signal processing, audio engineering, spatial audio
reproduction and auditory perception.
However, many of the ideas presented here can be applied more widely.

.. todo:: Reproducible Research, Reproducible Science, Reproducible Analysis, ...

.. todo:: Reproducible Research vs. Non-Reproducible Research?

.. todo:: reproducible vs. easily reproducible

.. todo:: online material as supplement to traditional publications

.. todo:: http://en.wikipedia.org/wiki/Reproducibility

.. todo:: https://github.com/mgeier/python-audio

`Vandewalle et al. <http://dx.doi.org/10.1109/MSP.2009.932122>`_ distinguish six
degrees of reproducibility:

  5. The results can be easily reproduced by an independent researcher with at
     most 15 min of user effort, requiring only standard, freely available tools
     (C compiler, etc.).

  4. The results can be easily reproduced by an independent researcher with at
     most 15 minutes of user effort, requiring some proprietary source packages
     (MATLAB, etc.).

  3. The results can be reproduced by an independent researcher, requiring
     considerable effort.

  2. The results could be reproduced by an independent researcher, requiring
     extreme effort.

  1. The results cannot seem to be reproduced by an independent researcher.

  0. The results cannot be reproduced by an independent researcher.

While I don't agree with all details (especially the over-concrete time
specifications and the overly vague effort metrics), I like the general idea.

Guidelines
----------

Here are few guidelines which may (or may not) help to make your work more
reproducible:

make everything public (and each step of it)
  At some point, every aspect of your work should be publicly accessible.
  And not only the parts which (you think) are most interesting ... every single
  bit and every single step.
  This way it will be easiest for others to reproduce your work.

  You may not want to publish everything from the very beginning, which leads to
  the next point ...

release early
  This is borrowed from the Open Source movement, but it's also applicable here.
  Even if you feel it's not finished yet, just make it public! Because if you
  wait too long, you'll probably never release it ...

  If you release early, you also give others the chance to comment on your work
  and to suggest improvements before you think it's "finished" (which may never
  happen).

make stuff public by default
  In case of doubt, make it public! Keep things only for yourself if there is a
  good reason. And even if there is a reason now, you should think about making
  it public later (e.g. after publication of a related paper).

think about others
  Don't just think about how great your results are, also think about how you
  can make it as easy as possible for others to reproduce them.

use tools that others can use, too
  If you have a choice, prefer tools that are available to other researchers,
  too.

  Of course, often expensive equipment is needed in research, and sometimes only
  few laboratories have even the theoretical possibility to reproduce your
  experiments. We have to live with that.

  When it comes to software, there is often an alternative to expensive
  programs, sometimes the free ones are even better.
  Try to choose software that is accessible to most people, and try to use
  software that runs on different operating systems.

use open source software
  TODO: content

specify a license
  If provide something to the public and don't specify a license, said public
  may have a hard time using the thing legally.
  With everything you publish, you should also tell people what they may and may
  not do with it.

  But remember: the more restrictions you impose, the more freedom you take away
  from people who want to use your work.
  You can waive all your rights (at least with regard to copyright law), you can
  request attribution, you can demand that derived works must be published under
  the same conditions as the original work (a.k.a. *share-alike*), you can
  forbid commercial use, ...

  Try these links to help you choose an appropriate license:

  * http://creativecommons.org/choose/
  * http://three.org/openart/license_chooser/

  For more details, have a look there: https://tldrlegal.com/.

bring research and teaching closer together
  Every research starts from some existing knowledge.

  TODO: more arguments

  Today's students are tomorrow's researchers.

What Should be Reproducible?
----------------------------

Short answer: everything!

But let's be a bit more verbose.
Ideally, the whole research process should be reproducible.
The following list shows things that can (and should!) be made reproducible.
There are also some tools mentioned that may help, see below for links to more
software and libraries.

All this is of course very much dependent on the research area. Some points may
apply to your area, others won't.

collecting ideas
  Ideas are the core of any research activity. They are also one of the main
  resources needed by researchers (besides funding). Understandably, many
  researcher are reluctant to make their ideas public before they reap their
  fruits themselves.

  But at a later time, e.g. after a publication, there may not be a reason
  anymore to keep the ideas a secret.
  Also, some researchers (mostly the good ones) have more ideas than they could
  possibly work on. In this case they should make their "vacant" ideas public
  for other researchers to work on.

  In the era of the world-wide-web there are countless possibilities to share
  your ideas, no need to give any pointers here, you'll find something.

symbolic derivations
  In many areas, deriving equations is the daily drill of a researcher.
  In traditional publications, however, only a limited amount of space can be
  used for equations, so typically only a few steps of the derivation are shown
  or even only the final resulting equation.

  This can make it very time-consuming for other researchers to reproduce and
  build on your results.
  Ideally, for every published equation the complete and detailed derivation
  should also be publicly available.

  You can create nice equations using LaTeX documents, but also some blogging
  systems support entering math equations. IPython also supports nice-looking
  equations (using MathJax).

  TODO: CASs

numeric calculations, simulations, visualizations, plots
  TODO: NumPy, SciPy, matplotlib, Mayavi, ...

cluster computing
  TODO: IPython

measurements
  TODO: settings, logs, software, pre-/post-processing scripts

experimental apparatus
  TODO: detailed description, drawings, photos, detailed list of devices ant
     the used configuration, ...

  TODO: software (ideally open source), scripts, configuration files, data
     files, ...

statistical evaluation
  TODO: raw data, all scripts

  TODO: pandas, R

Software
--------

The following is a completely subjective selection of open-source software.
This is not at all exhaustive, there are a lot of alternatives, both commercial
and non-commercial.

Python
^^^^^^

.. note:: Why Python?

   The chief reason is that it's just a beautiful programming language.
   And it's versatile ... so the *two* reasons are its beauty and versatility
   ... and its extensive standard library,
   therefore the *three* reasons to use Python are its beauty, versatility and
   extensive standard library ... and a sheer unimaginably humongous number of
   third-party libraries and extensions.

   Let's just say *amongst* the reasons to choose Python are such diverse
   elements as beauty, versatility, extremely useful standard library and
   tons of third-party stuff.

   For more information, watch this: http://youtu.be/vt0Y39eMvpI

Scientific Python (SciPy)
    http://scipy.org/

    This is a collection of many software projects:
    `NumPy <http://www.numpy.org/>`_,
    `SciPy <http://scipy.org/scipylib/>`_,
    `matplotlib <http://matplotlib.org/>`_,
    `IPython <http://ipython.org/>`_,
    `SymPy <http://sympy.org/>`_,
    `pandas <http://pandas.pydata.org/>`_,
    `Mayavi <http://code.enthought.com/projects/mayavi/>`_,
    `PyTables <http://www.pytables.org/>`_,
    and many more ...

.. todo:: IPython notebook

   connection to other tools via "magic" (R, Ruby, Octave, Julia, ...)

LaTeX
^^^^^

...

.. todo:: TikZ, gnuplot, beamer

Git
^^^

See :doc:`git`.

More Software
^^^^^^^^^^^^^

There's always more ...

R
    http://www.r-project.org/

Julia
    http://julialang.org/

Sage
    http://sagemath.org/

Publication Tools
-----------------

IPython
    http://ipython.org/

IJulia
    https://github.com/JuliaLang/IJulia.jl (`example notebook
    <http://nbviewer.ipython.org/url/jdj.mit.edu/~stevenj/IJulia%20Preview.ipynb>`_)

VisTrails
    http://www.vistrails.org/index.php/Main_Page

Sweave
    http://en.wikipedia.org/wiki/Sweave

knitr
    http://yihui.name/knitr/

Pweave
    http://mpastell.com/pweave/

ActivePapers
    http://dirac.cnrs-orleans.fr/plone/software/activepapers/
        * active_papers (JVM):
          https://bitbucket.org/khinsen/active_papers
        * active_papers_py (Python):
          https://bitbucket.org/khinsen/active_papers_py/wiki/Home

Online Services
---------------

IPython Notebook Viewer
    http://nbviewer.ipython.org/

Github
    http://github.com/

Bitbucket (free unlimited accounts for academic users)
    http://bitbucket.org/

figshare
    http://figshare.com/, `connecting Github and figshare <http://figshare.com/blog/Working_with_Github_and_Mozilla_to_enable_Code_as_a_Research_Output_/117>`_

zenodo
    http://zenodo.org/

ORCID
    http://orcid.org/

crossref
    http://crossref.org/

DataCite
    http://www.datacite.org/

my experiment
    http://www.myexperiment.org/

re3data (Registry of Research Data Repositories)
    http://www.re3data.org/

RADAR - Research Data Repository
    http://www.radar-projekt.org/display/RE/Home

Open Science Framework
    https://osf.io/

DataUp
    http://dataup.cdlib.org/

Journals
--------

F1000Research (life sciences)
    http://f1000research.com/

Scientific Data - nature.com (launching in May 2014)
    http://www.nature.com/scientificdata/

DRYAD
    http://datadryad.org/

Publications
------------

Patrick Vandewalle, Jelena Kovačević, Martin Vetterli,
`Reproducible Research in Signal Processing
<http://dx.doi.org/10.1109/MSP.2009.932122>`_,
IEEE Signal Processing Magazine Volume 26, Issue 3, 2009.

Robert Gentleman, Duncan Temple Lang,
`Statistical Analyses and Reproducible Research
<http://dx.doi.org/10.1198/106186007X178663>`_,
Journal of Computational and Graphical Statistics Volume 16, Issue 1, 2007.

Bruce G. Charlton,
`Peer usage versus peer review
<http://dx.doi.org/10.1136/bmj.39304.581574.94>`_,
BMJ Volume 335, Issue 7617, 2007.

Arturo Casadevall, Ferric C. Fang,
`Reproducible Science <http://dx.doi.org/10.1128/IAI.00908-10>`_,
Infection and Immunity Volume 78, Issue 12, 2010.

Jonathan B. Buckheit, David L. Donoho,
`WaveLab and Reproducible Research
<http://dx.doi.org/10.1007/978-1-4612-2544-7_5>`_,
in `Wavelets and Statistics <http://dx.doi.org/10.1007/978-1-4612-2544-7>`_,
Springer, 1995.

Darrel C. Ince, Leslie Hatton, John Graham-Cumming,
`The Case for Open Computer Programs <http://dx.doi.org/10.1038/nature10836>`_,
Nature Volume 482, 2012.

Nature special `Challenges in Irreproducible Research
<http://www.nature.com/nature/focus/reproducibility/>`_, 2010-2013

Links
-----

Coursera course about *Reproducible Research* (starting on May 5th 2014)
    https://www.coursera.org/course/repdata

results may vary (slides for keynote at ISMB/ECCB 2013)
    http://www.slideshare.net/carolegoble/ismb2013-keynotecleangoble

Reproducibility in Computational Science (slides)
    http://www.stanford.edu/~vcs/talks/UMN-Oct102013-STODDEN.pdf

The Role of Data Repositories in Reproducible Research:
    http://isps.yale.edu/news/blog/2013/07/the-role-of-data-repositories-in-reproducible-research

#solo13lego: Research Roles Through Lego
    http://sophiekershaw.wordpress.com/2013/11/14/research-roles-through-lego/

Reproducibility: An important altmetric
    http://altmetrics.org/altmetrics12/iorns/

The Truth Wears Off: An odd twist in the scientific method
    http://www.newyorker.com/reporting/2010/12/13/101213fa_fact_lehrer

Report reveals missteps in Duke cancer trial review
    http://blogs.nature.com/news/2011/01/report_reveals_missteps_in_ini.html

Reproducible Research in Signal/Image Processing
    http://reproducibleresearch.net/

European Commission: *Towards better access to scientific information*
    http://www.eesc.europa.eu/?i=portal.en.int-opinions.24976 (`PDF <http://eur-lex.europa.eu/LexUriServ/LexUriServ.do?uri=COM:2012:0401:FIN:EN:PDF>`_)

Preserving Research: The top online archives for storing your unpublished findings
    http://www.the-scientist.com/?articles.view/articleNo/36695/title/Preserving-Research/

Post-Publication Peer Review Mainstreamed
    http://www.the-scientist.com/?articles.view/articleNo/37969/title/Post-Publication-Peer-Review-Mainstreamed/

Offene Wissenschaft (de)
    http://www.offene-wissenschaft.de/

mozilla Science Lab
    http://mozillascience.org/

Panton Principles
    http://pantonprinciples.org/

Guide to Open Data Licensing
    http://opendefinition.org/guide/data/

CC0
    http://creativecommons.org/publicdomain/zero/1.0/

Joint Declaration of Data Citation Principles
    https://www.force11.org/node/4769

Madagascar
    http://reproducibility.org/

.. vim:textwidth=80:spell
