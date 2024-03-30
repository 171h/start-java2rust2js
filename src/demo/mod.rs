use std::convert::TryFrom;

#[allow(unused_imports)]
use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};

pub fn hello() -> Result<(), Box<dyn std::error::Error>> {
    let jvm = JvmBuilder::new().build()?;

    let message = String::from("Hello, I am in Rust!");

    let result_java = jvm.invoke_static(
        "org.astonbitecode.j4rs.demo.Hello",
        "sayHelloStatic",
        &[InvocationArg::try_from(&message)?],
    )?;

    let result_rust: String = jvm.to_rust(result_java)?;

    println!("Rust says: {}", result_rust);

    Ok(())
}
