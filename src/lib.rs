use std::rc::Rc;
use no_panic::no_panic;

enum Void {}
struct Not<T>(Rc<dyn Fn(T) -> Void>);

// We want to prove that for all A, B:
// And<Not<A>, Not<B>> <-> Not<Or<A, B>>
struct LHS<A, B> (Not<A>, Not<B>);
struct RHS<A, B> (Not<Result<A, B>>);

impl<A: 'static, B: 'static> Into<RHS<A, B>> for LHS<A, B> {
    #[no_panic]
    fn into(self) -> RHS<A, B> {
        RHS(Not(Rc::new(move |r| match r {
                Ok(a) => (self.0.0)(a),
                Err(b) => (self.1.0)(b),
            }))
        )
    }
}

impl<A: 'static, B: 'static> From<RHS<A, B>> for LHS<A, B> {
    #[no_panic]
    fn from(rhs: RHS<A, B>) -> Self {
        let f = rhs.0.0.clone();
        LHS (
            Not(Rc::new(move |a| f(Ok(a)))),
            Not(Rc::new(move |b| (rhs.0.0)(Err(b)))),
        )
    }
}
