use std::fmt::Display;
use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T> {
    predicate: Box<dyn Fn(T) -> bool + 'a>,
    substitution: String,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<P, S>(matcher: P, subs: S) -> Matcher<'a, T>
    where
        P: Fn(T) -> bool + 'a,
        S: Display,
    {
        Matcher {
            predicate: Box::new(matcher),
            substitution: subs.to_string(),
        }
    }

    fn substitute(&self, val: T) -> Option<&str> {
        if (self.predicate)(val) {
            Some(&self.substitution)
        } else {
            None
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
#[derive(Default)]
pub struct Fizzy<'a, T> {
    matchers: Vec<Matcher<'a, T>>,
}

impl<'a, T> Fizzy<'a, T>
where
    T: ToString + Clone + 'a,
{
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String> + 'a
    where
        I: IntoIterator<Item = T> + 'a,
    {
        iter.into_iter().map(move |val| self.apply_val(val))
    }

    fn apply_val(&self, val: T) -> String {
        let substitutions: Vec<&str> = self
            .matchers
            .iter()
            .flat_map(|matcher| matcher.substitute(val.clone()))
            .collect();
        if substitutions.is_empty() {
            val.to_string()
        } else {
            substitutions.join("")
        }
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: Clone + PartialEq + Rem<T, Output = T> + Display + From<u8> + 'a,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
