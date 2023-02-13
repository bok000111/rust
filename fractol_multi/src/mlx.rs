mod mlx;

#[repr(C)]
pub struct Data {
    mlx_ptr: *mut std::ffi::c_void,
    win_ptr: *mut std::ffi::c_void,
    img: *mut std::ffi::c_void,
    addr: *mut i32,
    bits_per_pixel: i32,
    size_line: i32,
    endian: i32,
    mouse_x: i64,
    mouse_y: i64,
    calc: CalcData,
}

struct CalcData {
    width: i64,
    height: i64,
    shift_x: i64,
    shift_y: i64,
    scale: f64,
    iter_max: i32,
    color: i32,
}

impl CalcData {
    fn mandelbrot(&self, c_re: f64, c_im: f64) -> i32 {
        let mut iteration = 0;
        let mut z_re = 0.0;
        let mut z_im = 0.0;
        while z_re * z_re + z_im * z_im < 4.0 && iteration < self.iter_max {
            (z_re, z_im) = (z_re * z_re - z_im * z_im + c_re, 2.0 * z_re * z_im + c_im);
            iteration += 1;
        }
        iteration
    }

    fn f64tocolor(&self, iter: i32) -> i32 {
        if iter == self.iter_max {
            0
        } else {
            ((iter) as f64 / self.iter_max as f64 * self.color as f64) as i32
        }
    }
}

impl Data {
    pub fn new(name: &str, screen_size: i32, iter_max: i32, color: i32) -> Box<Data> {
        unsafe {
            let mlx_ptr = mlx::mlx_init();
            let win_ptr = mlx::mlx_new_window(mlx_ptr, screen_size, screen_size, name.to_owned().as_mut_ptr() as *mut i8);
            let img = mlx::mlx_new_image(mlx_ptr, screen_size, screen_size);
            let mut data = Box::new(Data {
                mlx_ptr,
                win_ptr,
                img,
                addr: std::ptr::null_mut(),
                calc: CalcData {
                    width: screen_size as i64,
                    height: screen_size as i64,
                    shift_x: 0,
                    shift_y: 0,
                    scale: 0.005,
                    iter_max,
                    color,
                },
                bits_per_pixel: 0,
                size_line: 0,
                endian: 0,
                mouse_x: 0,
                mouse_y: 0,
            });
            data.addr = mlx::mlx_get_data_addr(
                data.img, 
                &mut data.bits_per_pixel as *mut i32, 
                &mut data.size_line as *mut i32, 
                &mut data.endian as *mut i32
            ) as *mut i32;
            data
        }
    }

    pub fn mlx_loop(&self) {
        unsafe {
            self.refresh();
            mlx::mlx_loop(self.mlx_ptr);
        }
    }

    pub fn mlx_hook(&mut self, x_event: i32, funct: extern "C" fn() -> i32) -> i32 {
        unsafe {
            mlx::mlx_hook(
                self.win_ptr,
                x_event,
                0,
                Some(funct),
                self as *mut Self as *mut std::ffi::c_void
            )
        }
    }

    pub fn shift(&mut self, mode: bool, dir: bool) {
        if mode {
            if dir {
                self.calc.shift_x += self.calc.width / 10;
            } else {
                self.calc.shift_x -= self.calc.width / 10;
            }
        } else {
            if dir {
                self.calc.shift_y -= self.calc.height / 10;
            } else {
                self.calc.shift_y += self.calc.height / 10;
            }
        }
    }

    pub fn mouse(&mut self, x: i32, y: i32) {
        self.mouse_x = x as i64 - self.calc.width / 2;
        self.mouse_y = self.calc.height / 2 - y as i64;
    }

    pub fn scale(&mut self, mode: bool) {
        if mode {
            self.calc.scale /= 1.1;
            self.calc.shift_x = ((self.calc.shift_x as f64 + self.mouse_x as f64 * 0.1) * 1.1) as i64;
            self.calc.shift_y = ((self.calc.shift_y as f64 + self.mouse_y as f64 * 0.1) * 1.1) as i64;
        } else {
            self.calc.scale *= 1.1;
            self.calc.shift_x = ((self.calc.shift_x as f64 - self.mouse_x as f64 * 0.1) / 1.1) as i64;
            self.calc.shift_y = ((self.calc.shift_y as f64 - self.mouse_y as f64 * 0.1) / 1.1) as i64;
        }
    }

    pub fn refresh(&self) {
        if let Ok(available_thread) = std::thread::available_parallelism() {
            let reciever = self.multi_thread(available_thread.get() as i64 * 2 - 4);
            for (x, y, color) in reciever {
                unsafe {
                    *self.addr.offset((x + (self.calc.height as i32 - (y + 1)) * 
                    self.size_line / std::mem::size_of::<i32>() as i32) as isize) = self.calc.f64tocolor(color);
                }
            }
        } else {
            self.single_thread();
        }
        unsafe {
            mlx::mlx_put_image_to_window(
                self.mlx_ptr,
                self.win_ptr,
                self.img,
                0,
                0
            );
        }
    }

    fn multi_thread<'s>(&self, threads: i64) -> std::sync::mpsc::Receiver<(i32, i32, i32)> {
        let mut remain = self.calc.width;
        let block_size = remain / threads;
        let (sender, receiver) = std::sync::mpsc::channel();
        while remain != 0 {
            let start;
            let sender_thread;
            if remain < block_size {
                start = 0;
                sender_thread = sender.to_owned();
            } else {
                start = remain - block_size;
                sender_thread = sender.clone();
            }
            let calc = CalcData {
                width: self.calc.width,
                height: self.calc.height,
                shift_x: self.calc.shift_x,
                shift_y: self.calc.shift_y,
                scale: self.calc.scale,
                iter_max: self.calc.iter_max,
                color: self.calc.color,
            };
            std::thread::spawn(move || {
                for x in start..remain {
                    for y in 0..calc.height {
                        sender_thread.send((
                            x as i32,
                            y as i32,
                            calc.mandelbrot(
                                (x + calc.shift_x - calc.width / 2) as f64 * calc.scale,
                                (y + calc.shift_y - calc.height / 2) as f64 * calc.scale
                            )
                        )).unwrap();
                    }
                }
            });
            if remain < block_size {
                remain = 0;
            } else {
                remain -= block_size;
            }
        }
        receiver
    }

    fn single_thread(&self) {
        for x in 0..self.calc.width {
            for y in 0..self.calc.height {
                unsafe {
                    *self.addr.offset(
                        (x + (self.calc.height - (y + 1)) * self.size_line as i64 / std::mem::size_of::<i32>() as i64) as isize
                    ) = self.calc.f64tocolor(self.calc.mandelbrot(
                        (x + self.calc.shift_x - self.calc.width / 2) as f64 * self.calc.scale,
                        (y + self.calc.shift_y - self.calc.height / 2) as f64 * self.calc.scale
                    ));
                }
            }
        }
    }
}