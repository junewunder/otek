const { stream, N, C } = require('parser-combinator')

const document = '|4.6l';

const floorCombinator = C
  .char('|')
  .thenRight( N.numberLiteral )    // we had [ '|' , 4.6], we keep 4.6
  .thenLeft( C.char('l') )   // we had [ 4.6 , '|' ], we keep 4.6
  .map(x => {console.log(x); return Math.floor(x)}); // we transform selected value in meaningful value

// Parsec needs a stream of characters
const parsing = floorCombinator.parse(stream.ofString(document));

console.log(parsing);

console.log( parsing.value === 4 );


function template() {}

function text() {}

function replacement() {

  return

}
