"""Packaging settings."""


from codecs import open
from os.path import abspath, dirname, join
from subprocess import call

from setuptools import find_packages, setup

from otek import __version__


this_dir = abspath(dirname(__file__))
with open(join(this_dir, 'README.rst'), encoding='utf-8') as file:
    long_description = file.read()


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
        'Programming Language :: Python :: 2',
        'Programming Language :: Python :: 2.6',
        'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.2',
        'Programming Language :: Python :: 3.3',
        'Programming Language :: Python :: 3.4',
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
