#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
  name: String,
  attributes: Vec<(String, String)>,
  children: Vec<Element>
}


// Fn(&str) -> Result<(&str, Element), &str>

// our first parser
fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
  match input.chars().next() {
    Some('a') => Ok((&input['a'.len_utf8()..], ())),
    _ => Err(input),
  }
}

// a parser builder
fn match_literal(expected: &'static str) 
  -> impl Fn(&str) -> Result<(&str, ()), &str>
  {
    move |input | match input.get(0..expected.len()) {
      Some(next) if next == expected => {
        Ok((&input[expected.len()..], ()))
      }
      _ => Err(input),
    }
  }

#[test]
fn literal_parser() {
  let parse_test = match_literal("test parse text");
  
  assert_eq!(
    Ok(((""), ())),
    parse_test("test parse text")
  );

  assert_eq!(
    Ok(((" remaining content"), ())),
    parse_test("test parse text remaining content")
  );

  assert_eq!(
    Err("error content"),
    parse_test("error content")
  );
}

fn identifier(input: &str) -> Result<(&str, String), &str> {
  let mut matched = String::new();
  let mut chars = input.chars();

  match chars.next() {
    Some(next) if next.is_alphabetic() => matched.push(next),
    _ => return Err(input),
  }

  while let Some(next) = chars.next() {
    if next.is_alphabetic() || next == '-' {
      matched.push(next)
    } else {
      break;
    }
  }

  let next_index = matched.len();
  Ok((&input[next_index..], matched))
}

#[test]
fn identifier_parser() {
  assert_eq!(
    Ok(("", "i-am-an-identifier".to_string())),
    identifier("i-am-an-identifier")
  );

  assert_eq!(
    Ok((" rest", "identifier".to_string())),
    identifier("identifier rest")
  );

  assert_eq!(
    Err("!not"),
    identifier("!not")
  );
}

// a parser that takes two parsers as input and returns
// a new parser which parses both of them in order
// i.e a parser combinator
fn pair_old<P1, P2, R1, R2>(parser1: P1, parser2: P2) ->
  impl Fn(&str) -> Result<(&str, (R1, R2)), &str>
where
  P1: Fn(&str) -> Result<(&str, R1), &str>,
  P2: Fn(&str) -> Result<(&str, R2), &str>
  {
    move |input| 
      match parser1(input) {
        Ok((next_input, result1)) => 
          match parser2(next_input) {
            Ok((final_input, result2)) => 
              Ok((final_input, (result1, result2))),
              Err(err) => Err(err,)
          },
        Err(err) => Err(err),
    }
  }

  #[test]
  fn pair_combinator() {
      let tag_opener = pair_old(match_literal("<"), identifier);
      assert_eq!(
          Ok(("/>", ((), "my-first-element".to_string()))),
          tag_opener("<my-first-element/>")
      );
      assert_eq!(Err("oops"), tag_opener("oops"));
      assert_eq!(Err("!oops"), tag_opener("<!oops"));
  }

// enter the functor
/*
fn map<P, F, A, B>(parser: P, map_fn: F) -> Result<(&str, B), &str>
where
  P: Fn(&str) -> Result<(&str, A), &str>,
  F: Fn(A) -> B,
{
  move |input| 
    parser(input) 
      .map(|(next_input, result)| (next_input, map_fn(result)))
}
*/

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output> {
  fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
  F: Fn(&'a str) -> ParseResult<Output>,
{
  fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
    self(input)
  }
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
  P: Parser<'a, A>,
  F: Fn(A) -> B,
{
  move |input|
    parser.parse(input)
      .map(|(next_input, result)| (next_input, map_fn(result)))
}

fn pair_untidy<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  move |input| match parser1.parse(input) {
    Ok((next_input, result1)) => match parser2.parse(next_input) {
      Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
      Err(err) => Err(err),
    },
    Err(err) => Err(err),
  }
}

fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  move |input| {
    parser1.parse(input).and_then(|(next_input, result1)| {
      parser2.parse(next_input)
        .map(|(last_input, result2)| (last_input, (result1, result2)))
    })
  }
}

fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  map(pair(parser1, parser2), |(left, _)| left)
}

fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
  P1: Parser<'a, R1>,
  P2: Parser<'a, R2>,
{
  map(pair(parser1, parser2),|(_, right)| right)
}
