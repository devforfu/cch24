mod tasks;

use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    Ok(config.into())
}

fn config(cfg: &mut ServiceConfig) {
    cfg
        .service(tasks::task0::hello_world)
        .service(tasks::task0::seek)
        .service(tasks::task1::egregious_encryption)
        .service(tasks::task1::egregious_encryption_check)
        .service(tasks::task2::manifest);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    macro_rules! test_service {
        () => {{
            use actix_web::App;
            test::init_service(App::new().configure(config)).await
        }};
    }

    macro_rules! post {
        ($uri:expr, $data:expr) => {
            test::TestRequest::post().uri($uri).set_payload($data).to_request()
        };
    }

    #[actix_web::test]
    async fn test_task2_manifest_ok() {
        let service = test_service!();
        let request = post!["/5/manifest", "test test test"];
        let response = test::call_service(&service, request).await;
        assert!(response.status().is_success());
    }

    #[actix_web::test]
    async fn test_task2_manifest_toml() {
        let toml_example =
            "[package]\n\
            name = 'not-a-gift-order'\n\
            authors = ['Not Santa']\n\
            keywords = ['Christmas 2024']\n\
            \n\
            [[package.metadata.orders]]\n\
            item = 'Toy car'\n\
            quantity = 2\n\
            \n\
            [[package.metadata.orders]]\n\
            item = 'Lego brick'\n\
            quantity = 230\n\
        ";
        let service = test_service!();
        let request = post!["/5/manifest", toml_example];
        let response = test::call_service(&service, request).await;
        assert!(response.status().is_success());
    }
}