#![cfg(unix)]
use std::sync::Once;
use tiberius::{AuthMethod, Client, Config, EncryptionLevel, Result};
use tokio::{net::TcpStream, runtime::Runtime};
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[allow(dead_code)]
static LOGGER_SETUP: Once = Once::new();

// Generate a rsa private key (this is your CA private key)
// Create a self signed certificate using the CA for the purposes of 
// Generate another rsa private key (this is your server private key)
// Create a CSR based on the "server private key"
// Complete the CSR using the "CA private key"
// https://docs.openssl.org/master/man1/openssl-ca/#synopsis
// openssl-ca - OpenSSL Documentation
 

#[test]
#[cfg(any(
    feature = "rustls",
    feature = "native-tls",
    feature = "vendored-openssl"
))]
fn connect_to_custom_cert_instance_ado() -> Result<()> {
    LOGGER_SETUP.call_once(|| {
        env_logger::init();
    });

    let rt = Runtime::new()?;

    rt.block_on(async {
        let mut config = Config::from_ado_string(
            "server=tcp:localhost,1433;IntegratedSecurity=true;TrustServerCertificateCA=mssql.crt",
        )?;
        config.authentication(AuthMethod::sql_server("sa", "<YourStrong@Passw0rd>"));

        let tcp = TcpStream::connect(config.get_addr()).await?;

        let client = Client::connect(config, tcp.compat_write()).await;

        assert!(client.is_err());

        Ok(())
    })
}

#[test]
#[cfg(any(
    feature = "rustls",
    feature = "native-tls",
    feature = "vendored-openssl"
))]
fn connect_to_custom_cert_instance_jdbc() -> Result<()> {
    LOGGER_SETUP.call_once(|| {
        env_logger::init();
    });

    let rt = Runtime::new()?;

    rt.block_on(async {
        // Careful: the / in the TrustServerCertificateCA needs to be escaped
        let mut config = Config::from_jdbc_string(
            "jdbc:sqlserver://localhost:1433;TrustServerCertificateCA=mssql.crt",
        )?;
        config.authentication(AuthMethod::sql_server("sa", "<YourStrong@Passw0rd>"));
        // config.trust_cert_ca("mssql.crt");

        let tcp = TcpStream::connect(config.get_addr()).await?;

        let client = Client::connect(config, tcp.compat_write()).await;

        assert!(client.is_err());

        // let row = client
        //     .query("SELECT @P1", &[&-4i32])
        //     .await?
        //     .into_row()
        //     .await?
        //     .unwrap();

        // assert_eq!(Some(-4i32), row.get(0));

        Ok(())
    })
}

#[test]
fn connect_to_custom_cert_instance_without_ca() -> Result<()> {
    LOGGER_SETUP.call_once(|| {
        env_logger::init();
    });

    let rt = Runtime::new()?;

    rt.block_on(async {
        let mut config = Config::new();
        config.authentication(AuthMethod::sql_server("sa", "<YourStrong@Passw0rd>"));
        config.encryption(EncryptionLevel::Required);
        config.host("localhost");
        config.port(1433);
        // config.trust_cert_ca("mssql.crt");

        let tcp = TcpStream::connect(config.get_addr()).await?;

        let client = Client::connect(config, tcp.compat_write()).await;

        assert!(client.is_err());
        Ok(())
    })
}
