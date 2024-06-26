pub mod record_main {
    use crate::record::mouse_monitor;
    use crate::MOUSE_MOVE_TIME;
    use crate::MOUSE_PATH;
    use crate::MOUSE_THREAD_FLAG;
    use crate::MOUSE_THREAD_START;
    use crate::NOW_DIR;
    use crate::PAUSE_FLAG;
    use crate::PAUSE_TIME;
    use crate::SCREEN_SHOT_FLAG;
    use crate::SHOULD_STOP;
    use crate::SHOULD_STOP_KEYBOARD;
    use crate::START_TIME;
    use crate::TEXTSHOT_ACTION_TIME;
    use chrono::prelude::*;
    use clipboard::ClipboardContext;
    use clipboard::ClipboardProvider;
    use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};
    use rdev::listen;
    use std::process::Command;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Instant;
    use std::{fs::File, fs::OpenOptions, io::Write, thread, thread::sleep, time::Duration};
    use tauri::window;
    use tauri::{Manager, Window};
    #[derive(Clone, serde::Serialize)]
    struct Payload {
        message: String,
    }
    #[tauri::command]
    //参数为recordstart: bool,window: Window
    pub async fn start_record(window: Window) {
        println!("你好");
        let mut status = "init";
        let device_state = DeviceState::new();
        let mut now_dir = String::new();
        loop {
            sleep(Duration::from_millis(200));
            println!("程序开始");
            println!("F2启动截图功能");
            let local: DateTime<Local> = Local::now();
            now_dir = local.format("%Y-%m-%d %H-%M-%S").to_string();
            std::fs::create_dir_all(format!("../Automated-testing/result/{}", now_dir)).unwrap();
            let mut global_now_dir = NOW_DIR.lock().unwrap();
            *global_now_dir = now_dir.clone();
            let mut mouse_path = MOUSE_PATH.lock().unwrap();
            *mouse_path = now_dir.clone();

            status = "started";
            let mut start_time = START_TIME.lock().unwrap();
            *start_time = Instant::now();
            let mut pause_time = PAUSE_TIME.lock().unwrap();
            *pause_time = 0;

            *MOUSE_MOVE_TIME.lock().unwrap() = Some(start_time.elapsed().as_millis());
            let mut mouse_flag = MOUSE_THREAD_FLAG.lock().unwrap();
            *mouse_flag = false;
            let mut stop_flag = SHOULD_STOP.lock().unwrap();
            *stop_flag = false;
            break;
        }

        //键盘监听文件
        let mut save_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(format!(
                "../Automated-testing/result/{}/record.txt",
                now_dir.clone()
            ))
            .unwrap();

        File::create(format!(
            "../Automated-testing/result/{}/record_result.txt",
            now_dir
        ))
        .unwrap();

        // 鼠标监听线程
        let mut mouse_flag = MOUSE_THREAD_START.lock().unwrap();
        if *mouse_flag == false {
            *mouse_flag = true;
            let window_clone = window.clone();
            thread::spawn(move || {
                if let Err(error) = listen(move |event| mouse_monitor::mouse::callback(event,&window_clone)) {
                    println!("Error: {:?}", error);
                }
            });
        }

        //按下监听闭包
        let _guard_down = device_state.on_key_down(|key| {
            // let keys = device_state.get_keys();
            if *key != Keycode::F1 {
                let pause_time = PAUSE_TIME.lock().unwrap();
                let duration_key =
                    START_TIME.lock().unwrap().elapsed().as_millis() - *pause_time;
                println!("{}ms, 捕捉到键盘输入按下{:?}", duration_key, key);
                let output = format!("{}ms,key_down,{:#?}\n", duration_key, key);

                if *key != Keycode::F1
                    && *key != Keycode::F2
                    && *key != Keycode::F3
                    && *key != Keycode::F4
                    && *key != Keycode::F5
                    && *key != Keycode::F6
                    && *key != Keycode::F7
                    && *key != Keycode::F8
                {
                    // window.emit(...) // Uncomment when necessary
                    let global_now_dir = NOW_DIR.lock().unwrap();
                    let now_dir = global_now_dir.clone();
                    let mut save_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(format!(
                            "../Automated-testing/result/{}/record.txt",
                            now_dir.clone()
                        ))
                        .unwrap();

                    File::create(format!(
                        "../Automated-testing/result/{}/record_result.txt",
                        now_dir
                    ))
                    .unwrap();
                    save_file.write_all(output.as_bytes()).unwrap();
                }
            }else if *key == Keycode::F1 {
                let mut stop_flag = SHOULD_STOP.lock().unwrap();
                *stop_flag = true;
            }
        });

        //抬起监听闭包
        let _guard_up = device_state.on_key_up(|key| {
            // let keys = device_state.get_keys();
            if *key != Keycode::F1 {
                let pause_time = PAUSE_TIME.lock().unwrap();
                let duration_key =
                    START_TIME.lock().unwrap().elapsed().as_millis() - *pause_time;
                println!("{}ms, 捕捉到键盘抬起{:?}", duration_key, key);
                let output = format!("{}ms,key_up,{:#?}\n", duration_key, key);

                if *key != Keycode::F1
                    && *key != Keycode::F2
                    && *key != Keycode::F3
                    && *key != Keycode::F4
                    && *key != Keycode::F5
                    && *key != Keycode::F6
                    && *key != Keycode::F7
                    && *key != Keycode::F8
                {
                    let global_now_dir = NOW_DIR.lock().unwrap();
                    let now_dir = global_now_dir.clone();
                    let mut save_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(format!(
                            "../Automated-testing/result/{}/record.txt",
                            now_dir.clone()
                        ))
                        .unwrap();

                    File::create(format!(
                        "../Automated-testing/result/{}/record_result.txt",
                        now_dir
                    ))
                    .unwrap();
                    save_file.write_all(output.as_bytes()).unwrap();
                }
            }
        });

        loop {
            let should_stop = {
                let stop_flag = SHOULD_STOP.lock().unwrap();
                *stop_flag
            };
            if should_stop {
                println!("录制被前端终止");
                let mut mouse_flag = MOUSE_THREAD_FLAG.lock().unwrap();
                *mouse_flag = true;
                if status == "started" {
                    println!(
                        "\n经过了：{}毫秒",
                        START_TIME.lock().unwrap().elapsed().as_millis()
                    );
                    return;
                }
            }

            //判断暂停录制
            let mut pause_state = {
                let pause_flag = PAUSE_FLAG.lock().unwrap();
                *pause_flag
            };
            let mut m_flag = false;
            let mut pause_start_time: u128 = 0;
            let mut pause_end_time: u128 = 0;
            while pause_state {
                println!("录制被暂停");
                let pause_flag = PAUSE_FLAG.lock().unwrap();
                if *pause_flag == false {
                    pause_end_time = START_TIME.lock().unwrap().elapsed().as_millis();
                    let mut mouse_flag = MOUSE_THREAD_FLAG.lock().unwrap();
                    *mouse_flag = false;
                    println!("暂停结束mouse_flag{}", mouse_flag);
                    pause_state = false;
                    println!("暂停结束");
                    let mut pause_time = PAUSE_TIME.lock().unwrap();
                    *pause_time = *pause_time + pause_end_time - pause_start_time;
                } else if m_flag == false {
                    let mut mouse_flag = MOUSE_THREAD_FLAG.lock().unwrap();
                    *mouse_flag = true;
                    pause_start_time = START_TIME.lock().unwrap().elapsed().as_millis();
                    println!("暂停1s");
                    println!("暂停开始mouse_flag{}", mouse_flag);
                    m_flag = true;
                }
                thread::sleep(Duration::from_secs(1));
            }
            sleep(Duration::from_millis(300));
            // let keys: Vec<Keycode> = device_state.get_keys();
            // if keys.len() != 0 && keys[0] != Keycode::F1 {
            //     let pause_time = PAUSE_TIME.lock().unwrap();
            //     let duration_key = START_TIME.lock().unwrap().elapsed().as_millis() - *pause_time;
            //     println!("{}ms,捕捉到键盘输入{:?}", duration_key, keys[0]);
            //     let output = format!("{},key,{:?}\n", duration_key, keys[0]);
            //     // window
            //     // .emit(
            //     //     "press-listen-keyboard",
            //     //     Payload {

            //     //         message: "Tauri is awesome!".into(),
            //     //     },
            //     // )
            //     // .unwrap();
            //     if keys[0] != Keycode::F1
            //         && keys[0] != Keycode::F2
            //         && keys[0] != Keycode::F3
            //         && keys[0] != Keycode::F4
            //         && keys[0] != Keycode::F5
            //         && keys[0] != Keycode::F6
            //         && keys[0] != Keycode::F7
            //         && keys[0] != Keycode::F8
            //     {
            //         save_file.write_all(output.as_bytes()).unwrap();
            //     }
            //     sleep(Duration::from_millis(300));
            //     continue;
            // } else if keys.len() == 0 {
            //     continue;
            // }




            // let mut mouse_flag = MOUSE_THREAD_FLAG.lock().unwrap();

            // *mouse_flag = true;

            // if status == "started" {
            //     println!(
            //         "\n经过了：{}毫秒",
            //         START_TIME.lock().unwrap().elapsed().as_millis()
            //     );
            //     return;
            // }
        }
    }

    //录制截图
    #[tauri::command]
    pub async fn start_screen(mut click_button: bool, window: Window) {
        loop {
            let mut now_dir = String::new();
            let device_state = DeviceState::new();
            let keys: Vec<Keycode> = device_state.get_keys();

            let global_now_dir = NOW_DIR.lock().unwrap();
            now_dir = global_now_dir.clone();

            // 使用 Arc<AtomicBool> 共享键盘监听标志
            let should_stop_keyboard = SHOULD_STOP_KEYBOARD.clone();

            if should_stop_keyboard.load(Ordering::Relaxed) {
                println!("键盘监听线程结束");
                break;
            }

            if !click_button {
                continue;
            }

            if click_button == true {
                println!("传进来第一次的clickButton是{}", click_button);
                click_button = false;
                sleep(Duration::from_millis(300));
                println!("进入截图功能");
                println!("请等待几秒，正在提取图片文字...");
                {
                    let mut screen_flag = SCREEN_SHOT_FLAG.lock().unwrap();
                    *screen_flag = true;
                }
                let output = Command::new("C:/Users/trookie/venv/Scripts/textshot")
                    .arg("eng+chi_sim")
                    .output()
                    .expect("无法启动 textshot 命令");

                if output.status.success() {
                    let textshot_time = *TEXTSHOT_ACTION_TIME.lock().unwrap();

                    let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(format!(
                            "../Automated-testing/result/{}/textshot_{}.txt",
                            now_dir,
                            textshot_time.to_string()
                        ))
                        .expect("无法打开文件");

                    let output_confirm = String::from_utf8_lossy(&output.stdout);
                    if output_confirm
                        .contains("ERROR: Unable to read text from image, did not copy")
                    {
                    } else {
                        let mut output_without_info: ClipboardContext =
                            ClipboardProvider::new().unwrap();
                        let mut clipboard_content = output_without_info.get_contents().unwrap();
                        clipboard_content.push('\n');
                        file.write_all(clipboard_content.as_bytes())
                            .expect("提取文字命令写入文件失败");
                    }
                    window
                        .emit(
                            "event-name",
                            Payload {
                                message: "Tauri is awesome!".into(),
                            },
                        )
                        .unwrap();
                    println!("提取文字执行成功！请继续操作。");
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    println!("提取文字命令执行失败: {}", error);
                }

                continue;
            } else if keys.len() != 0 && keys[0] != Keycode::F1 {
                sleep(Duration::from_millis(175));
                continue;
            } else if keys.len() == 0 {
                continue;
            }
        }
    }

    //录制完成前端按钮触发函数
    #[tauri::command]
    pub fn record_end() {
        let mut stop_flag = SHOULD_STOP.lock().unwrap();
        *stop_flag = true;
    }

    //暂停录制按钮触发
    #[tauri::command]
    pub fn pause_record() {
        let mut pause_flag = PAUSE_FLAG.lock().unwrap();
        *pause_flag = true;
    }
    //恢复录制按钮触发
    #[tauri::command]
    pub fn resume_record() {
        let mut pause_flag = PAUSE_FLAG.lock().unwrap();
        *pause_flag = false;
    }
}
