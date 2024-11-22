Reproducible Research
=====================

.. note::

  This is, much like research itself, and the art of eating spaghetti without
  soiling yourself, work-in-progress.

This page is not as general as it should be.
It is biased towards audio signal processing, audio engineering, spatial audio
reproduction and auditory perception.
However, many of the ideas presented here can be applied more widely.

Other collections of similar information:

 * https://github.com/INRIA/awesome-open-science-software

 * https://danmackinlay.name/notebook/reproducible_research


Definitions
-----------

Openness
^^^^^^^^

.. todo:: The open definition

  http://opendefinition.org/

Definition by `Wikipedia <https://en.wikipedia.org/wiki/Open_science>`__:

  **Open science** is the movement to make scientific research, data and
  dissemination accessible to all levels of an inquiring society, amateur or
  professional. It encompasses practices such as publishing open research,
  campaigning for open access, encouraging scientists to practice open notebook
  science, and generally making it easier to publish and communicate scientific
  knowledge.  [...]
  In modern times there is debate about the extent to which scientific
  information should be shared. The conflict is between the desire of
  scientists to have access to shared resources versus the desire of individual
  entities to profit when other entities partake of their resources.

Definition by `Wikipedia <https://en.wikipedia.org/wiki/Open_research>`__:

  **Open research** is research conducted in the spirit of free and open source
  software. Much like open source schemes that are built around a source code
  that is made public, the central theme of open research is to make clear
  accounts of the methodology freely available via the internet, along with any
  data or results extracted or derived from them. This permits a massively
  distributed collaboration, and one in which anyone may participate at any
  level of the project.

  Especially if the research is scientific in nature, it is frequently referred
  to as *open science*. Open research can also include social sciences, the
  humanities, mathematics, engineering and medicine.

Definition by `Wikipedia <https://en.wikipedia.org/wiki/Open_data>`__:

  **Open data** is the idea that some data should be freely available to
  everyone to use and republish as they wish, without restrictions from
  copyright, patents or other mechanisms of control.  The goals of the open data
  movement are similar to those of other "open" movements such as open source,
  open hardware, open content, and open access.

Definition by `Wikipedia <https://en.wikipedia.org/wiki/Open_science_data>`__:

  **Open science data** is a type of open data focused on publishing
  observations and results of scientific activities available for anyone to
  analyze and reuse.

Definition by `Wikipedia <https://en.wikipedia.org/wiki/Open_notebook_science>`__:

  **Open notebook science** is the practice of making the entire primary record
  of a research project publicly available online as it is recorded. This
  involves placing the personal, or laboratory, notebook of the researcher
  online along with all raw and processed data, and any associated material, as
  this material is generated. The approach may be summed up by the slogan 'no
  insider information'. It is the logical extreme of transparent approaches to
  research and explicitly includes the making available of failed, less
  significant, and otherwise unpublished experiments; so called 'dark data'.

Definition by `Wikipedia <https://en.wikipedia.org/wiki/Open_access>`__:

  **Open access** (**OA**) refers to online research outputs that are free of
  all restrictions on access (e.g. access tolls) and free of many restrictions
  on use (e.g. certain copyright and license restrictions). Open access can be
  applied to all forms of published research output, including peer-reviewed
  and non peer-reviewed academic journal articles, conference papers, theses,
  book chapters, and monographs.

  Two degrees of open access can be distinguished: gratis open access, which is
  online access free of charge, and libre open access, which is online access
  free of charge plus various additional usage rights.


.. todo:: Reproducible Research vs. Non-Reproducible Research?

.. todo:: reproducible vs. easily reproducible

.. todo:: online material as supplement to traditional publications

.. todo:: https://en.wikipedia.org/wiki/Reproducibility

.. todo:: https://en.wikipedia.org/wiki/Open_research

`Vandewalle et al. <https://doi.org/10.1109/MSP.2009.932122>`_ distinguish six
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

