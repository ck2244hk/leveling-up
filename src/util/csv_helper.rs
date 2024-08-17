use bevy::app::{App, Plugin};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetApp, AssetLoader, AsyncReadExt, Handle, LoadContext};
use bevy::prelude::TypePath;
use bevy::utils::BoxedFuture;
use std::marker::PhantomData;
use thiserror::Error;

use serde::de::{self, Unexpected};
use serde::{Deserialize, Deserializer};

/// Converts a string to a boolean based on truthy and falsy values
pub fn bool_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.to_lowercase().as_str() {
        "t" | "true" | "1" | "on" | "y" | "yes" => Ok(true),
        "f" | "false" | "0" | "off" | "n" | "no" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"Must be truthy (t, true, 1, on, y, yes) or falsey (f, false, 0, off, n, no)",
        )),
    }
}

/// Plugin to load your asset type `A` from csv files.
pub struct CsvAssetPlugin<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
    delimiter: u8,
}

impl<A> Plugin for CsvAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .init_asset::<LoadedCsv<A>>()
            .register_asset_loader(CsvAssetLoader::<A> {
                extensions: self.extensions.clone(),
                _marker: PhantomData,
                delimiter: self.delimiter,
            });
    }
}

impl<A> CsvAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    /// Create a new plugin that will load assets from files with the given extensions.
    pub fn new(extensions: &[&'static str]) -> Self {
        Self {
            extensions: extensions.to_owned(),
            _marker: PhantomData,
            delimiter: b',',
        }
    }

    /// Change the delimiter used to parse the CSV file.
    ///
    /// The default is ","
    ///
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_common_assets::csv::CsvAssetPlugin;
    /// App::new()
    ///     .add_plugins(CsvAssetPlugin::<TreePosition>::new(&["some_file.csv"]).with_delimiter(b';'));
    /// # #[derive(serde::Deserialize, Asset, TypePath, Debug)]
    /// # struct TreePosition {
    /// #     x: f32,
    /// #     y: f32,
    /// #     z: f32,
    /// # }
    /// ```
    pub fn with_delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }
}

struct CsvAssetLoader<A> {
    extensions: Vec<&'static str>,
    _marker: PhantomData<A>,
    delimiter: u8,
}

/// Possible errors that can be produced by [`CsvAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CsvLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [CSV Error](serde_csv::Error)
    #[error("Could not parse CSV: {0}")]
    CsvError(#[from] csv::Error),
}

/// Asset representing a loaded CSV file with rows deserialized to Assets of type `A`
#[derive(TypePath, Asset)]
pub struct LoadedCsv<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    /// Handles to the Assets the were loaded from the rows of this CSV file
    pub rows: Vec<Handle<A>>,
}

impl<A> AssetLoader for CsvAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = LoadedCsv<A>;
    type Settings = ();
    type Error = CsvLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let mut reader = csv::ReaderBuilder::new()
                .delimiter(self.delimiter)
                .from_reader(bytes.as_slice());
            let mut handles = vec![];
            for (index, result) in reader.deserialize().enumerate() {
                let asset: A = result?;
                handles
                    .push(load_context.add_loaded_labeled_asset(index.to_string(), asset.into()));
            }
            Ok(LoadedCsv { rows: handles })
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}
