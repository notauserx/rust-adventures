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

fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
  move |input: &'a str | match input.get(0..expected.len()) {
    Some(next) if next == expected => Ok((&input[expected.len()..], ())),
    _ => Err(input)
  }
}

fn identifier(input: &str) -> ParseResult<String> {
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

#[test]
fn right_combinator() {
  let tag_opener = right(match_literal("<"), identifier);
  assert_eq!(
    Ok(("/>", "element".to_string())),
    tag_opener.parse("<element/>")
  );

  assert_eq!(Err("element"), tag_opener.parse("element"));
  assert_eq!(Err("!element"), tag_opener.parse("<!element"));
}

fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
  P: Parser<'a, A>,
{
  move |mut input| {
    let mut result = Vec::new();

    if let Ok((next_input, first_item)) = parser.parse(input) {
      input = next_input;
      result.push(first_item);
    } else {
      return Err(input);
    }

    while let Ok((next_input, next_item)) = parser.parse(input) {
      input = next_input;
      result.push(next_item);
    }

    Ok((input, result))
  }
}

#[test]
fn one_or_more_combinator() {
  let parser = one_or_more(match_literal("ha"));

  assert_eq!(Ok(("", vec![(),(),()])), parser.parse("hahaha"));
  assert_eq!(Err("ahah"), parser.parse("ahah"));
  assert_eq!(Err(""), parser.parse(""));
}

fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
  P: Parser<'a, A>,
{
  move |mut input| {
    let mut result = Vec::new();

    while let Ok((next_input, next_item)) = parser.parse(input) {
      input = next_input;
      result.push(next_item);
    }

    Ok((input, result))
  }
}

#[test]
fn zero_or_more_combinator() {
    let parser = zero_or_more(match_literal("ha"));
    assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
    assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
    assert_eq!(Ok(("", vec![])), parser.parse(""));
}

fn any_char(input: &str) -> ParseResult<char> {
  match input.chars().next() {
    Some(next) => Ok((&input[next.len_utf8()..],next)),
    _ => Err(input),
  }
}

fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
  P: Parser<'a, A>,
  F: Fn(&A) -> bool,
{
  move |input| {
    if let Ok((next_input, value)) = parser.parse(input) {
      if predicate(&value) {
        return Ok((next_input, value))
      }
    }
    Err(input)
  }
}

#[test]
fn predicate_combinator() {
  let parser = pred(any_char, |c| *c == 'o');

  assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
  assert_eq!(Err("lol"), parser.parse("lol"));
}

fn whitespace_char<'a>() -> impl Parser<'a, char> {
  pred(any_char, |c| c.is_whitespace())
}

fn one_or_more_whitespace<'a>() -> impl Parser<'a, Vec<char>> {
  one_or_more(whitespace_char())
}

fn zero_or_more_whitespace<'a>() -> impl Parser<'a, Vec<char>> {
  zero_or_more(whitespace_char())
}