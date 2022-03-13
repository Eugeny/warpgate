pub use sea_schema::migration::*;

mod m00001_create_ticket;
mod m00002_create_session;
mod m00003_create_recording;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m00001_create_ticket::Migration),
            Box::new(m00002_create_session::Migration),
            Box::new(m00003_create_recording::Migration),
        ]
    }
}
