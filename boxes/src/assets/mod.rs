use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/// Структура для хранения загруженных ассетов
#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "models/mafia.glb#Scene0")]
    pub level: Handle<Scene>,
}

/// Состояния приложения
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}

/// Плагин для загрузки ассетов
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::InGame)
                .load_collection::<LevelAssets>(),
        );
    }
}
