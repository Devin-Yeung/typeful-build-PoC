use builder::{Empty, WithA, WithAState, WithB, WithBState, WithBoth, WithBothState};
use typestate::typestate;

use crate::builder::{Builder, EmptyState};

#[typestate]
mod builder {
    #[automaton]
    pub struct Builder {
        pub a: usize,
        pub b: usize,
    }

    pub struct T {
        pub a: usize,
        pub b: usize,
    }

    #[state]
    pub struct Empty;

    #[state]
    pub struct WithA;

    #[state]
    pub struct WithB;

    #[state]
    pub struct WithBoth;

    pub trait Empty {
        fn new() -> Empty;
        fn with_a(self, a: usize) -> WithA;
        fn with_b(self, b: usize) -> WithB;
    }

    pub trait WithA {
        fn with_b(self, b: usize) -> WithBoth;
    }

    pub trait WithB {
        fn with_a(self, a: usize) -> WithBoth;
    }

    pub trait WithBoth {
        fn build(self) -> T;
    }
}

impl EmptyState for Builder<Empty> {
    fn new() -> Builder<Empty> {
        Builder {
            a: 0,
            b: 0,
            state: Empty,
        }
    }

    #[must_use]
    fn with_a(self, a: usize) -> Builder<WithA> {
        Builder {
            a,
            b: self.b,
            state: WithA,
        }
    }

    #[must_use]
    fn with_b(self, b: usize) -> Builder<WithB> {
        Builder {
            a: self.a,
            b,
            state: WithB,
        }
    }
}

impl WithAState for Builder<WithA> {
    #[must_use]
    fn with_b(self, b: usize) -> Builder<WithBoth> {
        Builder {
            a: self.a,
            b,
            state: WithBoth,
        }
    }
}

impl WithBState for Builder<WithB> {
    #[must_use]
    fn with_a(self, a: usize) -> Builder<WithBoth> {
        Builder {
            a,
            b: self.b,
            state: WithBoth,
        }
    }
}

impl WithBothState for Builder<WithBoth> {
    fn build(self) -> builder::T {
        builder::T {
            a: self.a,
            b: self.b,
        }
    }
}

fn main() {
    let t = Builder::new().with_a(42).with_b(1024).build();
    assert_eq!(t.a, 42);
    assert_eq!(t.b, 1024);
    let t = Builder::new().with_b(1024).with_a(42).build();
    assert_eq!(t.a, 42);
    assert_eq!(t.b, 1024);
    // let t = Builder::new().with_a(42).build(); // <- compile-time check
}
