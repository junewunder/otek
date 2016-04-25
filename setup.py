"""Packaging settings."""


from codecs import open
import os
import shutil
from os.path import abspath, dirname, join
from subprocess import call

from setuptools import find_packages, setup

from otek import __version__


this_dir = abspath(dirname(__file__))
with open(join(this_dir, 'README.rst'), encoding='utf-8') as file:
    long_description = file.read()

# create home folder
if not os.path.exists(os.environ['HOME'] + '/.otek/'):
    dotOtek = __file__.replace('setup.py', 'dotOtek')
    shutil.copytree(dotOtek, os.environ['HOME'])

setup(
    name='otek',
    version=__version__,
    description='An unopinionated project builder for everyone.',
    long_description=long_description,
    url='https://github.com/jaywunder/otek',
    author='Jacob Wunder',
    author_email='public@jacobwunder.com',
    license='MIT',
    classifiers=[
        'Intended Audience :: Developers',
        'Topic :: Utilities',
        'License :: OSI Approved :: MIT License',
        'Natural Language :: English',
        'Operating System :: OS Independent',
        'Programming Language :: Python :: 3.5',
    ],
    keywords='project builder cli tool',
    packages=find_packages(exclude=['docs']),
    install_requires=['docopt'],
    entry_points={
        'console_scripts': [
            'otek=otek.cli:main',
        ],
    }
)
