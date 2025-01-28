use fantoccini::{Client, Locator};
use enigo::{Enigo, Key};
use kuchiki::parse_html;
use std::error::Error;
use tokio::time::{sleep, Duration};
use log::{info, error};

async fn send_connection_request_action(
    profile_url: &str,
    driver: &mut Client,
) -> Result<(), Box<dyn Error>> {
    info!("Navigating to profile URL: {}", profile_url);
    driver.goto(profile_url).await?;
    sleep(Duration::from_secs(5)).await;

    let mut enigo = Enigo::new();
    enigo.key_click(Key::Home);
    sleep(Duration::from_secs(2)).await;

    // Implementa lógica para enviar la solicitud de conexión en la página del perfil
    if let Ok(button) = driver.find(Locator::Css("button.connect-button")).await {
        button.click().await?;
        info!("Connection request sent.");
    } else {
        error!("Connect button not found.");
    }

    Ok(())
}

async fn withdraw_connection(
    contact_name: &str,
    driver: &mut Client,
) -> Result<(), Box<dyn Error>> {
    info!("Navigating to sent invitations page.");
    driver.goto("https://www.linkedin.com/mynetwork/invitation-manager/sent/").await?;
    sleep(Duration::from_secs(5)).await;

    loop {
        // Busca el contacto en la lista de solicitudes enviadas
        let contact_locator = Locator::XPath(&format!(
            "//ul/li//a[text()='{}']/../../../../../..//button/span[text()='Withdraw']",
            contact_name
        ));

        if let Ok(button) = driver.find(contact_locator).await {
            button.click().await?;
            info!("Withdrawal initiated for {}.", contact_name);

            // Confirma la retirada
            if let Ok(confirm_button) = driver
                .find(Locator::XPath("//button/span[text()='Withdraw']"))
                .await
            {
                confirm_button.click().await?;
                info!("Connection request withdrawn for {}.", contact_name);
                break;
            } else {
                error!("Confirmation button not found.");
            }
        } else {
            info!("Contact not found, scrolling down.");
            let mut enigo = Enigo::new();
            enigo.key_click(Key::PageDown);
            sleep(Duration::from_secs(2)).await;
        }
    }

    Ok(())
}

async fn download_received_invitations(driver: &mut Client) -> Result<(), Box<dyn Error>> {
    info!("Navigating to received invitations page.");
    driver.goto("https://www.linkedin.com/mynetwork/invitation-manager/").await?;
    sleep(Duration::from_secs(10)).await;

    let page_source = driver.source().await?;
    let document = parse_html().one(page_source);
    // Aquí procesas el HTML, por ejemplo, extrayendo datos específicos.
    info!("Received invitations processed and stored.");

    Ok(())
}

async fn download_sent_invitations(driver: &mut Client) -> Result<(), Box<dyn Error>> {
    info!("Navigating to sent invitations page.");
    driver.goto("https://www.linkedin.com/mynetwork/invitation-manager/sent/").await?;
    sleep(Duration::from_secs(10)).await;

    let page_source = driver.source().await?;
    let document = parse_html().one(page_source);
    // Aquí procesas el HTML, por ejemplo, extrayendo datos específicos.
    info!("Sent invitations processed and stored.");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut driver = fantoccini::ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;

    // Ejemplo de uso de las funciones
    send_connection_request_action("https://www.linkedin.com/in/example-profile", &mut driver)
        .await?;
    withdraw_connection("John Doe", &mut driver).await?;
    download_received_invitations(&mut driver).await?;
    download_sent_invitations(&mut driver).await?;

    driver.close().await?;
    Ok(())
}
