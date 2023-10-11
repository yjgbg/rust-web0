use actix_web::{middleware, dev::Service};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
        .wrap_fn(|req,srv| {
            let x = srv.call(req);
            x
        })
            // 格式化请求路径
            .wrap(middleware::NormalizePath::default())
            // 响应压缩
            .wrap(middleware::Compress::default())
            .service(handlers::hello)
            .service(handlers::hello)
            .service(handlers::hello)
            .service(handlers::hello)
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
