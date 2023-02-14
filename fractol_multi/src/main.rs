mod mlx;
use crate::mlx::Data;

fn main() {
    let  argv: Vec<String> = std::env::args().collect();
    if argv.len() != 3 {
        println!("usage ./fractol_multi [Screen_size] [Iter_max]");
        std::process::exit(1);
    }
    let mut data = Data::new(
        &argv[0],
        argv[1].parse::<i32>().expect("not a number"),
        argv[2].parse::<i32>().expect("not a number"),
        0x00ffffff,
    );
    unsafe {
        data.mlx_hook(
            2,
            std::mem::transmute::<
                extern "C" fn(i32, *mut std::ffi::c_void) -> i32,
                extern "C" fn() -> i32
            >(key_hook),
        );
        data.mlx_hook(
            6,
            std::mem::transmute::<
                extern "C" fn(i32, i32, *mut std::ffi::c_void) -> i32,
                extern "C" fn() -> i32
            >(mouse_hook),
        );
    }
    data.mlx_loop();
}

extern "C" fn key_hook(keycode: i32, param: *mut std::ffi::c_void) -> i32 {
    unsafe {
        let data = &mut *(param as *mut Data);
        match  keycode {
            53 => std::process::exit(0),
            24 => data.scale(true),
            27 => data.scale(false),
            123 => data.shift(true, false),
            124 => data.shift(true, true),
            125 => data.shift(false, true),
            126 => data.shift(false, false),
            _ => {println!("{}", keycode)},
        }
        data.refresh();
        0
    }
}

extern "C" fn mouse_hook(x: i32, y: i32, param: *mut std::ffi::c_void) -> i32 {
    unsafe {
        let data = &mut *(param as *mut Data);
        data.mouse(x, y);
        0
    }
}