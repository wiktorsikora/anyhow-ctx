use anyhow::bail;

#[test]
fn simple_function() {
    #[anyhow_ctx::with_context(context = "error executing foo")]
    fn foo() -> Result<usize, anyhow::Error> {
        bail!("some function error.")
    }
    let err = foo().unwrap_err();

    assert_eq!("error executing foo: some function error.".to_string(), format!("{err:#}"));
}