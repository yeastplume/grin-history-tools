use diesel::prelude::*;
use std::sync::Arc;

use grin_core::core::hash::Hashed;
use grin_core::pow::Difficulty;
use grin_core::{genesis, global};
use grin_p2p as p2p;
use grin_servers::{resolve_dns_to_addrs, MAINNET_DNS_SEEDS, TESTNET_DNS_SEEDS};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use crate::service::SeedCheckError;
use crate::DbError;

use crate::database::PooledConnection;
use chrono::*;

use grin_history_tools_models::schema;
use grin_history_tools_models::seedcheck::model::{
    SeedCheckResults, SeedCheckResult, SeedCheckConnectAttempt, SeedCheckResultInsertable, SeedCheckResultQueryable
};

pub fn update_seeds(db: &PooledConnection) -> Result<SeedCheckResultQueryable, DbError> {
    info!("--- MAINNET SEEDS ---");
    // Mainnet
    let mainnet_res = check_seeds(false);
    // Testnet
    info!("--- TESTNET SEEDS ---");
    let testnet_res = check_seeds(true);

    let results = SeedCheckResults {
        mainnet: mainnet_res,
        testnet: testnet_res,
    };

    use schema::seed_check_results::dsl::seed_check_results;

    db.transaction::<SeedCheckResultQueryable, DbError, _>(|| {

        let sc_result = SeedCheckResultInsertable {
    		created_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
            results,
        };
        let inserted_results: SeedCheckResultQueryable =
            diesel::insert_into(seed_check_results).values(sc_result).get_result(db)?;

        Ok(inserted_results)
    })
}

pub fn check_seeds(is_testnet: bool) -> Vec<SeedCheckResult> {
    let mut result = vec![]; 
    let (default_seeds, port) = match is_testnet {
        true => (TESTNET_DNS_SEEDS, "13414"),
        false => (MAINNET_DNS_SEEDS, "3414"),
    };

    if is_testnet {
        global::set_local_chain_type(global::ChainTypes::Testnet);
    }

    for s in default_seeds.iter() {
        info!("Checking seed health for {}", s);
        let mut seed_result = SeedCheckResult::default();
        seed_result.url = s.to_string();
        let resolved_dns_entries = resolve_dns_to_addrs(&vec![format!("{}:{}", s, port)]);
        if resolved_dns_entries.is_empty() {
            info!("FAIL - No dns entries found for {}", s);
            result.push(seed_result);
            continue;
        }
        seed_result.dns_resolutions_found = true;
        // Check backwards, last contains the latest (at least on my machine!)
        for r in resolved_dns_entries.iter().rev() {
            let res = check_seed_health(*r, is_testnet);
            if let Ok(p) = res {
                info!("SUCCESS - Performed Handshake with seed for {} at {}. {} - {:?}", s, r, p.info.user_agent, p.info.capabilities);
                //info!("{:?}", p);
                seed_result.success = true;
                seed_result.successful_attempts.push(SeedCheckConnectAttempt{
                    ip_addr: r.to_string(),
                    handshake_success: true,
                    user_agent: Some(p.info.user_agent),
                    capabilities: Some(format!("{:?}", p.info.capabilities)),
                });
            } else {
                seed_result.unsuccessful_attempts.push(SeedCheckConnectAttempt{
                    ip_addr: r.to_string(),
                    handshake_success: false,
                    user_agent: None,
                    capabilities: None,
                });
            }
        }

        if !seed_result.success {
            info!(
                "FAIL - Unable to handshake at any known DNS resolutions for {}",
                s
            );
        }

        result.push(seed_result);
    }
    result
}

fn check_seed_health(addr: p2p::PeerAddr, is_testnet: bool) -> Result<p2p::Peer, SeedCheckError> {
    let capabilities = p2p::types::Capabilities::default();
    let config = p2p::types::P2PConfig::default();
    let adapter = Arc::new(p2p::DummyAdapter {});
    let peers = Arc::new(p2p::Peers::new(
        p2p::store::PeerStore::new("peer_store_root")?,
        adapter,
        config.clone(),
    ));
    let genesis_hash = match is_testnet {
        true => genesis::genesis_test().hash(),
        false => genesis::genesis_main().hash(),
    };

    let handshake = p2p::handshake::Handshake::new(genesis_hash, config.clone());

    match TcpStream::connect_timeout(&addr.0, Duration::from_secs(5)) {
        Ok(stream) => {
            let addr = SocketAddr::new(config.host, config.port);
            let total_diff = Difficulty::from_num(1);

            let peer = p2p::Peer::connect(
                stream,
                capabilities,
                total_diff,
                p2p::PeerAddr(addr),
                &handshake,
                peers,
            )?;
            Ok(peer)
        }
        Err(e) => {
            trace!(
                "connect_peer: on {}:{}. Could not connect to {}: {:?}",
                config.host,
                config.port,
                addr,
                e
            );
            Err(p2p::Error::Connection(e).into())
        }
    }
}
