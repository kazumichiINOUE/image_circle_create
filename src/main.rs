use minifb::{Key, Window, WindowOptions};
use rand::Rng;

struct Circle {
    cx: usize,
    cy: usize,
    radius: usize,
    hue: usize,
}

impl Circle {
    fn new_circle_random(w: usize, h: usize, max_r: usize) -> Circle {
        let mut rng = rand::thread_rng();
        Circle {
            cx:     rng.gen_range(0..=w),
            cy:     rng.gen_range(0..=h),
            radius: rng.gen_range(1..=max_r),
            hue:    rng.gen_range(0..=360),
        }
    }
}

fn main() {
    let (width, height) = (2000, 1200);

    // 表示する画像をバッファとして作成する
    // 色並びは右の通り ARGB
    let argb: u32 = 255 << 24 | 125 << 16 | 125 << 8 | 125;
    let mut buffer: Vec<u32> = vec![argb; width as usize * height as usize];
    // 円を描く
    for _i in 0..100 {
        let circ = Circle::new_circle_random(width, height, 100);
        draw_circle_fill(&mut buffer, width, height, circ.cx, circ.cy, circ.radius, hsv_to_rgb(circ.hue as f32, 1.0, 1.0));
    }

    // 画像を表示するウィンドウを作成
    let mut window = Window::new(
        "Image with Plots",
        width as usize,
        height as usize,
        WindowOptions::default(),
        )
        .expect("ウィンドウの作成に失敗しました");

    // ウィンドウの表示
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .expect("ウィンドウの更新に失敗しました");
    }

    println!("Bye!");
}

fn draw_circle_fill(buffer: &mut Vec<u32>, width: usize, height: usize, center_x: usize, center_y: usize, radius: usize, argb: u32) {
    // 描画範囲を評価する式
    let is_in_img = |x: i32, y: i32, width: usize, height: usize| -> bool {
        x >= 0 && x < (width as i32) && y >= 0 && y < (height as i32)
    };

    let cx: i32 = center_x as i32;
    let cy: i32 = center_y as i32;
    let r: i32 = radius as i32;

    for x in cx - r..=cx + r {
        for y in cy - r..=cy + r {
            let dx = x - cx;
            let dy = y - cy;
            let distance_squared = dx*dx + dy*dy;
            if distance_squared <= r * r {
                if is_in_img(x, y, width, height) {
                    buffer[(y as usize) * width + (x as usize)] = argb;
                }
            }
        }
    }
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> u32 {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _=> (c, 0.0, x),
    };

    // ARGBの順に32ビットで返す
    255 << 24 |(((r + m) * 255.0) as u32) << 16 | (((g + m) * 255.0) as u32) << 8 | (((b + m) * 255.0) as u32)
}
