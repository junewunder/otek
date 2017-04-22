// https://pegjs.org/online

{

  const noUndef = arr =>  arr.filter(elem => elem !== undefined)

  function dotSyntax(string, object) {
    const [ LHS, RHS ] = string.split('.')

    // remove the possibility of the variable being an array
    if (Array.isArray(object[LHS])) return object[LHS]

    // otherwise the left hand side is either an object or a value
    return (
      typeof object[LHS] === 'object'
      ? dotSyntax(RHS, object[LHS])
      : object[LHS]
    )
  }


  function getReplacementText(variable) {
    const replacements = {
      sucks: 'rocks',
      arr:[1,2,3],
      obj: {
        item: 'yayinobject'
      }
    }

    return dotSyntax(variable, replacements)
  }

}
File
  = texts:( __ Text __ Replacement __ )* {
    return texts
      .reduce((acc, elem) => acc.concat(elem), [])
      .reduce((acc, elem) => acc.concat(elem), [])
      .reduce((acc, elem) => acc + elem, '')
  }

OpenSeparator = "("
CloseSeparator = ")"

Text = [^( \t\n\r] *

Replacement
  = OpenSeparator _ replacement:Template _ CloseSeparator {
  	return replacement
  }

Template
  = VariableOrElse
  / Variable

Variable
  = variable:[^) \t\n\r|]* {
    console.log(variable)
    return getReplacementText(variable.join(''))
  }

VariableOrElse
  = variable:[^) \t\n\r|]* _ '|' _ other:QuotedText {
  	console.log(variable)
    return getReplacementText(variable.join('')) || other
  }

QuotedText
  = text:(("'" [^'\n\r]* "'") / ('"' [^"\n\r]* '"')) {
    return text[1].join('')
  }

__ "whitespace"
  = space:[ \t\n\r]* { return space }

_ "whitespaceIgnore"
  = [ \t\n\r]* { return }
