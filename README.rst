Otek
======

Otek is a command line tool to help you make new projects with ease.

Otek stores templates of projects so you don’t have to remake the same
file structure every time your start developing a new idea. Just make the
template once and keep getting started from that. Otek avoids being
opinonated because the user knows what project format is best. This is
different from projects like `yeoman`_ where the user has no control over their
project structure.

**What's with the name?**
"Otek" is a translitaterion of the hebrew word for "copy". I'm using hebrew because
using a different language is the only way to find a name that wasn't taken.

Installation
~~~~~~~~~~~~

Just use pip!

.. code:: bash

    pip install otek

.otek Folder
~~~~~~~~~~~~

In the `.otek` folder there's an `otekrc` which stores the defaults values for
variables in JSON format.  It supports full JSON.  To see how to use variables, read the next section.

otekrc Variables
~~~~~~~~~~~~~~~~

There are some variables predefined by otek.  Here are the default values of each variable.

`PROJNAME` : The name of the folder
`STARTEXPR` : `%>`
`ENDEXPR` : `%>`


Templates
~~~~~~~~~

Templates let you pre-process your projects with different variables.  First
create a template project in your `~/.otek` folder.  This will look like

.. code:: python

    # ~/.otek/example-project/main.py
    __author__ = '<% name %>'

    print('hello, world.  This is the <% PROJNAME %> ')

Next create a new project for your template to be created

.. code:: bash

     $ mkdir example && cd example
     $ otek create example-project

Now our main python file has subsituted the `name` for `Jacob Wunder`

.. code:: python

    # ~/Documents/example/main.py
    __author__ = 'Jacob Wunder'

    print('hello, world.  This is the example-project ')

To change values for certain variables on a project to project basis, just use
the `-v` flag followed by a variable name. WARNING: only works for top-level variables.

.. code:: bash

    otek create example-project -v name="Linus Torvalds"

Now value of `name` is 'Linus Torvalds' so the `__author__` will be Linus Torvalds

.. code:: python

    # ~/Documents/example/main.py
    __author__ = 'Linus Torvalds'

    print('hello, world.  This is the example-project ')

``create`` Script
~~~~~~~~~~~~~~~~~

Need more than files to create your project?  Just create a file called ``create``
which will be run upon creation time as a bash script.  This could be used to
run `npm init` or `npm install` if you already have a default package.json.

Roadmap
~~~~~~~

 -  Download other people’s project templates using github in a similar way to homebrew formulae
 -  Allow for a project-local otek folder to allow for otek templates specific to a project

.. _yeoman: http://yeoman.io
