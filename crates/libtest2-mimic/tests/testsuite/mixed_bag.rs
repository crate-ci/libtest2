use snapbox::prelude::*;
use snapbox::str;

fn test_cmd() -> snapbox::cmd::Command {
    static BIN: once_cell_polyfill::sync::OnceLock<(std::path::PathBuf, std::path::PathBuf)> =
        once_cell_polyfill::sync::OnceLock::new();
    let (bin, current_dir) = BIN.get_or_init(|| {
        let package_root = crate::util::new_test(
            r#"
fn main() {
    use libtest2_mimic::Trial;
    use libtest2_mimic::RunError;
    libtest2_mimic::Harness::with_env()
        .cases(vec![
            Trial::test("cat", |_| Ok(())),
            Trial::test("dog", |_| Err(RunError::fail("was not a good boy"))),
            Trial::test("fox", |_| Ok(())),
            Trial::test("bunny", |state| {
                state.ignore_for("fails")?;
                Err(RunError::fail("jumped too high"))
            }),
            Trial::test("frog", |state| {
                state.ignore_for("slow")?;
                Ok(())
            }),
            Trial::test("owl", |state| {
                state.ignore_for("fails")?;
                Err(RunError::fail("broke neck"))
            }),
            Trial::test("fly", |state| {
                state.ignore_for("fails")?;
                Ok(())
            }),
            Trial::test("bear", |state| {
                state.ignore_for("fails")?;
                Err(RunError::fail("no honey"))
            }),
        ])
        .main();
}
"#,
            false,
        );
        let bin = crate::util::compile_test(&package_root);
        (bin, package_root)
    });
    snapbox::cmd::Command::new(bin).current_dir(current_dir)
}

fn check(args: &[&str], code: i32, single: impl IntoData, parallel: impl IntoData) {
    test_cmd()
        .args(args)
        .args(["--test-threads", "1"])
        .assert()
        .code(code)
        .stdout_eq(single);
    test_cmd()
        .args(args)
        .assert()
        .code(code)
        .stdout_eq(parallel);
}

