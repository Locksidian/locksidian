//! Peer command line interface.

use error::*;
use persistence::prelude::*;

use blockchain::get_current_timestamp;
use blockchain::peer::*;
use blockchain::identity::identity_cli::get_active_identity;

/// Register a batch of `Peer`s into the registry.
pub fn register_batch(peers: &mut Vec<Peer>, repository: &PeerRepository, current_address: &str) -> LocksidianResult<()> {
    for peer in peers.iter_mut() {
		register(peer, &repository, current_address)?;
	}
	
	Ok(())
}

/// Register a `Peer` into the registry.
pub fn register(peer: &mut Peer, repository: &PeerRepository, current_address: &str) -> LocksidianResult<()> {
    match peer.address().eq(current_address) {
        true => Ok(()),
        false => {
            peer.set_last_recv(get_current_timestamp());
            peer.set_last_sent(get_current_timestamp());

            match repository.get(&peer.identity()) {
                Some(mut entity) => update_existing_peer(&mut entity, &repository),
                None => register_new_peer(&peer, &repository)
            }
        }
    }
}

/// Update an existing `PeerEntity`.
fn update_existing_peer(entity: &mut PeerEntity, repository: &PeerRepository) -> LocksidianResult<()> {
    entity.last_recv = get_current_timestamp() as i32;
    entity.last_sent = get_current_timestamp() as i32;

    match repository.update(&entity) {
        Ok(1) => Ok(()),
        Ok(_) => Err(LocksidianError::new(String::from("An unexpected number of rows were updated in the registry"))),
        Err(err) => Err(LocksidianError::from_err(err))
    }
}

/// Insert a new `Peer` into the registry.
fn register_new_peer(peer: &Peer, repository: &PeerRepository) -> LocksidianResult<()> {
    match PeerEntity::new(&peer) {
        Ok(entity) => match repository.save(&entity) {
            Ok(1) => Ok(()),
            Ok(_) => Err(LocksidianError::new(String::from("An unexpected number of rows were updated in the registry"))),
            Err(err) => Err(LocksidianError::from_err(err))
        },
        Err(err) => Err(LocksidianError::from_err(err))
    }
}

/// Create a `Peer` structure based on the current `Identity` and address.
pub fn current_identity_as_peer(connection: &SqliteConnection, address: String) -> LocksidianResult<Peer> {
    match get_active_identity(&*connection) {
		Ok(identity) => {
            let key = identity.public_key_to_hex()?;
            Peer::new(key, address)
        },
		Err(err) => Err(LocksidianError::from_err(err))
	}
}