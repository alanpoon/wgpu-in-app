use ambient_native_std::{
    asset_cache::{AssetCache, SyncAssetKeyExt,SyncAssetKey},
    asset_url::{ContentBaseUrlKey, UsingLocalDebugAssetsKey},
    download_asset::AssetsCacheOnDisk,
    download_asset::ReqwestClientKey,
};
use ambient_settings::SettingsKey;

pub mod client;
mod shared;

use ambient_physics::physx::PhysicsKey;
use anyhow::Context;
use serde::Deserialize;
use winit::event_loop::EventLoop;
use std::path::Path;
use ambient_network::native::client::ResolvedAddr;
use std::path::PathBuf;
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;
use ambient_audio::AudioStream;
use ambient_app::{ffi::IOSViewObj,AppWrapper};
pub mod ios;
#[cfg(target_os = "android")]
pub fn new_main(eventloop:EventLoop<()>,android_app:AndroidApp){
    tracing::info!("start main..");
    
    ambient_dirs::init(android_app.clone());

    ambient_git_rev_init::init().expect("Should be called exactly once");

    shared::components::init().unwrap();

    let audio_bool = false;
    let audio_stream = if audio_bool {
        match AudioStream::new() {
            Ok(v) => Some(v),
            Err(err) => {
                tracing::error!("Failed to initialize audio stream: {err}");
                None
            }
        }
    } else {
        None
    };

    let is_debug = false;
    //box_init();
   
    tracing::info!("before event_loop");
    let  aw = AppWrapper::new_with_event_loop(eventloop);

    tracing::info!("after event_loop");


    // let status = aw.run_blocking(client::init2,android_app,Box::new(||{
    //     //shared::components::init().unwrap();

    // }));


}

pub fn new(eventloop:EventLoop<()>){
    ambient_git_rev_init::init().expect("Should be called exactly once");

    shared::components::init().unwrap();

    let audio_bool = false;
    let audio_stream = if audio_bool {
        match AudioStream::new() {
            Ok(v) => Some(v),
            Err(err) => {
                tracing::error!("Failed to initialize audio stream: {err}");
                None
            }
        }
    } else {
        None
    };

    let is_debug = false;
    //box_init();

    tracing::info!("before event_loop");
    let  aw = AppWrapper::new_with_event_loop(eventloop);

    tracing::info!("after event_loop");
    let status = aw.run_blocking(client::init,Box::new(||{
        shared::components::init().unwrap();
    }));
}
#[cfg(target_os="ios")]
pub fn new_ios(obj:ambient_app::ffi::IOSViewObj)->AppWrapper{
    let resource_path = ios::get_resource_path();
    println!("resource_path {:?}",resource_path);
    ambient_git_rev_init::init().expect("Should be called exactly once");

    shared::components::init().unwrap();

    let audio_bool = false;
    let audio_stream = if audio_bool {
        match AudioStream::new() {
            Ok(v) => Some(v),
            Err(err) => {
                tracing::error!("Failed to initialize audio stream: {err}");
                None
            }
        }
    } else {
        None
    };

    let is_debug = false;
    //box_init();
    tracing::info!("before event_loop");
    println!("before new_with_vie");
    
    let  mut aw = AppWrapper::new_with_view(obj,Box::new(||{}),client::init2);
    //aw.run_with_view(client::init);
    println!("runwithview");
    return aw

}
#[cfg(target_os="ios")]
pub fn run_with_view(aw:&mut AppWrapper){
    aw.run_with_view(client::init);
}