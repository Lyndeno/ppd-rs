use zbus::{Connection, Result, proxy};
fn main() {
    let result = futures::executor::block_on(get_profile()).unwrap();
    println!("Current profile is {result}");
    futures::executor::block_on(set_profile("performance".to_string())).unwrap();
    let result = futures::executor::block_on(get_profile()).unwrap();
    println!("Current profile is {result}");
}

#[proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_service = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles"
)]
trait Ppd {
    #[zbus(property)]
    fn active_profile(&self) -> Result<String>;

    #[zbus(property)]
    fn set_active_profile(&self, string: String) -> Result<()>;
}

async fn get_profile() -> Result<String> {
    let connection = Connection::system().await?;

    let proxy = PpdProxy::new(&connection).await?;
    let reply = proxy.active_profile().await?;
    Ok(reply)
}

async fn set_profile(string: String) -> Result<()> {
    let connection = Connection::system().await?;

    let proxy = PpdProxy::new(&connection).await?;
    proxy.set_active_profile(string).await?;
    Ok(())
}
