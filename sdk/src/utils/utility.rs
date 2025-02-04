use tonic::{Request, Status};
use {
    argon2::{
        password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash,
        PasswordVerifier, Version,
    },
    serde::{de, Deserialize, Deserializer},
    std::{fmt, str::FromStr, time::Duration},
    tokio::{
        signal::{ctrl_c, unix},
        time,
    },
    uuid::Uuid,
};

pub fn compute_password_hash(password: String) -> Result<String, String> {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let mut us: Vec<u8> = vec![];

    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password_into(password.as_bytes(), &salt.as_str().as_bytes(), &mut us)
    .unwrap();

    let a = String::from_utf8(us);

    if let Err(e) = a {
        return Err(e.to_string());
    }

    Ok(a.unwrap())
}

pub fn compare_password(expected: &str, password: String) -> bool {
    let password_hash = PasswordHash::new(expected);

    if let Err(_e) = password_hash {
        return false;
    }

    let _ = Argon2::default()
        .verify_password(password.as_bytes(), &password_hash.unwrap())
        .map_err(|_| false);

    true
}

pub fn sqlite_test_document(id: Uuid) -> String {
    format!("sqlite://tests/sqlite/tests-{id}.sqlite?mode=rwc")
}

pub async fn graceful_shutdown() {
    let ctr_l = async { ctrl_c().await.expect("FAILED TO HANDLE CONTROL C") };

    #[cfg(unix)]
    let terminate = async {
        unix::signal(unix::SignalKind::terminate())
            .expect("FAILED TO INSTALL SIGNAL HANDLER")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = future::pending::<()>();

    tokio::select! {
        _ = ctr_l => {},
        _ = terminate => {},
        _ = time::sleep(Duration::from_secs(30)) => {
            println!("Timed out waiting for shutdown signal, forcing shutdown...");
        }
    }

    println!("SIGNAL RECEIVED🚨: Handling graceful shutdown🛑 server🦾")
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub fn service_auth<T>(token: &str) -> T
where T: FnMut(Request<()>) -> Result<Request<()>, Status>
{
    |mut req: Request<()>| -> Result<Request<()>, Status> {
        match req.metadata().get("authorization") {
            Some(t) if token == t => Ok(req),
            _ => Err(Status::unauthenticated("No valid auth token")),
        }
    }
}
