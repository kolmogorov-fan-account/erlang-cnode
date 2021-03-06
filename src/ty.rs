use std::{collections, str};

#[derive(Copy, Clone, Debug)]
pub enum NameKind {
  Node,
  Alive,
  Host,
}

/// An atom is a constant term with a name made of up to 255 unicode code points.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Atom(pub(crate) Box<str>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
  pub name: Atom,
  pub(crate) serial_number: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Reference {
  pub node: Node,
  pub id: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pid {
  pub node: Node,
  pub(crate) id: u16,
  pub(crate) serial: u16,
}

#[derive(Debug)]
pub struct Tuple(pub Box<[Term]>);

#[derive(Debug)]
pub struct List(pub Box<[Term]>);

#[derive(Debug)]
pub struct Binary(pub Box<[u8]>);

#[derive(Debug)]
pub enum Term {
  Nil,
  Integer(i32),
  Float(f64),
  Atom(Atom),
  Pid(Pid),
  Reference(Reference),
  Tuple(Tuple),
  List(List),
  Binary(Binary),
}

#[derive(Debug, Eq, PartialEq)]
pub enum TermKind {
  Nil,
  Integer,
  Float,
  Atom,
  Pid,
  Reference,
  Tuple,
  List,
  Binary,
}

#[derive(Debug)]
pub struct TraceToken {
  serial: i64,
  previous: i64,
  from: Pid,
  label: i64,
  flags: i64,
}

#[derive(Debug)]
pub enum ControlMessage {
  Send {
    from: Pid,
    to: Pid,
    trace_token: Option<TraceToken>,
  },
  RegisteredSend {
    from: Pid,
    to: Atom,
    trace_token: Option<TraceToken>,
  },
  Link {
    from: Pid,
    to: Pid,
  },
  Unlink {
    from: Pid,
    to: Pid,
  },
  Exit {
    from: Pid,
    to: Pid,
  },
}

#[derive(Debug)]
pub enum Message {
  Send {
    from: Pid,
    to: Pid,
    trace_token: Option<TraceToken>,
    term: Term,
  },
  RegisteredSend {
    from: Pid,
    to: Atom,
    trace_token: Option<TraceToken>,
    term: Term,
  },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AtomCacheSegment {
  S0,
  S1,
  S2,
  S3,
  S4,
  S5,
  S6,
  S7,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AtomCacheKey {
  pub segment_index: AtomCacheSegment,
  pub internal_index: u8,
}

pub struct AtomCache {
  pub entries: collections::HashMap<AtomCacheKey, Atom>,
}

pub struct TermViewBuffer<'term> {
  pub atoms: cursed_collections::AppendOnlyVec<&'term str>,
}

#[derive(Debug)]
pub enum TermView<'term> {
  Nil,
  Atom(&'term str),
  Integer(i32),
  Float(f64),
}