Replicability vs. Reproducibility
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Great overview: `Language Log: Replicability vs. reproducibility — or is it the other way around? <https://web.archive.org/web/20210302204128/https://languagelog.ldc.upenn.edu/nll/?p=21956>`__

`Wikipedia <https://en.wikipedia.org/wiki/Reproducibility>`__ thinks it's the
same:

  **Reproducibility** is the ability of an entire experiment or study to be
  duplicated, either by the same researcher or by someone else working
  independently. Reproducing an experiment is called **replicating** it.
  Reproducibility is one of the main principles of the scientific method.

`Chris Drummond <http://cogprints.org/7691/>`__ claims they are different:

  Reproducibility requires changes; replicability avoids them. Although
  reproducibility is desirable, I contend that the impoverished version,
  replicability, is one not worth having.

`Roger D. Peng <https://doi.org/10.1093/biostatistics/kxp014>`__
also claims that they are different, but uses slightly different definitions:

  The replication of scientific findings using independent investigators,
  methods, data, equipment, and protocols has long been, and will continue to
  be, the standard by which scientific claims are evaluated. However, in many
  fields of study there are examples of scientific investigations that cannot be
  fully replicated because of a lack of time or resources. In such a situation,
  there is a need for a minimum standard that can fill the void between full
  replication and nothing. One candidate for this minimum standard is
  “reproducible research”, which requires that data sets and computer code be
  made available to others for verifying published results and conducting
  alternative analyses.

`Victoria Stodden <https://magazine.amstat.org/blog/2011/07/01/trust-your-science/>`__
defines them slightly differently (and throws in a third concept --
"repeatability"):

  We can reserve the term "replicability" for the regeneration of published
  results from author-provided code and data. [...] Reproducibility is a more
  general term, implying both replication and the regeneration of findings with
  at least some independence from the code and/or data associated with the
  original publication. Both refer to the analysis that occurs after
  publication. A third term, "repeatability," is sometimes used in place of
  reproducibility, but this is more typically used as a term of art referring to
  the sensitivity of results when underlying measurements are retaken.


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

  * https://creativecommons.org/choose/
  * http://three.org/openart/license_chooser/

  For more details, have a look there: https://tldrlegal.com/.
  
  Licensing your research, webinar with Brandon Butler: https://osf.io/6uupa/

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

Criticism
---------

Three points from
https://en.wikipedia.org/wiki/Open_notebook_science#Drawbacks:

#. data theft
#. not patentable once published
#. data deluge

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

   For more information, watch this: https://youtu.be/vt0Y39eMvpI

Scientific Python (SciPy)
    https://scipy.org/

    This is a collection of many software projects:
    `NumPy <https://numpy.org/>`_,
    `SciPy <https://scipy.org/scipylib/>`_,
    `matplotlib <https://matplotlib.org/>`_,
    `IPython <https://ipython.org/>`_,
    `SymPy <https://www.sympy.org/>`_,
    `pandas <https://pandas.pydata.org/>`_,
    `Mayavi <https://docs.enthought.com/mayavi/mayavi/>`_,
    `PyTables <https://www.pytables.org/>`_,
    and many more ...

See also my `introduction to Python, NumPy, IPython, ...
<https://nbviewer.jupyter.org/github/mgeier/python-audio/blob/master/index.ipynb>`_

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
    https://julialang.org/

Sage
    https://www.sagemath.org/

Publication Tools
-----------------

IPython
    http://ipython.org/

IJulia
    https://github.com/JuliaLang/IJulia.jl (`example notebook
    <https://nbviewer.jupyter.org/url/jdj.mit.edu/~stevenj/IJulia%20Preview.ipynb>`_)

VisTrails
    https://www.vistrails.org/index.php/Main_Page

Sweave
    https://en.wikipedia.org/wiki/Sweave

knitr
    https://yihui.org/knitr/

Pweave
    http://mpastell.com/pweave/

ActivePapers
    https://activepapers.github.io/
        * active_papers (JVM):
          https://activepapers.github.io/jvm-edition/
        * activepapers-python (Python):
          https://activepapers.github.io/python-edition/

