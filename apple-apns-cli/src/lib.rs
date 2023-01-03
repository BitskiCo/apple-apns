use std::fs;

use anyhow::Result;
use apple_apns::{
    Alert, Authentication, CertificateAuthority, ClientBuilder, InterruptionLevel, Request, Sound,
};
use clap::Parser;

mod cli;

pub use cli::*;

#[allow(unused_assignments)]
pub async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let mut builder = ClientBuilder::new();

    if let Some(endpoint) = cli.endpoint {
        builder.endpoint = endpoint;
    }

    if let Some(user_agent) = &cli.user_agent {
        builder.user_agent = user_agent;
    }

    let mut ca_pem = None;
    if let Some(ca_pem_file) = &cli.ca_pem_file {
        ca_pem = Some(fs::read(ca_pem_file)?);
        builder.ca = Some(CertificateAuthority::Pem(ca_pem.as_ref().unwrap()))
    }

    let mut client_pem = None;
    let mut key_pem = None;
    if let Some(client_pem_file) = &cli.client_pem_file {
        client_pem = Some(fs::read(client_pem_file)?);
        builder.authentication = Some(Authentication::Certificate {
            client_pem: client_pem.as_ref().unwrap(),
        })
    } else if let (Some(key_id), Some(key_pem_file), Some(team_id)) =
        (&cli.key_id, &cli.key_pem_file, &cli.team_id)
    {
        key_pem = Some(fs::read(key_pem_file)?);
        builder.authentication = Some(Authentication::Token {
            key_id,
            key_pem: key_pem.as_ref().unwrap(),
            team_id,
        });
    }

    let client = builder.build()?;

    let sound = cli.sound.map(|name| {
        let critical = cli.interruption_level == Some(InterruptionLevel::Critical);
        let mut sound = Sound {
            critical,
            name,
            ..Default::default()
        };
        if let Some(volume) = cli.volume {
            sound.volume = volume;
        }
        sound
    });

    let request = Request {
        device_token: cli.device_token,
        push_type: cli.push_type,
        id: cli.id,
        expiration: cli.expiration,
        priority: cli.priority,
        topic: cli.topic,
        collapse_id: cli.collapse_id,
        alert: Some(Alert {
            title: cli.title.map(Into::into),
            subtitle: cli.subtitle.map(Into::into),
            body: cli.body.map(Into::into),
            launch_image: cli.launch_image,
            ..Default::default()
        }),
        badge: cli.badge,
        sound,
        thread_id: cli.thread_id,
        category: cli.category,
        content_available: cli.content_available,
        mutable_content: cli.mutable_content,
        target_content_id: cli.target_content_id,
        interruption_level: cli.interruption_level,
        relevance_score: cli.relevance_score,
        user_info: cli.user_info,
    };

    let apns_id = client.post(request).await?;
    println!("{}", apns_id.as_hyphenated());

    Ok(())
}
