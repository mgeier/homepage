.. highlight:: sh

Getting Started with Git
========================

There are a lot of pages on the web dedicated to Git, this page just shows a few
random bits of information which might or might not help you getting started.

Git is a distributed version control system (DVCS) and it's great!

That's about all I'm gonna say about it in general, if you want to read more
about it, have a look at the links at the end of this page.

Cloning a Repository
--------------------

To obtain the contents of a remote repository, you have to *clone* it.
For example, to clone the repository where this very page was created from, do::

    git clone https://github.com/mgeier/homepage.git

This will create a directory named like the repository (in this case
:file:`homepage/`). ::

    cd homepage

Checking the Status
-------------------

At any time, you can get the current status of your files with::

    git status

Here you will see if there are changes to any files of the repository and if
there are local files which are not yet part of the repository (see below for
how to add files).

Getting Recent Changes from the Server
--------------------------------------

If the repository on the server changed since you cloned it, you can get up to
date with::

    git pull

But be aware that if you made changes to your local files, this may lead to
conflicts. It's a good idea to always commit before pulling (see below how to
commit changes).

Initial Setup
-------------

Before you make your first commit, you should set up a few things (you have to
do this only once)::

    git config --global user.name "Your Name"
    git config --global user.email you@example.com

You may also want to set your favorite editor::

    git config --global core.editor "gvim --nofork"

To enable colors in ``git log`` et al.::

    git config --global color.ui auto

And, if you want, you can set a few aliases for your convenience::

    git config --global alias.s "status --short"
    git config --global alias.c "checkout"

    git config --global alias.lol "log --oneline --graph --decorate"
    git config --global alias.lola "log --oneline --graph --decorate --all"

All these options are stored in :file:`~/.gitconfig` (or somewhere else
depending on your operating system).

Command Line Prompt
-------------------

When working with branches, it is crucial to know the currently active branch.
To show the current branch in the command line prompt, put this at the end of
your :file:`~/.bashrc` (or wherever else you store settings for your shell)::

    # add git branch to prompt
    GIT_PS1_SHOWDIRTYSTATE=1
    PS1="${PS1%'\$ '}"'$(__git_ps1 " (%s)")'"\$ "

You'll also see an asterisk, e.g. ``(master *)``, if your working directory is
*dirty*, i.e. if you have local changes which are not yet committed.

Committing Changes
------------------

Any new files have to be added to Git control before you can do anything with
them::

    git add myfile.txt

Once you have done some changes, you can make a commit::

    git commit -a

Here, your favorite text editor will be opened and you can (and *should*!) enter
a commit message, describing the changes you have made. After you save the
file and close the editor, the commit will actually be created.

A commit message should have a short (no more than 50 characters) one-line
summary in the first line, then a blank line and then a more detailed
description. Have a look at this `note about commit messages
<http://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html>`_.

Remember, a commit is a local operation in Git, so nothing was transferred to
the server yet.

.. todo:: add before commit, staging area, local commits

Pushing Your Changes to the Server
----------------------------------

After one or several commits, you can push everything to the server::

    git push

If you created a new online repository and cloned the empty repository, you
have to use this command the first time to set up the ``master`` branch::

    git push origin master

After that, ``git push`` will suffice.

Ignoring Local Files
--------------------

.. todo:: :file:`.gitignore`

Attributes
----------

You can set per-file (or per-path) attributes if you create a file named
:file:`.gitattributes`, for example like this:

.. code-block:: none

    *.bib diff=bibtex
    *.cpp diff=cpp
    *.h diff=cpp
    *.htm diff=html
    *.html diff=html
    *.java diff=java
    *.php diff=php
    *.py diff=python
    *.rb diff=ruby
    *.tex diff=tex
    *.pbxproj binary

GUIs for Git
------------

There are many GUIs for Git to choose from; I personally like *gitg* (available
as Debian package with the same name) most but there are many more available
(see http://git-scm.com/downloads/guis).

Getting Help
------------

To get help just use::

    git help

You'll get something like this:

.. code-block:: none

    The most commonly used git commands are:
       add        Add file contents to the index
       bisect     Find by binary search the change that introduced a bug
       branch     List, create, or delete branches
       checkout   Checkout a branch or paths to the working tree
       clone      Clone a repository into a new directory
       commit     Record changes to the repository
       diff       Show changes between commits, commit and working tree, etc
       fetch      Download objects and refs from another repository
       grep       Print lines matching a pattern
       init       Create an empty Git repository or reinitialize an existing one
       log        Show commit logs
       merge      Join two or more development histories together
       mv         Move or rename a file, a directory, or a symlink
       pull       Fetch from and integrate with another repository or a local branch
       push       Update remote refs along with associated objects
       rebase     Forward-port local commits to the updated upstream head
       reset      Reset current HEAD to the specified state
       rm         Remove files from the working tree and from the index
       show       Show various types of objects
       status     Show the working tree status
       tag        Create, list, delete or verify a tag object signed with GPG
    
    See 'git help <command>' or 'git help <concept>' to read about a specific
    subcommand or concept.

Git and Subversion (SVN)
------------------------

See http://git-scm.com/book/en/Git-and-Other-Systems-Git-and-Subversion

Public Git Hosting Sites
------------------------

There are several free Git hosting services available, for an overview visit
https://git.wiki.kernel.org/index.php/GitHosting

More Documentation/Links
------------------------

* The Pro Git Book (CC license): http://book.git-scm.com/
* Understanding Git Conceptually: http://www.eecs.harvard.edu/~cduan/technical/git/
* Git Quick Reference: http://jonas.nitro.dk/git/quick-reference.html
* Git Immersion: http://gitimmersion.com/
* ...

There are many different strategies and methodologies how to use Git, just have a look with your favorite search engine or try this:

* http://nvie.com/posts/a-successful-git-branching-model/
* http://betterexplained.com/articles/aha-moments-when-learning-git/
* ...

There are also some nice videos:

* beginner
    * http://www.youtube.com/watch?v=4XpnKHJAok8
    * http://www.youtube.com/watch?v=ZDR433b0HJY
    * http://www.youtube.com/watch?v=GYnOwPl8yCE

* intermediate
    * http://www.youtube.com/watch?v=Z2ZL14WWEJI

* advanced
    * http://blip.tv/scott-chacon/git-tips-4232122

TODO
----

I probably should write about these, too:

* creating branches and switching between them
* pushing and pulling branches
* adding remotes
* merging
* rebasing
* interactive rebasing
* cherry-picking
* ``git stash``

.. vim:textwidth=80