Online Services
---------------

IPython/Jupyter Notebook Viewer
    https://nbviewer.jupyter.org/

Binder (Turn a GitHub repo into a collection of interactive notebooks)
    https://mybinder.org/

Github
    https://github.com/

Bitbucket (free unlimited accounts for academic users)
    https://bitbucket.org/

figshare
    https://figshare.com/, `connecting Github and figshare <https://figshare.com/blog/Working_with_Github_and_Mozilla_to_enable_Code_as_a_Research_Output_/117>`_

zenodo
    https://zenodo.org/

ORCID
    https://orcid.org/

crossref
    https://www.crossref.org/

DataCite
    https://datacite.org/

my experiment
    https://www.myexperiment.org/

re3data (Registry of Research Data Repositories)
    https://www.re3data.org/

RADAR - Research Data Repository
    https://www.radar-service.eu/en

Open Science Framework
    https://osf.io/

DataUp
    ``http://dataup.cdlib.org/``

Authorea
    https://www.authorea.com/

PubPeer (post publication peer review)
    https://pubpeer.com/

PubMed Commons (post publication peer review)
    ``https://www.ncbi.nlm.nih.gov/pubmedcommons/`` (discontinued, see
    https://ftp.ncbi.nlm.nih.gov/pubmed/pubmedcommons/README.txt)

CKAN (Open Source data portal platform)
    https://ckan.org/

sciety (curated preprints)
    https://sciety.org/

Peer Community in
    https://peercommunityin.org/

Journals
--------

F1000Research (life sciences)
    https://f1000research.com/

Scientific Data - nature.com
    https://www.nature.com/sdata/

DRYAD
    https://datadryad.org/

The ReScience Journal
    http://rescience.github.io/

Peer Community Journal
    https://peercommunityjournal.org/


Publications
------------

Patrick Vandewalle, Jelena Kovačević, Martin Vetterli,
`Reproducible Research in Signal Processing
<https://doi.org/10.1109/MSP.2009.932122>`_,
IEEE Signal Processing Magazine Volume 26, Issue 3, 2009.

Robert Gentleman, Duncan Temple Lang,
`Statistical Analyses and Reproducible Research
<https://doi.org/10.1198/106186007X178663>`_,
Journal of Computational and Graphical Statistics Volume 16, Issue 1, 2007.

Bruce G. Charlton,
`Peer usage versus peer review
<https://doi.org/10.1136/bmj.39304.581574.94>`_,
BMJ Volume 335, Issue 7617, 2007.

Arturo Casadevall, Ferric C. Fang,
`Reproducible Science <https://doi.org/10.1128/IAI.00908-10>`_,
Infection and Immunity Volume 78, Issue 12, 2010.

Jonathan B. Buckheit, David L. Donoho,
`WaveLab and Reproducible Research
<https://doi.org/10.1007/978-1-4612-2544-7_5>`_,
in `Wavelets and Statistics <https://doi.org/10.1007/978-1-4612-2544-7>`_,
Springer, 1995.

Darrel C. Ince, Leslie Hatton, John Graham-Cumming,
`The Case for Open Computer Programs <https://doi.org/10.1038/nature10836>`_,
Nature Volume 482, 2012.

Nature special `Challenges in Irreproducible Research
<https://web.archive.org/web/20170802213155/http://www.nature.com/news/reproducibility-1.17552>`_, 2010-2013.

Fernando Pérez, Brian E. Granger, John D. Hunter,
`Python: An Ecosystem for Scientific Computing
<https://doi.org/10.1109/MCSE.2010.119>`_,
Computing in Science Engineering, Volume 13, Issue 2, 2011.

Peter Suber,
`Open Access <https://mitpress.mit.edu/books/open-access>`_,
MIT Press, 2012.

Peter Suber,
`Gratis and libre open access <https://web.archive.org/web/20230810233553/https://dash.harvard.edu/bitstream/handle/1/4322580/suber_oagratis.html>`__,
SPARC Open Access Newsletter, issue #124, 2008.

