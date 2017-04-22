import os
import glob
import re


class Template(object):
    """docstring for Template"""
    def __init__(self, options):
        self.options = options

        self.pattern = re.compile(r'<%\s*([^\s%>]*)\s*%>')

    def getOption(self, match):
        option = match.group(1)
        options = self.options
        optionList = option.split('.')
        for option in optionList:
            options = options[option]

        return options

    def compileContents(self, contents):
        contents = re.sub(self.pattern, self.getOption, contents)

        return contents
