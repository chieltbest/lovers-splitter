#![feature(type_alias_impl_trait, const_async_blocks)]
#![no_std]
extern crate alloc;

use asr::{Address, future::next_tick, PointerSize, print_message, Process, timer};
use asr::deep_pointer::DeepPointer;
use asr::future::retry;
use asr::game_engine::unity::mono::{Image, Module, UnityPointer};
use asr::settings::Gui;
use asr::watcher::Watcher;

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

asr::async_main!(nightly);
asr::panic_handler!();


fn get_save_slots(usd: &UnityPointer<8>, process: &Process, module: &Module, img: &Image) -> Option<[Address; 3]> {
    usd.deref::<[u32; 3]>(process, module, img)
        .map(|r| r.map(|ptr| ptr.into())).ok()
}

#[derive(Gui)]
struct Settings {
    /// Split on Campaign Start
    #[default = false]
    split_start_campaign: bool,
}

async fn main() {
    let mut settings = Settings::register();

    loop {
        print_message("get process");
        let process = Process::wait_attach("LoversInADangerousSpacetime.exe").await;

        print_message("connection established!");

        process
            .until_closes(async {
                let level_addr = retry(|| process.get_module_address("LoversInADangerousSpacetime.exe").ok()).await
                    + 0xdcebb0;
                print_message("got level addr");

                let mut level_watch = Watcher::<u32>::new();

                // let img = module.wait_get_default_image(&process).await;
                let (module, img) = retry(|| {
                    let m = Module::attach_auto_detect(&process)?;
                    let img = m.get_default_image(&process)?;
                    Some((m, img))
                }).await;
                print_message("got image");

                // let game_state = UnityPointer::<8>::new("GameStateManager", 0,
                //                                         &[
                //                                             "0x0", // _instance
                //                                             "0x10", // currentState
                //                                         ]);
                // let mut state_watch = Watcher::<u32>::new();


                let save_index = UnityPointer::<8>::new("SaveDataManager", 0,
                                                        &[
                                                            "0x1C", // _sharedInstance
                                                            "0xC", // currentSaveSlotIndex
                                                        ]);
                let mut save_index_watch = Watcher::<i32>::new();

                let usd = UnityPointer::<8>::new("SaveDataManager", 0,
                                                 &[
                                                     "0x1C", // _sharedInstance
                                                     "0x8", // currentLoadedUserSaveData
                                                     "0x8", // saveSlot1
                                                 ]);
                let mut save_ptr_watch = Watcher::<[Address; 3]>::new();
                let mut sdm_level_watch = Watcher::<Option<u32>>::new();

                let input_enabled = UnityPointer::<8>::new("Controls", 0,
                                                           &[
                                                               "0x4C", // _playerInputDisabledCount
                                                           ]);
                let mut input_enabled_watch = Watcher::<u32>::new();

                loop {
                    settings.update();

                    // if let Some(new) = state_watch.update(game_state.deref(&process, &module, &img).ok()) {
                    //     if new.changed() {
                    //         print_limited::<128>(&format_args!("gameState: {:?}", new.current));
                    //     }
                    // }

                    if let Some(new_save_index) = save_index_watch.update(save_index.deref(&process, &module, &img).ok()) {
                        // if new_save_index.changed() {
                        //     print_limited::<128>(&format_args!("save index: {:?}", new_save_index.current));
                        // }

                        let save_ptrs = get_save_slots(&usd, &process, &module, &img);

                        if let Some(new_ptr) = save_ptr_watch.update(save_ptrs) {
                            // if new_ptr.changed() {
                            //     print_limited::<128>(&format_args!("save ptr: {:?} ({:?})", new_ptr.current, new_save_index.current));
                            // }

                            if new_save_index.current >= 0 {
                                let cur_save_ptr = new_ptr.current[new_save_index.current as usize];

                                if new_ptr.old[new_save_index.current as usize].is_null() && !cur_save_ptr.is_null() {
                                    timer::start();
                                }

                                let level = DeepPointer::<8>::new(cur_save_ptr, PointerSize::Bit32,
                                                                  &[
                                                                      0x8, // globalData
                                                                      // 0x10, // introCutsceneComplete
                                                                      // 0x11, // tutorialComplete
                                                                      // 0x14, // totalNumberOfMacGuffinsAcquired
                                                                      0xC, // resumeData
                                                                      0x1C, // currentLevelNumber
                                                                  ]).deref(&process).ok();

                                if let Some(new_lvl) = sdm_level_watch.update(Some(level)) {
                                    if new_lvl.changed() {
                                        // print_limited::<128>(&format_args!("lvl change: {:?}", new_lvl.current));

                                        if new_lvl.current != None || new_lvl.old == Some(5) {
                                            timer::split();
                                        }
                                    }

                                    if let Some(new) = input_enabled_watch.update(input_enabled.deref(&process, &module, &img).ok()) {
                                        // if new.changed() {
                                        //     print_limited::<128>(&format_args!("playerInputDisabledCount: {:?}", new.current));
                                        // }

                                        let campaign = DeepPointer::<8>::new(cur_save_ptr, PointerSize::Bit32,
                                                                             &[
                                                                                 0x8, // globalData
                                                                                 // 0x10, // introCutsceneComplete
                                                                                 // 0x11, // tutorialComplete
                                                                                 // 0x14, // totalNumberOfMacGuffinsAcquired
                                                                                 0xC, // resumeData
                                                                                 0x8, // worldPrefix
                                                                                 0x8, // string length
                                                                             ]).deref::<u32>(&process).ok();

                                        if let Some(len) = campaign {
                                            if len == 11 { // "KingCepheus"
                                                if new.changed_from_to(&0, &2) {
                                                    // final split
                                                    timer::split();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(new) = level_watch.update(process.read(level_addr).ok()) {
                        // if new.changed() {
                        //     print_limited::<128>(&format_args!("level: {:?}", new.current));
                        // }

                        if new.changed_to(&4) {
                            timer::reset();
                        }

                        if settings.split_start_campaign && new.changed_from_to(&6, &7) {
                            timer::start();
                        }
                    }

                    next_tick().await;
                }
            })
            .await;
    }
}
