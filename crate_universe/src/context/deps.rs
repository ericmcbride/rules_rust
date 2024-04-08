use crate::config::CrateId;
use crate::context::CrateContext;
use crate::select::Select;

/// Allows Dependencies to be resolved during conditional compilations.  An example
/// of a conditonal compilation would be `tokio` with the unstable flag:
/// `RUST_FLAGS='--cfg tokio_unstable' cargo build foo`
///
/// Or in bazel terms `bazel build -s --config local @crates//:tokio`
///
/// With the crate annotation looking like the following in WORKSPACE or MODULE.bazel
///
/// ## WORKSPACE
/// ```no_run
/// annotations = {
///     "tokio": [crate.annotation(
///         rustc_flags = ["--cfg", "tokio_unstable"],
///     )],
/// },
/// ```
/// ## MODULE.bazel
/// ```no_run
/// crate.annotation(
///    crate = "tokio",
///    repositories = ["crates"],
///    version = "1.28.0",
///    rustc_flags = ["--cfg", "tokio_unstable"],
/// )
/// ```
pub(super) fn resolve_cfg_deps(crate_context: &mut CrateContext) {
    let rustc_flags = get_cfg_flag_values(crate_context.common_attrs.rustc_flags.values());
    let crate_id = CrateId::new(crate_context.name.clone(), crate_context.version.clone());
    let mut new_crate_dependencies = Select::new();

    for (key, crate_feature_dep) in crate_context.common_attrs.deps.items() {
        if let Some(key) = key {
            let expr = cfg_expr::Expression::parse(&key);
            // if the key is not all, any, or not, we ignore it
            // (this causes an error in cfg-expr)
            if let Ok(expr) = expr {
                expr.eval(|pred| match pred {
                    cfg_expr::Predicate::Flag(f) => {
                        let flag_string = f.to_string();
                        if rustc_flags.contains(&flag_string) {
                            // dont want to accidently re insert the same dep
                            // as a dep dependency.
                            if crate_id != crate_feature_dep.id {
                                new_crate_dependencies.insert(crate_feature_dep.clone(), None);
                            }
                        }
                        true
                    }
                    ref _other_type => {
                        println!("Not a flag {:?}", pred);
                        false
                    }
                });
            }
            new_crate_dependencies.insert(crate_feature_dep.clone(), Some(key.clone()));
        } else {
            new_crate_dependencies.insert(crate_feature_dep.clone(), None);
        }
    }

    crate_context.common_attrs.deps = new_crate_dependencies;
}

/// We only care about the `--cfg` conditional compilation flags.  
/// An example of a conditional compilation flag is `--cfg tokio_unstable`
///
/// There could be other rustc flags, that are not lead with `--cfg`.
/// We only want to gather the conditional compilation cfg flags, and return
/// the values of those flags, so we can do a comparison to see if they exist
/// in the select dependencies, as a key value of `cfg(tokio_unstable)
fn get_cfg_flag_values(rustc_flags: Vec<String>) -> Vec<String> {
    let mut cfg_rustc_flags = vec![];
    let mut rustc_flags_peekable = rustc_flags.iter().peekable();
    while let Some(flag) = rustc_flags_peekable.next() {
        if flag == "--cfg" {
            match rustc_flags_peekable.peek() {
                Some(next_flag) => {
                    cfg_rustc_flags.push(next_flag.to_string());
                }
                None => {}
            }
        }
    }
    cfg_rustc_flags
}

/*
pub fn resolve_cfg_deps(crates: &mut BTreeMap<CrateId, CrateContext>) {
    // first we need to iterate and get the rustc flags for each dependency.  Once we get
    // the rustc flags, we need to look at the select optional depedencies.  If they match
    // the rustc_flags, we gather them and need to flip the feature on. What we do after
    // that we gotta figure out how to do feature stuff, or how to stuff the dependency
    // in.
    // if u flip this to BSet::new()no compiler error
    let mut new_crates: BTreeMap<CrateId, Select<BTreeSet<CrateDependency>>> = BTreeMap::new();

    for (crate_id_key, ctx) in crates.iter() {
        let mut new_crate_dependencies = Select::new();

        let crate_id = CrateId::new(ctx.name.clone(), ctx.version.clone());
        let rustc_flags = ctx.common_attrs.rustc_flags.values();
        // We have to filter for cfg flags here.  We dont care about any
        // other rustc flags but the --cfg flags.

        // TODO: make function
        let mut cfg_rustc_flags = vec![];
        let mut rustc_flags_peekable = rustc_flags.iter().peekable();
        while let Some(flag) = rustc_flags_peekable.next() {
            if flag == "--cfg" {
                match rustc_flags_peekable.peek() {
                    Some(next_flag) => {
                        cfg_rustc_flags.push(next_flag.to_string());
                    }
                    None => {}
                }
            }
        }

        // see if we need to lift any of the select deps to common deps
        for (key, crate_feature_dep) in ctx.common_attrs.deps.items() {
            println!("key is {:?}", key);
            println!("crate feature dep is {:?}", crate_feature_dep);

            if let Some(key) = key {
                // use ctx.name for key of map, and a btree set to gather all
                // the new features!
                let expr = cfg_expr::Expression::parse(&key);
                // TODO: CHECK THIS, checks to make sure no parse error happened.
                // For example aarch64-pc-windows-gnullvm is causing
                // `Result::unwrap()` on an `Err` value: ParseError
                // { original: "aarch64-pc-windows-gnullvm", span: 7..8,
                // reason: Unexpected(["<key>", "all", "any", "not"]) }
                if let Ok(expr) = expr {
                    println!("EXPR IS {:?}", expr);
                    expr.eval(|pred| match pred {
                        cfg_expr::Predicate::Flag(f) => {
                            //println!("Flag is {:?}", f);
                            // since its a flag type, we just do a 1:1 string
                            // match and not worry about "nots" and such.
                            let flag_string = f.to_string();
                            //println!("FLAG_STRING is {:?}", flag_string);
                            //println!("Cfg rustc_flags are {:?}", cfg_rustc_flags);
                            if cfg_rustc_flags.contains(&flag_string) {
                                println!("Features being inserted");
                                if crate_id != crate_feature_dep.id {
                                    new_crate_dependencies.insert(crate_feature_dep.clone(), None);
                                }
                            }
                            true
                        }
                        ref other_type => {
                            println!("Not a flag {:?}", pred);
                            false
                        }
                    });
                }
                new_crate_dependencies.insert(crate_feature_dep.clone(), Some(key.clone()));
            } else {
                new_crate_dependencies.insert(crate_feature_dep.clone(), None);
            }
        }
        new_crates.insert(crate_id, new_crate_dependencies);
    }
    println!("New crates are {:#?}", new_crates);

    // update deps
    for (key, new_crate) in new_crates {
        crates.get_mut(&key).unwrap().common_attrs.deps = new_crate;
    }
}*/
