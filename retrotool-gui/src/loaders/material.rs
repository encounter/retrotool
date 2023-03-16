use anyhow::Error;
use bevy::{
    app::{App, Plugin},
    asset::{AddAsset, AssetLoader, BoxedFuture, LoadContext},
};
use binrw::Endian;
use retrolib::format::{cmdl::ModelData, foot::locate_meta, mtrl::MaterialData};

#[derive(Debug, Clone, bevy::reflect::TypeUuid)]
#[uuid = "83269869-1209-408e-8835-bc6f2496e82a"]
pub struct MaterialAsset {
    pub inner: ModelData,
}

pub struct MaterialAssetLoader;

impl Plugin for MaterialAssetLoader {
    fn build(&self, app: &mut App) {
        app.add_asset::<MaterialAsset>().add_asset_loader(MaterialAssetLoader);
    }
}

impl AssetLoader for MaterialAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async move {
            let meta = locate_meta(bytes, Endian::Little)?;
            // let (desc, data, remain) = FormDescriptor::slice(bytes, Endian::Little)?;
            // log::info!("Loading material {:?}", desc);
            let _mtrl = MaterialData::slice(bytes, meta, Endian::Little)?;
            // fs::write("mtrl.out", &mtrl.decompressed)?;
            // load_context.set_default_asset(
            //     LoadedAsset::new(ModelAsset { inner: data, textures })
            //         .with_dependencies(dependencies.into_values().collect()),
            // );
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] { &["mtrl"] }
}