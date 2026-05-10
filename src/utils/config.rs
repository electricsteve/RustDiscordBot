macro_rules! component_config {
    (
        $config_name:ident,
        $component_id:expr,
        $(
            $(#[$field_attr:meta])*
            pub $field:ident : $ty:ty
        )*
    ) => {
        static SETTINGS: OnceLock<RwLock<$config_name>> = OnceLock::new();

        #[derive(SurrealValue, Clone, Default, Debug, PartialEq)]
        pub struct $config_name {
            $(
                $(#[$field_attr])*
                pub $field: $ty,
            )*
        }

        async fn ensure_loaded(db: &Surreal<Db>) -> Result<(), crate::Error> {
            if SETTINGS.get().is_none() {
                let cfg = get_component_config($component_id, db).await?;
                let _ = SETTINGS.set(RwLock::new(cfg)); // ignore race if another task set first
            }
            Ok(())
        }

        pub async fn get_config(db: &Surreal<Db>) -> Result<$config_name, crate::Error> {
            ensure_loaded(db).await?;
            let lock = SETTINGS.get().ok_or_else(|| {
                crate::ErrorType::LockError("Config not initialized while it should have been".to_string())
            })?;
            let cfg = lock.read().await;
            Ok(cfg.clone())
        }

        pub async fn update_config(db: &Surreal<Db>, new_cfg: $config_name) -> Result<(), crate::Error> {
            ensure_loaded(db).await?;
            set_component_config($component_id, new_cfg.clone(), db).await?;
            let lock = SETTINGS.get().ok_or_else(|| {
                crate::ErrorType::LockError("Config not initialized while it should have been".to_string())
            })?;
            let mut cfg = lock.write().await;
            *cfg = new_cfg;
            Ok(())
        }
    };
}

pub(crate) use component_config;
