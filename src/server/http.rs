//Include macros to be able to use `insert_routes!`.

use std::error::Error;
use conf::*;
use cassandra::CassSession;

use rustful::{Server, Context, Response, TreeRouter, Handler, StatusCode};

fn default(context: Context, response: Response) {
	let _ = context;
    response.send(format!("Please use a supported endpoint of either /:ks/prepare/:statement or /:ks/execute/:statement",));
}

//DISABLED. See execute()
fn prepare(context: Context, mut response: Response) {
	let session: &CassSession = if let Some(session) = context.global.get() {
        session
    } else {
        //Oh no! Why is the global data not a File instance?!
        context.log.error("the global data should be of the type `CassSession`, but it's not");
        response.set_status(StatusCode::InternalServerError);
        return;
    };
    //Get the value of the path variable `:person`, from below.
    println!("{:#?}",context.variables);
    let ks = match context.variables.get("ks") {
        Some(name) => &name[..],
        None => "unknown ks"
    };
    
    let statement = match context.variables.get("statement") {
        Some(name) => &name[..],
        None => "unknown statemnet"
    };
    
    //session.execute(statement,0).wait();

    //Use the name from the path variable to say hello.
    response.send(format!("Preparing statement '{}' for keyspace, {}!", statement, ks));
}

//FIXME replace this with proper prepare/execute_prepared, instead of blindly executing raw strings
fn execute(context: Context, mut response: Response) {
	let session: &CassSession = if let Some(session) = context.global.get() {
        session
    } else {
        //Oh no! Why is the global data not a File instance?!
        context.log.error("the global data should be of the type `CassSession`, but it's not");
        response.set_status(StatusCode::InternalServerError);
        return;
    };
    //Get the value of the path variable `:person`, from below.
    let ks = match context.variables.get("ks") {
        Some(name) => &name[..],
        None => "unknown ks"
    };

    let statement = match context.variables.get("statement") {
        Some(name) => &name[..],
        None => "unknown statement"
    };
   match session.execute(statement,0).wait() {
   	Ok(result) => {
	    response.send(format!("Executing statement '{}'\n Response: '{}'", statement, result));		
   	}
   	Err(err) => {panic!("{:#?}",err)}
   }

    //Use the name from the path variable to say hello.
//    response.send(format!("Executing statement '{}' for keyspace, {}!", statement, ks));
}

//Dodge an ICE, related to functions as handlers.
struct HandlerFn(fn(Context, Response));

impl Handler for HandlerFn {
    fn handle_request(&self, context: Context, response: Response) {
        self.0(context, response);
    }
}


pub fn startup(cass_session:CassSession) {
	let http_port = toml::http_port().unwrap();
   //Build and run the server.
    let server_result = Server {
        //Turn a port number into an IPV4 host address (0.0.0.0:8080 in this case).
        host: http_port.into(),
		global: Box::new(cass_session).into(),
        //Create a TreeRouter and fill it with handlers.
        handlers: insert_routes!{
            TreeRouter::new() => {
                //Handle requests for root...
                Get: HandlerFn(default),
                //CQL endpoints
                "/:ks/prepare/:statement" => Get: HandlerFn(prepare),
                "/:ks/execute/:statement" => Get: HandlerFn(execute),
            }
        },
 
    //Use default values for everything else.
    ..Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(e) => println!("could not start server: {}", e.description())
    }
}