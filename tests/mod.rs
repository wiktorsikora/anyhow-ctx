use anyhow::bail;

#[test]
fn simple_function() {
    #[anyhow_ctx::with_context(context = "error executing foo")]
    fn foo() -> Result<(), anyhow::Error> {
        bail!("some function error.")
    }
    let err = foo().unwrap_err();

    assert_eq!(
        "error executing foo: some function error.".to_string(),
        format!("{err:#}")
    );
}

#[test]
fn generic_function() {
    #[anyhow_ctx::with_context(context = "error executing foo")]
    fn foo<T>(arg: T) -> Result<T, anyhow::Error>
    where
        T: ToString,
    {
        let _ = arg.to_string();
        bail!("some function error.")
    }
    let err = foo("unused arg").unwrap_err();

    assert_eq!(
        "error executing foo: some function error.".to_string(),
        format!("{err:#}")
    );
}

#[tokio::test]
async fn simple_async_function() {
    #[anyhow_ctx::with_context(context = "error executing async foo")]
    async fn foo() -> Result<(), anyhow::Error> {
        bail!("some function error.")
    }
    let err = foo().await.unwrap_err();

    assert_eq!(
        "error executing async foo: some function error.".to_string(),
        format!("{err:#}")
    );
}

#[test]
fn fmt_without_arg() {
    #[anyhow_ctx::with_context(fmt = "error executing foo")]
    fn foo() -> Result<(), anyhow::Error> {
        bail!("some function error.")
    }
    let err = foo().unwrap_err();

    assert_eq!(
        "error executing foo: some function error.".to_string(),
        format!("{err:#}")
    );
}

#[test]
fn fmt_with_borrowed_arg() {
    #[anyhow_ctx::with_context(fmt = "error executing foo with arg '{some_arg}'")]
    fn foo(some_arg: &str) -> Result<(), anyhow::Error> {
        let _ = some_arg;
        bail!("some function error.")
    }
    let err = foo("arg value").unwrap_err();

    assert_eq!(
        "error executing foo with arg 'arg value': some function error.".to_string(),
        format!("{err:#}")
    );
}