#[test]
fn normal() {
    check(
        &[],
        101,
        str![[r#"

running 8 tests
test bear  ... ignored
test bunny ... ignored
test cat   ... ok
test dog   ... FAILED
test fly   ... ignored
test fox   ... ok
test frog  ... ignored
test owl   ... ignored

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn test_mode() {
    check(
        &["--test"],
        101,
        str![[r#"

running 8 tests
test bear  ... ignored
test bunny ... ignored
test cat   ... ok
test dog   ... FAILED
test fly   ... ignored
test fox   ... ok
test frog  ... ignored
test owl   ... ignored

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn bench_mode() {
    check(
        &["--bench"],
        101,
        str![[r#"

running 8 tests
test bear  ... ignored
test bunny ... ignored
test cat   ... ok
test dog   ... FAILED
test fly   ... ignored
test fox   ... ok
test frog  ... ignored
test owl   ... ignored

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn list() {
    check(
        &["--list"],
        0,
        str![[r#"
bear: test
bunny: test
cat: test
dog: test
fly: test
fox: test
frog: test
owl: test

8 tests


"#]],
        str![[r#"
bear: test
bunny: test
cat: test
dog: test
fly: test
fox: test
frog: test
owl: test

8 tests


"#]],
    );
}

#[test]
fn list_ignored() {
    check(
        &["--list", "--ignored"],
        0,
        str![[r#"
bear: test
bunny: test
cat: test
dog: test
fly: test
fox: test
frog: test
owl: test

8 tests


"#]],
        str![[r#"
bear: test
bunny: test
cat: test
dog: test
fly: test
fox: test
frog: test
owl: test

8 tests


"#]],
    );
}

#[test]
fn list_with_filter() {
    check(
        &["--list", "a"],
        0,
        str![[r#"
bear: test
cat: test

2 tests


"#]],
        str![[r#"
bear: test
cat: test

2 tests


"#]],
    );
}

#[test]
fn list_with_specified_order() {
    check(
        &["--list", "--exact", "owl", "fox", "bunny", "frog"],
        0,
        str![[r#"
owl: test
fox: test
bunny: test
frog: test

4 tests


"#]],
        str![[r#"
owl: test
fox: test
bunny: test
frog: test

4 tests


"#]],
    );
}

#[test]
fn filter_c() {
    check(
        &["a"],
        0,
        str![[r#"

running 2 tests
test bear ... ignored
test cat  ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 6 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

test result: ok. 1 passed; 0 failed; 1 ignored; 6 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_o_test() {
    check(
        &["--test", "a"],
        0,
        str![[r#"

running 2 tests
test bear ... ignored
test cat  ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 6 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

test result: ok. 1 passed; 0 failed; 1 ignored; 6 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_o_test_include_ignored() {
    check(
        &["--test", "--include-ignored", "o"],
        101,
        str![[r#"

running 4 tests
test dog  ... FAILED
test fox  ... ok
test frog ... ok
test owl  ... FAILED

failures:

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    dog
    owl

test result: FAILED. 2 passed; 2 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
        str![[r#"

running 4 tests
...

failures:

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    dog
    owl

test result: FAILED. 2 passed; 2 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_o_test_ignored() {
    check(
        &["--test", "--ignored", "o"],
        101,
        str![[r#"

running 4 tests
test dog  ... FAILED
test fox  ... ok
test frog ... ok
test owl  ... FAILED

failures:

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    dog
    owl

test result: FAILED. 2 passed; 2 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
        str![[r#"

running 4 tests
...

failures:

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    dog
    owl

test result: FAILED. 2 passed; 2 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn normal_include_ignored() {
    check(
        &["--include-ignored"],
        101,
        str![[r#"

running 8 tests
test bear  ... FAILED
test bunny ... FAILED
test cat   ... ok
test dog   ... FAILED
test fly   ... ok
test fox   ... ok
test frog  ... ok
test owl   ... FAILED

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    bear
    bunny
    dog
    owl

test result: FAILED. 4 passed; 4 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    bear
    bunny
    dog
    owl

test result: FAILED. 4 passed; 4 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn normal_ignored() {
    check(
        &["--ignored"],
        101,
        str![[r#"

running 8 tests
test bear  ... FAILED
test bunny ... FAILED
test cat   ... ok
test dog   ... FAILED
test fly   ... ok
test fox   ... ok
test frog  ... ok
test owl   ... FAILED

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    bear
    bunny
    dog
    owl

test result: FAILED. 4 passed; 4 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    bear
    bunny
    dog
    owl

test result: FAILED. 4 passed; 4 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn lots_of_flags() {
    check(
        &["--ignored", "--skip", "g", "--test", "o"],
        101,
        str![[r#"

running 2 tests
test fox ... ok
test owl ... FAILED

failures:

---- owl ----
broke neck


failures:
    owl

test result: FAILED. 1 passed; 1 failed; 0 ignored; 6 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

failures:

---- owl ----
broke neck


failures:
    owl

test result: FAILED. 1 passed; 1 failed; 0 ignored; 6 filtered out; finished in [..]s


"#]],
    );
}

#[test]
#[cfg(feature = "json")]
fn list_json() {
    check(
        &["-Zunstable-options", "--format=json", "--list", "a"],
        0,
        str![[r#"
[
  {
    "event": "discover_start"
  },
  {
    "event": "discover_case",
    "name": "bunny",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "dog",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "fly",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "fox",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "frog",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "owl",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "bear"
  },
  {
    "event": "discover_case",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_complete"
  }
]
"#]]
        .is_json()
        .against_jsonlines(),
        str![[r#"
[
  {
    "event": "discover_start"
  },
  {
    "event": "discover_case",
    "name": "bunny",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "dog",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "fly",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "fox",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "frog",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "owl",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "bear"
  },
  {
    "event": "discover_case",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_complete"
  }
]
"#]]
        .is_json()
        .against_jsonlines(),
    );
}

#[test]
#[cfg(feature = "json")]
fn test_json() {
    check(
        &["-Zunstable-options", "--format=json", "a"],
        0,
        str![[r#"
[
  {
    "event": "discover_start"
  },
  {
    "event": "discover_case",
    "name": "bunny",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "dog",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "fly",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "fox",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "frog",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "owl",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "bear"
  },
  {
    "event": "discover_case",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_complete"
  },
  {
    "event": "suite_start"
  },
  {
    "event": "case_start",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "message": "fails",
    "name": "bear",
    "status": "ignored"
  },
  {
    "event": "case_start",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "suite_complete"
  }
]
"#]]
        .is_json()
        .against_jsonlines(),
        str![[r#"
[
  {
    "event": "discover_start"
  },
  {
    "event": "discover_case",
    "name": "bear"
  },
  {
    "event": "discover_case",
    "name": "bunny",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "cat"
  },
  {
    "event": "discover_case",
    "name": "dog",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "fly",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "fox",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "frog",
    "run": false
  },
  {
    "event": "discover_case",
    "name": "owl",
    "run": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_complete"
  },
  {
    "event": "suite_start"
  },
  {
    "event": "case_start",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "message": "fails",
    "name": "bear",
    "status": "ignored"
  },
  {
    "event": "case_start",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "suite_complete"
  }
]
"#]]
        .unordered()
        .is_json()
        .against_jsonlines(),
    );
}

#[test]
fn terse_output() {
    check(
        &["--quiet"],
        101,
        str![[r#"

running 8 tests
ii.Fi.ii
failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...
failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}
