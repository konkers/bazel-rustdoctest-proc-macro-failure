# Example failure for https://github.com/bazelbuild/rules_rust/pull/1952

## Running

```bash
CARGO_BAZEL_REPIN=true bazel test //lib_crate:lib_crate_doc_test
```

`//lib_crate:lib_crate_doc_test` will succeed with the patch in `WORKSPACE`
applied and fail with the patch commented out.
