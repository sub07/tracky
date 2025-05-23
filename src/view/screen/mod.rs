pub mod device_selection;
pub mod song_editor;

#[derive(Default, Debug)]
pub enum Screen {
    DeviceSelection(device_selection::State),
    #[default]
    SongEditor,
}
