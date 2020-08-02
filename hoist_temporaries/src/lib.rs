use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use std::borrow::BorrowMut;
use std::collections::BTreeSet;
use syn::export::ToTokens;
use syn::visit_mut::VisitMut;
use syn::{Block, Expr, Pat, PatIdent, PatType, Stmt};

mod attr_parser;

#[proc_macro_attribute]
pub fn hoist_temporaries(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        panic!("Invalid attribute - hoist_temporaries takes no arguments.");
    }
    let mut idents = BTreeSet::new();
    let stripped_item = attr_parser::parse(item, &mut idents)
        .map_err(|err| {
            panic!(
                "Failed to parse function marked with #[hoist_temporaries::hoist_temporaries]: {}",
                err
            )
        })
        .unwrap();
    let mut f = syn::parse::<syn::ItemFn>(stripped_item)
        .map_err(|err| panic!("Failed to parse function: {}", err))
        .unwrap();

    for ident in idents {
        let mut rewrite_visitor = RewriteAssignmentVisitor {
            ident: ident.to_string(),
            did_rewrite: false,
        };
        rewrite_visitor.visit_item_fn_mut(&mut f);
        if rewrite_visitor.did_rewrite {
            // Because we don't know what type to ascribe to the backing variable,
            // if the caller doesn't actually write to the variable,
            // they'll get an error about not being able to infer the type of our variable,
            // so don't make a backing variable if it won't be used.
            CreateBackingVisitor {
                ident: ident.to_string(),
            }
            .visit_item_fn_mut(&mut f);
        } else {
            // TODO: Improve this when https://github.com/rust-lang/rust/issues/54725 and/or https://github.com/rust-lang/rust/issues/54140 stabilises.
            eprintln!("warning: hoist_temporaries was enabled for variable `{}` on function `{}` but no temporaries were hoisted.", ident, f.sig.ident);
        }
    }
    f.into_token_stream().into()
}

fn backing_ident(ident: &str) -> proc_macro2::Ident {
    format_ident!("__hoist_temporaries_backing_variable_{}", ident)
}

struct CreateBackingVisitor {
    // Name of the variable whose values should be hoisted.
    ident: String,
}

impl VisitMut for CreateBackingVisitor {
    fn visit_block_mut(&mut self, block: &mut Block) {
        self.create_backing_variable(block);
        syn::visit_mut::visit_block_mut(self, block)
    }
}

impl CreateBackingVisitor {
    fn create_backing_variable(&self, block: &mut Block) {
        // let beep = "boop";
        //
        // becomes
        //
        // let beep = "boop";
        // let __backing_beep;

        // Assume only one let for the variable.
        let pos = block.stmts.iter().position(|stmt| {
            if let Stmt::Local(local) = stmt {
                if let Pat::Ident(PatIdent { ident, .. }) = &local.pat {
                    return *ident == self.ident;
                } else if let Pat::Type(PatType { pat, .. }) = &local.pat {
                    if let Pat::Ident(PatIdent { ident, .. }) = pat.as_ref() {
                        return *ident == self.ident;
                    }
                }
            }
            false
        });
        if let Some(pos) = pos {
            let backing_ident = backing_ident(&self.ident);
            block.stmts.insert(
                pos,
                syn::parse2::<syn::Stmt>(quote! { let mut #backing_ident; })
                    .expect("Could not parse code generated by the procedural macro"),
            );
        }
    }
}

struct RewriteAssignmentVisitor {
    // Name of the variable whose values should be hoisted.
    ident: String,
    // Whether have done any rewrite.
    did_rewrite: bool,
}

impl VisitMut for RewriteAssignmentVisitor {
    fn visit_block_mut(&mut self, block: &mut Block) {
        self.rewrite_assignments(block);
        syn::visit_mut::visit_block_mut(self, block)
    }
}
impl RewriteAssignmentVisitor {
    fn rewrite_assignments(&mut self, block: &mut Block) {
        let stmts = std::mem::replace(&mut block.stmts, vec![]);
        block.stmts = stmts
            .into_iter()
            .flat_map(|mut stmt| {
                if let Stmt::Semi(Expr::Assign(assign), _) = &mut stmt {
                    if let Expr::Path(path) = assign.left.borrow_mut() {
                        if let Some(ident) = path.path.get_ident() {
                            if *ident == self.ident {
                                if let Expr::Reference(rhs) = assign.right.borrow_mut() {
                                    // TODO: If RHS is a reference to a single ident, and that ident was let'd in our ident's scope or above, don't rewrite.
                                    self.did_rewrite = true;

                                    // beep = &format!("boop")
                                    //
                                    // becomes
                                    //
                                    // __backing_beep = format!("boop")
                                    // beep = &__backing_boop

                                    let backing_ident = backing_ident(&self.ident);
                                    assign.left = Box::new(Expr::Path(
                                        syn::parse2::<syn::ExprPath>(quote! { #backing_ident })
                                            .expect("Could not parse code generated by the procedural macro"),
                                    ));
                                    assign.right = rhs.expr.clone();

                                    let ident =
                                        proc_macro2::Ident::new(&self.ident, Span::call_site());
                                    let new_stmt = syn::parse2::<syn::Stmt>(
                                        quote! { #ident = &#backing_ident;},
                                    )
                                    .expect("Could not parse code generated by the procedural macro");
                                    return vec![stmt, new_stmt];
                                }
                            }
                        }
                    }
                }
                vec![stmt]
            })
            .collect();
    }
}
