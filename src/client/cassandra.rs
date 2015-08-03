use cassandra::*;

//use std::net::SocketAddr;

pub struct CassClient {
	pub session:CassSession
}

impl CassClient {
	pub fn connect(contact_points:String) -> Result<CassClient,CassError> {
    	let mut cluster = CassCluster::new();
    	cluster
        	.set_contact_points(contact_points).unwrap()
        	.set_load_balance_round_robin().unwrap();

	    let session_future = CassSession::new().connect(&cluster).wait();
	    Ok(CassClient{session:try!(session_future)})
	}
}