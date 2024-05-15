#![feature(type_alias_impl_trait, const_async_blocks)]
#![no_std]
extern crate alloc;

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

use core::fmt::{Debug};
use asr::{Address, future::next_tick, print_limited, print_message, Process, timer};
use asr::deep_pointer::DeepPointer;
use asr::game_engine::unity::mono::Module;
use asr::timer::{pause_game_time, resume_game_time};
use bytemuck::CheckedBitPattern;

asr::async_main!(nightly);
asr::panic_handler!();

fn readout<T: Debug + CheckedBitPattern>(process: &Process, static_table: Address, offset: u32) {
    let var: Result<T, _> = process.read(static_table + offset);
    if let Ok(val) = var {
        print_limited::<128>(&format_args!("{:X?}", val));
    }
}

async fn main() {
    // TODO: Set up some general state and settings.

    loop {
        print_message("connecting...");

        // let process = Process::wait_attach("LoversInADangerousSpacetime.exe").await;
        // let process = Process::wait_attach("wine").await;
        let process = Process::wait_attach("Receiver.exe").await;

        print_message("connection established!");

        process
            .until_closes(async {
                print_message("trying to get mod");
                let module = Module::wait_attach_auto_detect(&process).await;
                print_message("attached");
                let img = module.wait_get_default_image(&process).await;

                if true {
                    print_limited::<128>(&"classes:");
                    for class in img.classes(&process, &module) {
                        // print_limited::<128>(
                        //     &format_args!("Class: {}",
                        //                   class.get_name::<128>(&process, &module).unwrap().validate_utf8().unwrap()));
                        // if let Some(parent) = class.get_parent(&process, &module) {
                        //     print_limited::<128>(
                        //         &format_args!("  Parent: {:?}",
                        //                       parent.get_name::<128>(&process, &module).unwrap().validate_utf8().unwrap()));
                        // }
                        let static_addr = class.get_static_table(&process, &module);
                        if let Some(static_addr) = static_addr {
                            if false {
                                print_limited::<128>(&format_args!("  Static table addr: {:?}", static_addr));
                            } else {
                                let size = class.fields(&process, &module).last()
                                    .and_then(|f| f.get_offset(&process, &module))
                                    .unwrap_or(0);
                                print_limited::<128>(
                                    &format_args!("{:?}-{:?} {}",
                                                  static_addr,
                                                  static_addr + size + 4,
                                                  class.get_name::<128>(&process, &module).unwrap().validate_utf8().unwrap()));
                            }
                        }
                        // for field in class.fields(&process, &module) {
                        //     let offs = field.get_offset(&process, &module).unwrap();
                        //     print_limited::<128>(&format_args!("  - {:?} {:X?}",
                        //                                        field.get_name::<128>(&process, &module).unwrap().validate_utf8().unwrap(),
                        //                                        offs));
                        //     if let Some(addr) = static_addr {
                        //         readout::<i32>(&process, addr, offs);
                        //         readout::<i64>(&process, addr, offs);
                        //         readout::<u32>(&process, addr, offs);
                        //         readout::<u64>(&process, addr, offs);
                        //         readout::<f32>(&process, addr, offs);
                        //         readout::<f64>(&process, addr, offs);t
                        //     }
                        // }
                    }
                }

                // // "optionsmenuscript" -> "show_menu"
                //
                // // print_message("getting AimScript");
                // let obj = img.wait_get_class(&process, &module, "optionsmenuscript").await;
                // let static_table = obj.wait_get_static_table(&process, &module).await;
                // let offs = obj.wait_get_field_offset(&process, &module, "show_menu").await;
                //
                // let unityplayer_addr = process.get_module_address("UnityPlayer.dll").unwrap();
                // print_limited::<128>(&format_args!("{:x?}", unityplayer_addr));
                // let aimscript_ptr = DeepPointer::<16>::new_64bit(
                //     unityplayer_addr,
                //     &[
                //         0x1689000+0x268f08,
                //         0x2ec,
                //         0xe0,
                //         0xf0,
                //         0x50,
                //         0x10,
                //         0x28,
                //         0x0,
                //     ]);
                //
                // #[derive(Default)]
                // struct Vars {
                //     show_menu: u8,
                //     tapes_heard: u32,
                //     tapes_remaining: u32,
                //     total_tapes: u32,
                //     tape_in_progress: u32,
                //     unplayed_tapes: u32,
                //     tape_count: u32,
                // }
                // #[derive(Default)]
                // struct CalcVars {
                //     full_tapes_remaining: u32,
                // }
                //
                // let mut prev_vars = Vars::default();
                // let mut prev_calc_vars = CalcVars::default();
                //
                // let aimscript_addr = aimscript_ptr.deref_offsets(&process).unwrap();

                loop {
                    // let vars = Vars {
                    //     show_menu: process.read(static_table + offs).unwrap(),
                    //     tapes_heard: process.read_pointer_path32(aimscript_addr, &[0x148, 0x18]).unwrap(),
                    //     tapes_remaining: process.read_pointer_path32(aimscript_addr, &[0x150, 0x18]).unwrap(),
                    //     total_tapes: process.read_pointer_path32(aimscript_addr, &[0x158, 0x18]).unwrap(),
                    //     tape_in_progress: process.read_pointer_path32(aimscript_addr, &[0x2e4]).unwrap(),
                    //     unplayed_tapes: process.read_pointer_path32(aimscript_addr, &[0x2e8]).unwrap(),
                    //     tape_count: process.read_pointer_path32(aimscript_addr, &[0x2ec]).unwrap(),
                    // };
                    // timer::set_variable_int("show_menu", vars.show_menu);
                    // timer::set_variable_int("tapes_heard", vars.tapes_heard);
                    // timer::set_variable_int("tapes_remaining", vars.tapes_remaining);
                    // timer::set_variable_int("total_tapes", vars.total_tapes);
                    // timer::set_variable_int("tape_in_progress", vars.tape_in_progress);
                    // timer::set_variable_int("unplayed_tapes", vars.unplayed_tapes);
                    // timer::set_variable_int("tape_count", vars.tape_count);
                    //
                    // let calc_vars = CalcVars {
                    //     full_tapes_remaining: vars.tapes_remaining - vars.unplayed_tapes,
                    // };
                    // timer::set_variable_int("full_tapes_remaining", calc_vars.full_tapes_remaining);
                    //
                    //
                    // if calc_vars.full_tapes_remaining != prev_calc_vars.full_tapes_remaining {
                    //     if calc_vars.full_tapes_remaining > prev_calc_vars.full_tapes_remaining {
                    //         timer::reset();
                    //         timer::start();
                    //     } else {
                    //         timer::split();
                    //     }
                    // }
                    //
                    // if vars.show_menu != prev_vars.show_menu {
                    //     if vars.show_menu > 0 {
                    //         pause_game_time();
                    //     } else {
                    //         resume_game_time();
                    //     }
                    // }
                    //
                    // prev_vars = vars;
                    // prev_calc_vars = calc_vars;

                    next_tick().await;
                }
            }).await;
    }
}
