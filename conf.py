# Configuration file for Sphinx,
# see https://www.sphinx-doc.org/en/master/usage/configuration.html

extensions = [
    #'sphinx.ext.mathjax',
    'sphinx.ext.todo',
    'nbsphinx',
    'sphinx_last_updated_by_git',
]

todo_include_todos = True

templates_path = ['_templates']

project = u'homepage'
#html_show_copyright = False
copyright = u'Creative Commons CC0 - http://creativecommons.org/publicdomain/zero/1.0/. To the extent possible under law, Matthias Geier has waived all copyright and related or neighboring rights to this work'

exclude_patterns = ['_build']

default_role = 'any'

highlight_language = 'none'

nbsphinx_prolog = r"""
{% set docname = env.doc2path(env.docname, base='') %}

.. only:: html

    .. role:: raw-html(raw)
        :format: html

    .. nbinfo::

        This page was generated from `{{ docname }}`__.
        Interactive online version: :raw-html:`<a href="https://mybinder.org/v2/gh/mgeier/homepage/{{ env.config.release }}?filepath={{ docname }}"><img alt="Binder badge" src="https://mybinder.org/badge_logo.svg" style="vertical-align:text-bottom"></a>`

    __ https://github.com/mgeier/homepage/blob/
        {{ env.config.release }}/{{ docname }}
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
html_theme_options = {
}
html_title = project

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
