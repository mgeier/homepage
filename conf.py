# Configuration file for Sphinx,
# see https://www.sphinx-doc.org/en/master/usage/configuration.html

extensions = [
    'sphinx.ext.todo',
    'nbsphinx',
    'sphinx_last_updated_by_git',
]

todo_include_todos = True

templates_path = ['_templates']

project = u'homepage'

exclude_patterns = ['_build']

default_role = 'any'

highlight_language = 'none'

nbsphinx_prolog = r"""
{% set docname = env.doc2path(env.docname, base=None) %}

.. raw:: html

    <div class="admonition note">
      This page was generated from
      <a class="reference external" href="https://github.com/mgeier/homepage/blob/{{ env.config.release|e }}/{{ docname|e }}">{{ docname|e }}</a>.
      Interactive online version:
      <span style="white-space: nowrap;"><a href="https://mybinder.org/v2/gh/mgeier/homepage/{{ env.config.release|e }}?filepath={{ docname|e }}"><img alt="Binder badge" src="https://mybinder.org/badge_logo.svg" style="vertical-align:text-bottom"></a>.</span>
    </div>
"""

linkcheck_ignore = [
    # [SSL: CERTIFICATE_VERIFY_FAILED]: unable to get local issuer certificate
    'https://www.vistrails.org',
    'https://scipy.org/',

    # ConnectionResetError(104, ...)
    'https://doi.org/10.1093/biostatistics',

    # [Errno 110] Connection timed out
    'http://www.soundfieldsynthesis.org',

    # 500 Server Error: Internal Server Error for url:
    'http://blogs.nature.com/news/2011/01/report_reveals_missteps_in_ini.html',

    # 403 Client Error: Forbidden for url: https://f1000research.com/
    'https://f1000research.com/',

    # 403 Client Error: Forbidden for url: https://www.researchgate.net/publication/...
    'https://doi.org/10.13140/RG.2.1.4929.1363',

    # Since many of those give linkcheck error but they work in the browser,
    # let's just ignore all of them:
    'https://doi.org/',
    # Also, the point of DOIs is that they don't change, so we really shouldn't have to check them.

    # 403 Client Error
    'https://www.eesc.europa.eu/?i=portal.en.int-opinions.24976',

    # https://github.com/vim/vim/issues/13079
    'https://www.vim.org/',

    # 403 Client Error
    'https://www.consilium.europa.eu/en/press/press-releases/2023/05/23/council-calls-for-transparent-equitable-and-open-access-to-scholarly-publications/',

    'https://magazine.amstat.org/',

    'https://www.upf.edu/web/mtg/phenicx-anechoic',
]

# -- Get version information and date from Git ----------------------------

try:
    from subprocess import check_output
    release = check_output(['git', 'describe', '--tags', '--always'])
    release = release.decode().strip()
    today = check_output(['git', 'show', '-s', '--format=%ad', '--date=short'])
    today = today.decode().strip()
except Exception:
    release = '<unknown>'
    today = '<unknown date>'

# -- Options for HTML output ---------------------------------------------------

html_theme = 'insipid'
html_title = project

html_permalinks_icon = '\N{SECTION SIGN}'
html_favicon = 'favicon.svg'
html_copy_source = False
html_use_index = False

# -- Options for LaTeX output --------------------------------------------------

latex_elements = {
#'papersize': 'letterpaper',
#'pointsize': '10pt',
#'preamble': '',
}

# (source start file, target name, title, author, documentclass [howto/manual]).
latex_documents = [
  ('index', 'homepage.tex', project, u'Matthias Geier', 'manual'),
]
