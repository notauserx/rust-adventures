# rparsec

Writing a parser for a simplified version of XML from the excellent article from https://bodil.lol/parser-combinators/

- XML elements open with the symbol < and an identifier
- followed by some whitespace
- followed by optional list of attribute pairs
- Finally either a closing /> to signify a single element with no children
- or a > to signify there is a sequence of child elements and finally a closing tab </ followed by the identifier of the opening tag and then >


