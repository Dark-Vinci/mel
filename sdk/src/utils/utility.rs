use {
    argon2::{
        password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash,
        PasswordVerifier, Version,
    },
    serde::Deserialize,
    serde_json::json,
    std::time::Duration,
    tokio::{
        select,
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

    select! {
        _ = ctr_l => {},
        _ = terminate => {},
        _ = time::sleep(Duration::from_secs(30)) => {
            println!("Timed out waiting for shutdown signal, forcing shutdown...");
        }
    }

    println!("SIGNAL RECEIVED🚨: Handling graceful shutdown🛑 server🦾")
}

pub fn deserialize<T: Deserialize>(msg: Vec<u8>) -> T {
    let str = String::from_utf8(msg).unwrap();

    let j = json!(str);

    let message: T = serde_json::from_value(j).unwrap();

    message
}
