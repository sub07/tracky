pub mod device_selection;

#[derive(Default)]
pub enum Screen {
    DeviceSelection(device_selection::State),
    #[default]
    SongEditor,
}
