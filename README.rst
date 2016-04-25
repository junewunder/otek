Otek
======

Otek is a command line tool to help you make new projects with ease.

Otek stores templates of projects so you don’t have to remake the same
file structure every time your start a new project. Just make the
template once and keep starting from that. Otek avoids being
opinonated because the user knows what project format is best. This is
different from projects like `yeoman`_ in that yeoman is completely
opionated whereas otek has no opinions.

Installation
~~~~~~~~~~~~

**WORKING ON THAT NOW**

Templates
~~~~~~~~~

Templates let you pre-process your projects with different variables.

.. code:: python

    # ~/.otek/example-project/main.py
    __author__ = '<% name %>'

    print('hello, world.  This is the <% PROJNAME %> ')

.. code:: bash

     $ mkdir example && cd example
     $ otek create example-project

.. code:: python

    # ~/Documents/example/main.py
    __author__ = 'Jacob Wunder'

    print('hello, world.  This is the example-project ')

OR run: ``proj create example-project -v name="NOT JACOB WUNDER"``

.. code:: python

    # ~/Documents/example/main.py
    __author__ = 'NOT JACOB WUNDER'

    print('hello, world.  This is the example-project ')

Create script
~~~~~~~~~~~~~

Roadmap
~~~~~~~

-  Download other people’s project templates using github

.. _yeoman: http://yeoman.io
