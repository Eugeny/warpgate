#![feature(type_alias_impl_trait, let_else, try_blocks)]
mod api;
use poem_openapi::OpenApiService;

pub fn main() {
    let api_service = OpenApiService::new(
        api::get(),
        "Warpgate Web Admin",
        env!("CARGO_PKG_VERSION"),
    )
    .server("/@warpgate/admin/api");
    println!("{}", api_service.spec());
}
