//! Blocks management endpoint.

use iron::prelude::*;
use persistence::prelude::*;

use blockchain::metric::*;

use blockchain::block::BlockRepository;
use blockchain::peer::PeerRepository;
use blockchain::identity::IdentityRepository;

pub fn get_all(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_connection()?;
    
    let metrics = vec![
        get_blocks_metric(&*connection)?,
        get_peers_metric(&*connection)?,
        get_identities_metric(&*connection)?
    ];
    
    response!(Ok, metrics)
}

fn get_blocks_metric(connection: &SqliteConnection) -> IronResult<Metric<i64>> {
    let repository = BlockRepository::new(&connection);
    
    match repository.count() {
        Ok(count) => Ok(Metric::new("Blocks", count)),
        Err(err) => error!(InternalServerError, {"error": err.description()})
    }
}

fn get_peers_metric(connection: &SqliteConnection) -> IronResult<Metric<i64>> {
    let repository = PeerRepository::new(&connection);
    
    match repository.count() {
        Ok(count) => Ok(Metric::new("Peers", count)),
        Err(err) => error!(InternalServerError, {"error": err.description()})
    }
}

fn get_identities_metric(connection: &SqliteConnection) -> IronResult<Metric<i64>> {
    let repository = IdentityRepository::new(&connection);
    
    match repository.count() {
        Ok(count) => Ok(Metric::new("Identities", count)),
        Err(err) => error!(InternalServerError, {"error": err.description()})
    }
}