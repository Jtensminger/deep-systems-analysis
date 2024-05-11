use std::path::PathBuf;
use rfd::FileDialog;
use bevy::{prelude::*,
    tasks::{
        futures_lite::future, AsyncComputeTaskPool, Task
    },
};

#[derive(States, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FileImportState {
    #[default]
    Inactive,
    Select,
    Poll,
    Load,
    CleanUp,
}

impl FileImportState  {
    pub fn next(&self) -> Self {
        type S = FileImportState;
        match *self {
            S::Inactive => S::Select,
            S::Select   => S::Poll,
            S::Poll     => S::Load,
            S::Load     => S::CleanUp,
            S::CleanUp  => S::Inactive,
        }
    }
    pub fn reset(&self) -> Self {
        type S = FileImportState;
        match *self { _ => S::Inactive }
    }
}

#[derive(Component)]
pub struct SelectedFileTask(Task<Option<PathBuf>>);

#[derive(Component)]
pub struct SelectedFile {
    pub path_buf: PathBuf,
}

/* 
    This will open the native operating system file dialog,
    schedule an async task, and store the file path in a component
*/
pub fn import_file(
    state: Res<State<FileImportState>>,
    mut next_state: ResMut<NextState<FileImportState>>,
) {
        info!("state: {:?}", state.get());
        next_state.set(state.get().next());
}

pub fn file_dialog_selection(
    mut commands: Commands,
    state: Res<State<FileImportState>>,
    mut next_state: ResMut<NextState<FileImportState>>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    info!("Start Polling");

    let task = thread_pool.spawn(async move {
        FileDialog::new().pick_file()
    });
    commands.spawn(SelectedFileTask(task));
    next_state.set(state.get().next());
}

/* Polls the async task for its completion to get the resulting file */
pub fn poll_for_selected_file(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SelectedFileTask)>,
    state: Res<State<FileImportState>>,
    mut next_state: ResMut<NextState<FileImportState>>,
) {
    println!("Polling");
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(
            future::poll_once(&mut selected_file.0)
        ) {
            info!("{:?}", result);
            if let Some(path_buf) = result {
                commands.spawn(SelectedFile { path_buf });
                next_state.set(state.get().next());
            } else {
                next_state.set(state.get().reset());
            }
            commands.entity(entity).remove::<SelectedFileTask>();
        }
    }
}

pub fn import_clean_up(
    mut commands: Commands,
    selected_file_query: Query<Entity, With<SelectedFile>>,
    state: Res<State<FileImportState>>,
    mut next_state: ResMut<NextState<FileImportState>>,
) {
    let entity = selected_file_query
        .get_single()
        .expect("there should only be 1 selected file");
    commands.entity(entity).remove::<SelectedFile>();
    next_state.set(state.get().next());
}