Peter Suber,
`Knowledge Unbound: Selected Writings on Open Access, 2002–2011
<https://library.oapen.org/handle/20.500.12657/26045>`_,
MIT Press, 2016.

John P. A. Ioannidis,
`Why Most Published Research Findings Are False
<https://doi.org/10.1371/journal.pmed.0020124>`_,
PLoS Med 2(8): e124. doi:10.1371/journal.pmed.0020124, 2005.

Detailed comment to the above:
http://matthew-brett.github.io/teaching/ioannidis_2005.html

Chris Drummond,
`Replicability is not Reproducibility: Nor is it Good Science
<http://cogprints.org/7691/>`__,
Proc. of the Evaluation Methods for Machine
Learning Workshop at the 26th ICML, 2009.

Ian P. Gent,
`The Recomputation Manifesto
<https://arxiv.org/abs/1304.3674v1>`__,
Unpublished position paper, Version 1.9479, 2013.

Michael Woelfle, Piero Olliaro, Matthew H. Todd,
`Open science is a research accelerator <https://doi.org/10.1038/nchem.1149>`__,
Nature Chemistry, Volume 3, Issue 10, 2011.

Radovan Vrana,
`Open science, open access and open educational resources: Challenges and opportunities <https://doi.org/10.1109/MIPRO.2015.7160399>`__,
International Convention on Information and Communication Technology, Electronics and Microelectronics (MIPRO), 2015.

Yale Law School Roundtable on Data and Code Sharing,
`Reproducible Research: Addressing the Need for Data and Code Sharing in Computational Science <https://doi.org/10.1109/MCSE.2010.113>`__,
Computing in Science & Engineering, Volume 12, Issue 5, 2010.

Toronto International Data Release Workshop Authors,
`Prepublication Data Sharing <https://doi.org/10.1038/461168a>`__,
Nature 461, no. 7261, 2009.

Rinze Benedictus, Frank Miedema, and Mark W. J. Ferguson,
`Fewer Numbers, Better Science <https://doi.org/10.1038/538453a>`__,
Nature News, Volume 538, Issue 7626, 2016.

J. Wilsdon et al.,
`The Metric Tide: Report of the Independent Review of the Role of 
Metrics in Research Assessment and Management <https://doi.org/10.13140/RG.2.1.4929.1363>`__,
2015.

Barak A. Cohen,
`Point of View: How should novelty be valued in science?
<https://doi.org/10.7554/eLife.28699>`__,
2017.

D. Cicchetti,
`The reliability of peer review for manuscript and grant submissions: A
cross-disciplinary investigation <https://doi.org/10.1017/S0140525X00065675>`__,
1991.

J. Bollen et al.,
`From funding agencies to scientific agency <https://doi.org/10.1002/embr.201338068>`__,
2014.

J. Bollen et al.,
`An efficient system to fund science: from proposal review to peer-to-peer
distributions <https://doi.org/10.1007/s11192-016-2110-3>`__,
2017.

B. Alberts et al.,
`Self-Correction in Science at Work
<https://doi.org/10.1126/science.aab3847>`__,
Science Vol. 348, Issue 6242, pp. 1420-1422,
2015

B. A. Nosek et al.,
`Promoting an Open Research Culture
<https://doi.org/10.1126/science.aab2374>`__,
Science Vol. 348, Issue 6242, pp. 1422-1425,
2015

Mary C. Murphy et al.,
`Open science, communal culture, and women’s participation
in the movement to improve science
<https://doi.org/10.1073/pnas.1921320117>`__,
Proceedings of the National Academy of Sciences,
2020

Thomas H. Berquist,
`Peer Review: Is the Process Broken?
<https://doi.org/10.2214/AJR.12.9256>`__,
American Journal of Roentgenology, Volume 199, Issue 2,
2012

Melinda Baldwin,
`Peer Review <https://doi.org/10.34758/srde-jw27>`__,
Encyclopedia of the History of Science
2020

Links
-----

Coursera course about *Reproducible Research*
    https://www.coursera.org/learn/reproducible-research

