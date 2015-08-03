extern crate war_machine;

use war_machine::server::http;
use war_machine::client::cassandra::CassClient;
use war_machine::conf::toml;

static CREATE_KEYSPACE:&'static str = "CREATE KEYSPACE IF NOT EXISTS foo WITH replication = {'class': 'SimpleStrategy', 'replication_factor': '1'};";
static CREATE_TABLE:&'static str = "CREATE TABLE IF NOT EXISTS foo.bar (pk ascii PRIMARY KEY, val1 int)";
static INSERT1:&'static str = "INSERT INTO foo.bar (pk, val1 ) VALUES('abc', 123);";
static INSERT2:&'static str = "INSERT INTO foo.bar (pk, val1 ) VALUES('def', 456);";

fn main() {
	let contact_points = toml::contact_points();
	let cass_client = CassClient::connect(contact_points).unwrap();
	cass_client.session.execute(CREATE_KEYSPACE,0);
	cass_client.session.execute(CREATE_TABLE,0);

	cass_client.session.execute(INSERT1,0);
	cass_client.session.execute(INSERT2,0);

	http::startup(cass_client.session);
}