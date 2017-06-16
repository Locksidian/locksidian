//! UPnP features in order to communicate with the external world. o/

#![allow(dead_code)]

use error::*;
use igd;

fn find_gateway() -> LocksidianResult<igd::Gateway> {
	match igd::search_gateway() {
		Ok(gateway) => Ok(gateway),
		Err(err) => Err(LocksidianError::from_err(err))
	}
}
