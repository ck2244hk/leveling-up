use bevy::{prelude::*, utils::HashMap};
/// Resource: Representing the index (the default texture will be at 0), and a Texture handle that we can pass to a shader
#[derive(Default, Resource, Deref, DerefMut, Clone, Debug)]
pub struct TexHandleQueue(pub HashMap<usize, Handle<Image>>);
