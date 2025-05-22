
use std::process::Command;
use std::io::Read;


#[test]
fn test_std_io_pipe() -> Result<(), Box<dyn std::error::Error>> {
    let (mut recv, send) = std::io::pipe()?;

    let mut command = Command::new("path/to/bin")
        .stdout(send.try_clone()?)
        .stderr(send).spawn()?;
    
    let mut output = Vec::new();
    recv.read_to_end(&mut output)?;

    assert!(command.wait()?.success());

    Ok(())
}