results may vary (slides for keynote at ISMB/ECCB 2013)
    https://www.slideshare.net/carolegoble/ismb2013-keynotecleangoble

Reproducibility in Computational Science (slides)
    ``https://web.stanford.edu/~vcs/talks/UMN-Oct102013-STODDEN.pdf``

The Role of Data Repositories in Reproducible Research:
    https://isps.yale.edu/news/blog/2013/07/the-role-of-data-repositories-in-reproducible-research

#solo13lego: Research Roles Through Lego
    https://sophiekershaw.wordpress.com/2013/11/14/research-roles-through-lego/

Reproducibility: An important altmetric
    ``http://altmetrics.org/altmetrics12/iorns/``

The Truth Wears Off: An odd twist in the scientific method
    https://www.newyorker.com/magazine/2010/12/13/the-truth-wears-off

Report reveals missteps in Duke cancer trial review
    http://blogs.nature.com/news/2011/01/report_reveals_missteps_in_ini.html

Reproducible Research in Signal/Image Processing
    http://reproducibleresearch.net/

European Commission: *Towards better access to scientific information*
    https://www.eesc.europa.eu/?i=portal.en.int-opinions.24976 (`PDF <https://eur-lex.europa.eu/LexUriServ/LexUriServ.do?uri=COM:2012:0401:FIN:EN:PDF>`_)

Preserving Research: The top online archives for storing your unpublished findings
    https://www.the-scientist.com/careers/preserving-research-38930

Post-Publication Peer Review Mainstreamed
    https://www.the-scientist.com/daily-news/post-publication-peer-review-mainstreamed-38529

Offene Wissenschaft (de)
    https://web.archive.org/web/20180706043644/http://www.offene-wissenschaft.de/

mozilla Science Lab
    https://wiki.mozilla.org/ScienceLab

Panton Principles
    https://web.archive.org/web/20220921042649/https://pantonprinciples.org/

The Open Definition
    http://opendefinition.org/

Guide to Open Data Licensing
    http://opendefinition.org/guide/data/

CC0
    https://creativecommons.org/publicdomain/zero/1.0/

Joint Declaration of Data Citation Principles
    https://doi.org/10.25490/a97f-egyk

Madagascar
    http://www.ahay.org/wiki/Main_Page

Reproducibility Initiative
    ``http://reproducibilityinitiative.org/``

The Need for Openness in Data Journalism
    https://nbviewer.jupyter.org/github/brianckeegan/Bechdel/blob/master/Bechdel_test.ipynb

Guidelines for Open Educational Resources (OER) in Higher Education
    ``http://www.unesco.org/new/en/communication-and-information/resources/publications-and-communication-materials/publications/full-list/guidelines-for-open-educational-resources-oer-in-higher-education/``
    http://oasis.col.org/handle/11599/60

10 Simple Rules for the Care and Feeding of Scientific Data
    https://www.authorea.com/users/3/articles/3410/_show_article

Scientific Python Lectures:
    https://github.com/jrjohansson/scientific-python-lectures

Research Objects
    https://en.wikipedia.org/wiki/Research_Objects

An efficient workflow for reproducible science (SciPy 2013)
    https://youtu.be/Y-XFNg0QS14

Open Glossary
    https://blogs.egu.eu/network/palaeoblog/files/2015/02/OpenGlossary1.pdf

Open Access: Berlin Declaration
    https://openaccess.mpg.de/Berlin-Declaration,
    `Wikipedia article <https://en.wikipedia.org/wiki/Berlin_Declaration_on_Open_Access_to_Knowledge_in_the_Sciences_and_Humanities>`__

Reproducibility in Code and Science
    https://web.archive.org/web/20170903071534/http://justingosses.com/reproducibility/

The 7 biggest problems facing science, according to 270 scientists
    https://www.vox.com/2016/7/14/12016710/science-challeges-research-funding-peer-review-process

Journal of Articles in Support of the Null Hypothesis
    https://www.jasnh.com/

