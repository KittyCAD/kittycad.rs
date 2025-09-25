#![cfg(feature = "requests")]

use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use tokio::time::sleep;

#[derive(Deserialize)]
struct SmokeStatus {
    status: String,
}

#[cfg_attr(not(target_os = "windows"), ignore)]
#[tokio::test(flavor = "current_thread")]
async fn win_ca_smoke() -> Result<()> {
    if !should_run() {
        eprintln!("WIN_CA_SMOKE not set; skipping Windows CA smoke test");
        return Ok(());
    }

    let target =
        std::env::var("SMOKE_URL").unwrap_or_else(|_| "https://localhost:4443/".to_string());
    let attempts = env_u32("SMOKE_ATTEMPTS").unwrap_or(60);
    let delay = Duration::from_millis(env_u64("SMOKE_DELAY_MS").unwrap_or(500));

    let client = reqwest::Client::builder()
        .user_agent(format!(
            "kittycad.rs win-ca smoke/{}",
            env!("CARGO_PKG_VERSION")
        ))
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(5))
        .build()
        .context("building reqwest client for win-ca smoke")?;

    let mut last_error = None;

    for _ in 0..attempts {
        match client.get(&target).send().await {
            Ok(resp) => {
                if !resp.status().is_success() {
                    return Err(anyhow!(
                        "unexpected status {} from {}",
                        resp.status(),
                        target
                    ));
                }

                let body = resp
                    .text()
                    .await
                    .context("reading body from win-ca smoke target")?;

                let trimmed = body.trim();

                if trimmed.eq_ignore_ascii_case("ok") {
                    println!("win-ca smoke OK");
                    return Ok(());
                }

                if let Ok(json) = serde_json::from_str::<SmokeStatus>(trimmed) {
                    if json.status.trim().eq_ignore_ascii_case("ok") {
                        println!("win-ca smoke OK");
                        return Ok(());
                    }

                    return Err(anyhow!(
                        "unexpected JSON status {:?} from {}",
                        json.status,
                        target
                    ));
                }

                return Err(anyhow!(
                    "unexpected response body {:?} from {}",
                    body,
                    target
                ));
            }
            Err(err) => {
                last_error = Some(err.to_string());
            }
        }

        sleep(delay).await;
    }

    Err(anyhow!(
        "failed to reach {} within {attempts} attempts: {}",
        target,
        last_error.unwrap_or_else(|| "no response received".to_string())
    ))
}

fn should_run() -> bool {
    match std::env::var("WIN_CA_SMOKE") {
        Ok(val) => {
            let lower = val.to_ascii_lowercase();
            !(lower.is_empty() || lower == "0" || lower == "false")
        }
        Err(_) => false,
    }
}

fn env_u32(key: &str) -> Option<u32> {
    std::env::var(key).ok()?.parse().ok()
}

fn env_u64(key: &str) -> Option<u64> {
    std::env::var(key).ok()?.parse().ok()
}
