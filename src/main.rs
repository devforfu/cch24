mod tasks;

use actix_web::web::{service, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    let config = move |cfg: &mut ServiceConfig| {
        cfg
            .service(tasks::task0::hello_world)
            .service(tasks::task0::seek)
            .service(tasks::task1::egregious_encryption)
            .service(tasks::task1::egregious_encryption_check);
    };

    Ok(config.into())
}
