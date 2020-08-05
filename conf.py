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

html_add_permalinks = '\N{SECTION SIGN}'
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
