.. highlight:: sh

Jupyter Notebooks in a Git Repository
=====================================

It is a very nice feature of Jupyter__ notebooks that cell outputs
(e.g. images, plots, tables with data) can be stored within notebooks.
This makes it very easy to share notebooks with other people,
who can open the notebooks and can immediately see the results,
without having to execute the notebook
(which might have some complicated library or data dependencies,
or it might simply take a long time to run).

__ https://jupyter.org/

However, those cell outputs can be very annoying when using a
`version control system`__ like e.g. Git__.
Whenever a change is made to a code cell,
most likely the cell's output will also change.
The problem is that both changes will be shown in a "diff" view,
but the (often much larger) changes in outputs
will distract from the much more interesting changes in the code.
This can make it very tedious to work on a notebook with multiple people.

__ https://en.wikipedia.org/wiki/Version_control
__ https://git-scm.com/

To avoid this problem, it is recommended to strip all outputs from a notebook
before committing it to Git.

.. todo::

   clean/smudge filters
   (https://nbsphinx.readthedocs.io/en/latest/usage.html#Using-Notebooks-with-Git)

There is a catch, though.
If you don't commit the cell outputs,
other people looking at your repository don't see the outputs
(unless they execute the notebooks on their own).
But don't worry,
the next two sections will show two ways to work with "clean" notebooks
but still to be able to share the cell outputs publicly.


Executing Notebooks on a Server
-------------------------------

.. todo::

   https://nbsphinx.readthedocs.io/ and others

.. todo::

   advantages, disadvantages 


Executing Notebooks in a Separate Branch
----------------------------------------

In this scenario,
you and your collaborators mainly work on the ``dev`` branch
(never committing cell outputs),
and the ``master`` branch only contains a single additional commit
in which all notebooks are executed.

If you want to use pull requests for collaboration,
those should always be based upon the ``dev`` branch.

.. todo::

   advantages, disadvantages 


Getting Started from Scratch
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

This assumes that you have no Jupyter notebooks in your repository yet
or all your notebooks are "clean" (i.e. stored without outputs).

#. Make sure there are no un-committed (and un-pushed) local changes
#. Create a new branch called ``dev`` (starting at the ``master`` branch) and
   switch to the new branch::

      git checkout -b dev master

#. Push the new ``dev`` branch to the server::

      git push --set-upstream origin dev

#. If you already have some notebooks, you can execute them now in the
   ``master`` branch::

      git checkout master

   You can execute the notebooks one by one,
   or all at once, as described in `Executing All Notebooks`_.
   Once all the notebooks you want to execute are executed,
   commit the changes to the ``master`` branch::

      git commit -a -m "Execute notebooks"

   And if everything looks OK, you can push the new commit to the server::

      git push

That's it!

When you want to start changing existing notebooks,
continue with the section `Making a Change`_.


Getting Started with Pre-executed Notebooks
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Don't worry if you have previously committed notebooks
without stripping their outputs.
There is still a way of getting rid of them retroactively
by re-writing your Git history.

#. Make sure there are no un-committed (and un-pushed) local changes
#. Create a new branch called ``dev`` (starting at the ``master`` branch) and
   switch to the new branch::

      git checkout -b dev master

#. Do the steps listed in the section `Cleaning a Whole Repository`_
#. Push the changes from the ``dev`` branch to the server::

      git push --set-upstream origin dev

#. Switch back to the ``master`` branch and make a backup branch::

      git checkout master
      git branch backup

   .. note::

      If you think you might need it later (or if you are somewhat paranoid),
      you can also push the new ``backup`` branch to the server.

#. Reset the ``master`` branch to point to the same commit as ``dev``::

      git reset dev --hard

   .. warning::

      With this step you throw away all your old commits!
      But you can still use the ``backup`` branch to get them back.

#. Get the executed version of all notebooks (don't forget the dot!)::

      git checkout backup .

#. Create a new commit with a commit message like "Execute notebooks"::

      git commit -m "Execute notebooks"

#. If you are satisfied with the result,
   you can push your changes to the server,
   but note that you have to use ``--force``,
   because you changed the Git history::

      git push --force

   .. warning::

      At this point, you are deleting all your old commits from the server!
      If you want to keep them, you should also push the ``backup`` branch.


Making a Change
^^^^^^^^^^^^^^^

#. Switch to the ``dev`` branch::

      git checkout dev

#. Work on your notebooks

#. Create one or more commits with new notebooks or changes to existing ones

#. Push the ``dev`` branch to the server::

      git push

#. Switch to the ``master`` branch and re-base it onto ``dev``::

      git checkout master
      git rebase -X ours dev

   .. note::

      The parameter ``-X ours`` selects a merging strategy where
      the changes to ``dev`` are preferred over the changes to ``master``.

   Special care has to be taken before re-basing when notebooks are removed::

      git checkout master
      git rm the-deleted-notebook.ipynb the-other-deleted-notebook.ipynb
      git commit --amend
      git rebase -X ours dev

#. Manually (re-)run the changed (and any new) notebooks.

   You can execute the notebooks in the Jupyter application,
   or you can execute them with ``nbconvert``::

      python3 -m nbconvert --execute --inplace my-notebook.ipynb my-other-notebook.ipynb

   If you have many notebooks, it might be hard to keep in mind
   which ones you have changed.
   To get list of changed notebooks (but also other changed files),
   you can use this command::

      git diff --name-only dev $(git merge-base dev origin/master)

#. When all changed notebooks have been executed,
   you can update the "Execute notebooks" commit::

      git commit -a --amend

#. In the end, the changes to ``master`` have to be force-pushed::

      git push --force

   .. note::

      *Normally*, you should *never* use ``git push --force``
      on the ``master`` branch.
      However, this is a special case where it's OK,
      because all actual work will be done on the ``dev`` branch.
      This means that you should *never* use ``git push --force``
      on the ``dev`` branch!


Executing All Notebooks
-----------------------

To execute all notebooks (whether they have outputs in them or not),
you can use::

   python3 -m nbconvert --execute --inplace *.ipynb **/*.ipynb

To disable the timeout, add ``--ExecutePreprocessor.timeout=-1`` to the command.
This should actually be the default, but it's not,
see https://github.com/jupyter/nbconvert/issues/791.

Please note the two *globbing* patterns used here.
The second pattern (``**/*.ipynb``) is collecting all the notebooks recursively,
but it doesn't include the files in the current directory.
That's what the first pattern (``*.ipynb``) is used for.
If you don't have notebooks in the main directory, you should omit this pattern.

In a future release of ``nbconvert`` the second pattern might become superfluous.


Cleaning All Notebooks
----------------------

Removing outputs from all notebooks should work with this command::

   python3 -m nbconvert --clear-output *.ipynb **/*.ipynb

... except that ``--clear-output`` is currently broken,
see https://github.com/jupyter/nbconvert/issues/822.

It should work with the slightly more verbose::

   python3 -m nbconvert --ClearOutputPreprocessor.enabled=True --inplace *.ipynb **/*.ipynb


Cleaning a Whole Repository
---------------------------

Make sure you don't have any local changes and no un-committed files!

You might want to create a new branch (and switch to it) before doing this!

Cleaning the whole Git history of the current branch::

   git filter-branch --tree-filter "python3 -m nbconvert --ClearOutputPreprocessor.enabled=True --inplace *.ipynb **/*.ipynb"

If there are some commits without Jupyter notebook in them, you might want to
extend the command a bit (to ignore any errors)::

   git filter-branch --tree-filter "python3 -m nbconvert --ClearOutputPreprocessor.enabled=True --inplace *.ipynb **/*.ipynb || true"

Depending on the size of your repository and the number of commits,
this might take a while ...
