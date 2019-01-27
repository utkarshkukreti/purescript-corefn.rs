use {Alternative, Bind, Decl, Expression};

pub fn expression(
    expr: &Expression,
    pre: &mut impl FnMut(&Expression),
    post: &mut impl FnMut(&Expression),
) {
    macro_rules! f {
        ($a:expr) => {
            expression($a, pre, post);
        };
    }
    use Expression::*;
    pre(expr);
    match expr {
        Abs { body, .. } => f!(body),
        Accessor { expression, .. } => f!(expression),
        App {
            abstraction,
            argument,
            ..
        } => {
            f!(abstraction);
            f!(argument);
        }
        Case {
            alternatives,
            expressions,
            ..
        } => {
            for alternative in alternatives {
                match alternative {
                    Alternative::Guarded { expressions, .. } => {
                        for expression in expressions {
                            f!(&expression.guard);
                            f!(&expression.expression);
                        }
                    }
                    Alternative::Unguarded { expression, .. } => {
                        f!(expression);
                    }
                }
            }
            for expression in expressions {
                f!(expression);
            }
        }
        Constructor { .. } => {}
        Let {
            expression, binds, ..
        } => {
            f!(expression);
            for decl in binds {
                walk_decl(decl, pre, post);
            }
        }
        Literal { .. } => {}
        Var { .. } => {}
    }
    post(expr);

    fn walk_decl(
        decl: &Decl,
        pre: &mut impl FnMut(&Expression),
        post: &mut impl FnMut(&Expression),
    ) {
        let mut f = |bind: &Bind| expression(&bind.expression, pre, post);
        match decl {
            Decl::NonRec(bind) => f(bind),
            Decl::Rec { binds } => {
                for bind in binds {
                    f(bind)
                }
            }
        }
    }
}

pub fn expression_mut(
    expr: &mut Expression,
    pre: &mut impl FnMut(&mut Expression),
    post: &mut impl FnMut(&mut Expression),
) {
    macro_rules! f {
        ($a:expr) => {
            expression_mut($a, pre, post);
        };
    }
    use Expression::*;
    pre(expr);
    match expr {
        Abs { body, .. } => f!(body),
        Accessor { expression, .. } => f!(expression),
        App {
            abstraction,
            argument,
            ..
        } => {
            f!(abstraction);
            f!(argument);
        }
        Case {
            alternatives,
            expressions,
            ..
        } => {
            for alternative in alternatives {
                match alternative {
                    Alternative::Guarded { expressions, .. } => {
                        for expression in expressions {
                            f!(&mut expression.guard);
                            f!(&mut expression.expression);
                        }
                    }
                    Alternative::Unguarded { expression, .. } => {
                        f!(expression);
                    }
                }
            }
            for expression in expressions {
                f!(expression);
            }
        }
        Constructor { .. } => {}
        Let {
            expression, binds, ..
        } => {
            f!(expression);
            for decl in binds {
                walk_decl(decl, pre, post);
            }
        }
        Literal { .. } => {}
        Var { .. } => {}
    }
    post(expr);

    fn walk_decl(
        decl: &mut Decl,
        pre: &mut impl FnMut(&mut Expression),
        post: &mut impl FnMut(&mut Expression),
    ) {
        let mut f = |bind: &mut Bind| expression_mut(&mut bind.expression, pre, post);
        match decl {
            Decl::NonRec(bind) => f(bind),
            Decl::Rec { binds } => {
                for bind in binds {
                    f(bind)
                }
            }
        }
    }
}