The Transparency and Openness Promotion Guidelines
    https://www.cos.io/initiatives/top-guidelines

épisciences
    https://www.episciences.org/

The open archive HAL
    https://hal.archives-ouvertes.fr/

arXiv.org
    https://arxiv.org/

Directory of Open Access Journals (DOAJ)
    https://doaj.org/

Amsterdam Call for Action on Open Science
    https://web.archive.org/web/20170619030655/https://english.eu2016.nl/documents/reports/2016/04/04/amsterdam-call-for-action-on-open-science

Reproducibility and reliability of biomedical research
    https://acmedsci.ac.uk/policy/policy-projects/reproducibility-and-reliability-of-biomedical-research/

Rigor and Reproducibility (NIH guidelines)
    ``https://grants.nih.gov/reproducibility/index.htm``

Analysis of meta-analyses identifies where sciences' real problems lie
    https://arstechnica.com/science/2017/03/bias-in-science-small-samples-isolated-scientists-and-dodgy-individuals/

Vienna Principles
    http://viennaprinciples.org/

sciencecodemanifesto.org
    https://web.archive.org/web/20160218093215/http://sciencecodemanifesto.org/

Peer Reviewers' Openness Initiative
    https://www.opennessinitiative.org/

Initiative for Open Citations
    https://i4oc.org/

Workshop: Reproducible Research using Jupyter Notebooks
    https://reproducible-science-curriculum.github.io/workshop-RR-Jupyter/

ACM Artifact Review and Badging
    https://www.acm.org/publications/policies/artifact-review-badging

Science is "show me," not "trust me"
    https://www.bitss.org/science-is-show-me-not-trust-me/

An unhealthy obsession with p-values is ruining science
    https://www.vox.com/2016/3/15/11225162/p-value-simple-definition-hacking

The Irreproducibility Crisis of Modern Science: Causes, Consequences, and the Road to Reform
    https://www.nas.org/reports/the-irreproducibility-crisis-of-modern-science

    https://www.nas.org/storage/app/media/Reports/Irreproducibility%20Crisis%20Report/NAS_irreproducibilityReport.pdf

Why I've lost faith in p values
    https://lucklab.ucdavis.edu/blog/2018/4/19/why-i-lost-faith-in-p-values

Budapest Open Access Initiative
   https://www.budapestopenaccessinitiative.org/

FAIR Principles (Findable, Accessible, Interoperable, Re-usable)
   https://www.nature.com/articles/sdata201618

   https://www.go-fair.org/fair-principles/

   https://www.force11.org/group/fairgroup/fairprinciples

   https://www.force11.org/fairprinciples

The Turing Way: Guide for Reproducible Research
   https://the-turing-way.netlify.app/reproducible-research/reproducible-research.html

ASAPbio
   https://asapbio.org/

eLife's New Model: Changing the way you share your research
   https://elifesciences.org/inside-elife/54d63486/elife-s-new-model-changing-the-way-you-share-your-research

Project Free Our Knowledge
   https://freeourknowledge.org/

EU Council calls for transparent, equitable, and open access to scholarly publications
   https://www.consilium.europa.eu/en/press/press-releases/2023/05/23/council-calls-for-transparent-equitable-and-open-access-to-scholarly-publications/

The Rise of Peer Review: Melinda Baldwin on the History of Refereeing at Scientific Journals and Funding Bodies
   https://scholarlykitchen.sspnet.org/2018/09/26/the-rise-of-peer-review-melinda-baldwin-on-the-history-of-refereeing-at-scientific-journals-and-funding-bodies/

Peer Review -- A Historical Perspective
   https://mitcommlab.mit.edu/broad/commkit/peer-review-a-historical-perspective/

The Birth of Modern Peer Review
   https://blogs.scientificamerican.com/information-culture/the-birth-of-modern-peer-review/

Is the staggeringly profitable business of scientific publishing bad for science?
   https://amp.theguardian.com/science/2017/jun/27/profitable-business-scientific-publishing-bad-for-science

.. vim:textwidth=80:spell
