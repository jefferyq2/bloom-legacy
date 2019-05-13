
// use actix_web::{
//     App,
//     middleware::cors::Cors,
//     http::{header},
//     middleware::Logger,
//     web,
// };
// use rusoto_core::Region;
// use rusoto_s3::S3Client;
// use std::str::FromStr;
// // use notes::api::v1 as notesv1;
// // use contacts::api::v1 as contactsv1;
// // use gallery::api::v1 as galleryv1;
// // use music::api::v1 as musicv1;
// // use bitflow::api::v1 as bitflowv1;
// use sentry_actix::SentryMiddleware;
// use kernel::{
//     db::DbActor,
//     api,
//     api::middlewares,
//     config,
//     accounts::api::v1 as accountsv1,
// };


// pub fn init<A: actix_service::NewService, B>(db: actix::Addr<DbActor>, cfg: config::Config) -> App<A, B> {
//     let region = Region::from_str(&cfg.aws_region()).expect("AWS region not valid");
//     let api_state = api::State{
//         db,
//         config: cfg,
//         s3_client: S3Client::new(region),
//     };


//     return App::new()
//         .data(api_state.clone())
//         .wrap(
//             Cors::new()
//                 .send_wildcard() // TODO...
//                 .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
//                 .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
//                 .max_age(3600)
//         )
//         .wrap(Logger::default())
//         .service(web::resource("/").route(web::get().to(api::index)));

//     // App::with_state(api_state.clone())
//     // .middleware(middlewares::RequestIdMiddleware)
//     // .middleware(middlewares::LoggerMiddleware)
//     // .middleware(middlewares::DefaultHeaders)
//     // .middleware(
//     //     // cors 2 times because otherwise authmiddleware doesn't works...
//     //     Cors::build()
//     //     .send_wildcard()
//     //     .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
//     //     .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
//     //     .max_age(3600)
//     //     .finish()
//     // )
//     // .middleware(middlewares::AuthMiddleware)
//     // .middleware(SentryMiddleware::new())
//     // .default_resource(|r| r.f(api::route_404))
//     // .configure(|app| {
//     //     Cors::for_app(app)
//     //         // .allowed_origin("*")
//     //         .send_wildcard()
//     //         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
//     //         .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
//     //         .max_age(3600)
//     //         .resource("/", |r| r.method(http::Method::GET).f(api::index))


//             // // gallery
//             // .resource("/gallery/v1/media", |r| r.method(http::Method::GET).f(galleryv1::media::get))
//             // .resource("/gallery/v1/albums", |r| {
//             //     r.method(http::Method::GET).f(galleryv1::albums::get);
//             //     r.method(http::Method::POST).with_config(galleryv1::albums::post, api::json_default_config);
//             // })
//             // .resource("/gallery/v1/albums/{album_id}", |r| {
//             //     r.method(http::Method::GET).with(galleryv1::albums::album::get);
//             //     r.method(http::Method::DELETE).with(galleryv1::albums::album::delete);
//             //     r.method(http::Method::PUT).with_config(galleryv1::albums::album::put, api::json_default_config_path);
//             // })
//             // .resource("/gallery/v1/albums/{album_id}/add", |r| {
//             //     r.method(http::Method::POST).with_config(galleryv1::albums::album::add::post, api::json_default_config_path);
//             // })
//             // .resource("/gallery/v1/albums/{album_id}/remove", |r| {
//             //     r.method(http::Method::POST).with_config(galleryv1::albums::album::remove::post, api::json_default_config_path);
//             // })

//             // // music
//             // .resource("/music/v1/musics", |r| r.method(http::Method::GET).f(musicv1::musics::get))
//             // .resource("/music/v1/playlists", |r| {
//             //     r.method(http::Method::GET).f(musicv1::playlists::get);
//             //     r.method(http::Method::POST).with_config(musicv1::playlists::post, api::json_default_config);
//             // })
//             // .resource("/music/v1/playlists/{playlist_id}", |r| {
//             //     r.method(http::Method::GET).with(musicv1::playlists::playlist::get);
//             //     r.method(http::Method::DELETE).with(musicv1::playlists::playlist::delete);
//             //     r.method(http::Method::PUT).with_config(musicv1::playlists::playlist::put, api::json_default_config_path);
//             // })
//             // .resource("/music/v1/playlists/{playlist_id}/add", |r| {
//             //     r.method(http::Method::POST).with_config(musicv1::playlists::playlist::add::post, api::json_default_config_path);
//             // })
//             // .resource("/music/v1/playlists/{playlist_id}/remove", |r| {
//             //     r.method(http::Method::POST).with_config(musicv1::playlists::playlist::remove::post, api::json_default_config_path);
//             // })


//     //         .register()
//     // })
// }