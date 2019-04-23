use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use std::collections::HashSet;
use crate::{
    domain::playlist,
};
use drive::domain::File;


#[derive(Clone, Debug)]
pub struct AddFiles {
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for AddFiles {
    type Aggregate = playlist::Playlist;
    type Event = playlist::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use kernel::db::schema::{
            drive_files,
            music_playlists_files,
        };
        use diesel::prelude::*;
        use diesel::pg::expression::dsl::any;

        let mut valid_types = HashSet::new();
        valid_types.insert("audio/mpeg".to_string());
        valid_types.insert("audio/mp3".to_string());

        // check that file is owned by same owner
        let files: Vec<File> = drive_files::dsl::drive_files
            .filter(drive_files::dsl::owner_id.eq(self.owner_id))
            .filter(drive_files::dsl::deleted_at.is_null())
            .filter(drive_files::dsl::trashed_at.is_null())
            .filter(drive_files::dsl::id.eq(any(&self.files)))
            .load(ctx)?;

        if files.len() != self.files.len() {
            return Err(KernelError::Validation("File not found.".to_string()));
        }

        // check that files is not already in playlist
        let already_in_playlist: i64 = music_playlists_files::dsl::music_playlists_files
            .filter(music_playlists_files::dsl::playlist_id.eq(aggregate.id))
            .filter(music_playlists_files::dsl::file_id.eq(any(&self.files)))
            .count()
            .get_result(ctx)?;

        if already_in_playlist >= 1 {
            return Err(KernelError::Validation("File is already in playlist.".to_string()));
        }

        // check that file is good mimetype: TODO
        for file in files {
            if !valid_types.contains(&file.type_) {
                return Err(KernelError::Validation("File type is not valid.".to_string()));
            }
        }

        return Ok(());
    }

    fn build_event(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        use kernel::db::schema::{
            music_playlists_files::dsl::music_playlists_files,
        };
        use diesel::prelude::*;

        let files: Vec<playlist::PlaylistFile> = self.files.iter().map(|file_id|
            playlist::PlaylistFile{
                id: uuid::Uuid::new_v4(),
                playlist_id: aggregate.id,
                file_id: *file_id,
            }
        ).collect();

        diesel::insert_into(music_playlists_files)
            .values(&files)
            .execute(ctx)?;

        let data = playlist::EventData::FilesAddedV1(playlist::FilesAddedV1{
            files: self.files.clone(),
        });

        return  Ok((playlist::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
