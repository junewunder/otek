#! /usr/bin/env python3

"""
Otek.

Usage:
    otek list
    otek add <path> <name>
    otek create <name> [-v VARIABLES...]

Options:
    -h --help show this text
    -v --vars key="value" variables to replace in the template project

"""

import os
from docopt import docopt
import glob
import json
import subprocess
import shutil

from .template import Template
from . import __version__ as VERSION


class Otek:
    def __init__(self, args):
        self.home = os.environ['OTEK_HOME']
        self.args = args
        self.settings = {
            'PROJNAME': args['<name>']
        }

        # Check if the .otek directory exists
        if not os.path.exists(self.home):
            self.createDotOtek()

        # Parse cmd arguments
        if args['create']:
            self.copyTemplateFolder(self.home + '/' + args['<name>'] + '/')
            self.settings = {  # using merge dicts operator in 3.5
                **self.settings,
                **self.readHomeFileConfig(),
                **self.readArgsConfig(args['--vars'])
            }
            template = Template(self.settings)
            self.applyTemplate(template)
            if (os.path.exists(self.home + '/' + args['<name>'] + '/create')):
                subprocess.call('/usr/bin/env bash ' + self.home + '/' + args['<name>'] + '/create', shell=True)
                os.remove(os.getcwd() + '/create')

        elif args['add']:
            self.addTemplate(args['<path>'], args['<name>'])

        elif args['list']:
            self.listTemplates()

    def applyTemplate(self, template, origin=os.getcwd()):
        files = glob.glob(origin + '/*')
        fnames = [f.split('/')[-1] for f in files]

        for i, fname in enumerate(files):

            if os.path.isdir(fname):
                self.applyTemplate(template, origin=fname)

            elif os.path.isfile(fname):
                f = open(fname, mode='r+')
                contents = f.read()
                f.close()
                compiled = template.compileContents(contents)
                f = open(fname, mode='w+')
                f.write(compiled)
                f.close()

            else:
                raise Error('Found neither a file nor a folder in template application process')

    def createDotOtek(self):
        dotOtek = __file__.replace('otek/cli.py', 'dotOtek')
        shutil.copytree(dotOtek, self.home)

    def readHomeFileConfig(self):
        configFile = open(self.home + '/otekrc')
        j = json.load(configFile)
        configFile.close()
        return j

    def readArgsConfig(self, argsList):
        argsList = [arg.split('=') for arg in argsList]
        args = {}
        for arg in argsList:
            args[arg[0]] = arg[1]

        return args

    def addTemplate(self, path, name):
        os.mkdir(self.home + '/' + name)
        self.copyTemplateFolder(os.getcwd(), self.home + '/' + name)

    def listTemplates(self):
        arr = glob.glob(self.home + '/*')
        arr = [s.split('/')[-1] for s in arr]
        arr.remove('otekrc')
        print('\n'.join(arr))

    def copyTemplateFolder(self, origin, destination=os.getcwd()):
        files = glob.glob(origin + '/*')
        fnames = [f.split('/')[-1] for f in files]

        for i, fname in enumerate(files):

            if os.path.isdir(fname):
                os.mkdir(destination + '/' + fnames[i])
                self.copyTemplateFolder(fname, destination + '/' + fnames[i])

            elif os.path.isfile(fname):
                original = open(fname, mode='r+')
                copy = open(destination + '/' + fnames[i], mode='w+')
                copy.write(original.read())
                original.close()
                copy.close()

            else:
                raise Error('Found neither a file nor a folder in copy process')


def main():
    if 'OTEK_HOME' not in os.environ:
        os.environ['OTEK_HOME'] = str(os.environ['HOME'] + '/.otek')
    arguments = docopt(__doc__)
    otek = Otek(arguments)
