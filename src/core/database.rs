use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use crate::component::{Component, InitializerFuture};
use surrealdb::types::{RecordId, SurrealNone, SurrealValue};

const COMPONENT_DATA_TABLE: &str = "component_data";

pub fn migrate(data: &mut crate::GlobalData) -> InitializerFuture<'_> {
    Box::pin(async move {
        // Create tables with fields
        data.database
            .query(format!(
                "
DEFINE TABLE IF NOT EXISTS {COMPONENT_DATA_TABLE} SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS enabled  ON TABLE {COMPONENT_DATA_TABLE} TYPE bool;
DEFINE FIELD IF NOT EXISTS config ON TABLE {COMPONENT_DATA_TABLE} TYPE object FLEXIBLE DEFAULT {{}};
        "
            ))
            .await?;
        // Add rows for all components
        for component in data.components.iter() {
            let data_in_db: Option<ComponentData> =
                data.database.select(ComponentData::id_from_component(component)).await?;
            if data_in_db.is_none() {
                // Workaround for no IGNORE on `.insert().content()`
                let component_data = ComponentData::from_component(component);
                let _: Option<ComponentData> =
                    data.database.insert(component_data.id.clone()).content(component_data).await?;
            }
        }
        Ok(())
    })
}

pub async fn get_component_config<T: SurrealValue>(id: String, db: &Surreal<Db>) -> Result<T, crate::Error> {
    let component_config: Option<ComponentConfig<T>> = db.select(ComponentData::id_from_component_string(id.as_str())).await?;
    if let Some(data) = component_config {
        Ok(data.config)
    } else {
        Err(crate::ErrorType::NotFound(format!("No component config found for id {id}")).into())
    }
}

pub async fn set_component_config<T: SurrealValue>(id: String, config: T, db: &Surreal<Db>) -> Result<(), crate::Error> {
    let component_config: ComponentConfig<T> = ComponentConfig { config };
    let _: Option<ComponentConfig<T>> = db.update(ComponentData::id_from_component_string(id.as_str())).content(component_config).await?;
    Ok(())
}

// TODO(feat): component settings
// Issue URL: https://github.com/electricsteve/RustDiscordBot/issues/9
#[derive(SurrealValue)]
pub struct ComponentData {
    pub id: RecordId,
    pub enabled: bool
}

#[derive(SurrealValue)]
pub struct ComponentConfig<T: SurrealValue> {
    pub config: T,
}

#[derive(SurrealValue)]
pub struct Enabled {
    pub enabled: bool,
}

impl ComponentData {
    pub fn from_component(component: &Component) -> Self {
        Self { id: Self::id_from_component(component), enabled: true }
    }
    pub fn id_from_component(component: &Component) -> RecordId {
        RecordId::new(COMPONENT_DATA_TABLE, component.id.clone())
    }
    pub fn id_from_component_string(component: &str) -> RecordId {
        RecordId::new(COMPONENT_DATA_TABLE, component.to_string())
    }
}
