use sea_schema::migration::sea_orm::Schema;
use sea_schema::migration::sea_query::*;
use sea_schema::migration::*;

pub mod recording {
    use crate::m00002_create_session::session;
    use sea_orm::entity::prelude::*;
    use uuid::Uuid;

    #[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
    #[sea_orm(rs_type = "String", db_type = "String(Some(16))")]
    pub enum RecordingKind {
        #[sea_orm(string_value = "terminal")]
        Terminal,
        #[sea_orm(string_value = "traffic")]
        Traffic,
    }

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "recordings")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub name: String,
        pub started: DateTimeUtc,
        pub ended: Option<DateTimeUtc>,
        pub session_id: Uuid,
        pub kind: RecordingKind,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {
        Session,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Self::Session => Entity::belongs_to(session::Entity)
                    .from(Column::SessionId)
                    .to(session::Column::Id)
                    .into(),
            }
        }
    }

    impl Related<session::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Session.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m00003_create_recording"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        manager
            .create_table(schema.create_table_from_entity(recording::Entity))
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(recording::Entity)
                    .name("recording__unique__session_id__name")
                    .unique()
                    .col(recording::Column::SessionId)
                    .col(recording::Column::Name)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("recording__unique__session_id__name")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(recording::Entity).to_owned())
            .await
    }
}
