use bitmap::Bitmap;
use color::Color;
use rect::Rect;
use triangle::Triangle;
use util;

use std::path::Path;

pub struct Canvas {
    bitmap: Bitmap, 
}

impl Canvas {
    pub fn new(bitmap: Bitmap) -> Canvas {
        Canvas {
            bitmap: bitmap,
        }
    }

    pub fn clear(&mut self, color: &Color) {
        let srcpx = color.to_pixel();

        let w = self.bitmap.width;
        let h = self.bitmap.height;

        for i in 0..w*h {
            self.bitmap.pixels[i] = srcpx;
        }
    }

    pub fn fill_rect(&mut self, rect: &Rect, color: &Color) {
        if !rect.empty() {
            let srcpx = color.to_pixel();

            // Skip transparent fill colors
            let src_a = srcpx.a;
            if src_a == 0 {
                return;
            }

            let (w, h) = (self.bitmap.width, self.bitmap.height);

            // Clip rectangle with canvas
            let mut roi = Rect::make_wh(w as f32, h as f32).round();
            if !roi.intersect(&rect.round()) {
                return;
            }

            let mut i = (roi.top * w as f32 + roi.left) as usize;
            for _ in 0..roi.height() as usize {
                // Draw row
                for _ in 0..roi.width() as usize {
                    self.bitmap.pixels[i] = if src_a == 255 {
                        srcpx
                    } else {
                        util::blend(&srcpx, &self.bitmap.pixels[i])
                    };
                    i += 1;
                }

                // Advance to next row
                i += self.bitmap.width - roi.width() as usize;
            }
        }
    }

    pub fn fill_tri(&mut self, tri: &Triangle, color: &Color) {
        let srcpx = color.to_pixel();

        // Skip transparent fill colors
        let src_a = srcpx.a;
        if src_a == 0 {
            return;
        }

        let (w, h) = (self.bitmap.width, self.bitmap.height);

        // Clip triangle with canvas
        let bounds = tri.bounds();
        let mut roi = Rect::make_wh(w as f32, h as f32).round();
        if !roi.intersect(&bounds) {
            return;
        }

        // Vertices
        let (ax, ay) = (tri.a.x, tri.a.y);
        let (bx, by) = (tri.b.x, tri.b.y);
        let (cx, cy) = (tri.c.x, tri.c.y);

        let denom = ax*by - ay*bx - ax*cy + ay*cx + bx*cy - by*cx;

        let bxcy_bycx = bx*cy - by*cx;
        let axcy_aycx = ax*cy - ay*cx;

        let x0 = roi.left;

        for yi in roi.top as i32 .. roi.bottom as i32 + 1 {
            let y = yi as f32;

            let alpha_numer_start = bxcy_bycx + by*x0 - bx*y - cy*x0 + cx*y;
            let mut alpha_numer   = alpha_numer_start;

            let beta_numer_start = axcy_aycx + ay*x0 - ax*y - cy*x0 + cx*y;
            let mut beta_numer   = beta_numer_start;

            for xi in roi.left as i32 .. roi.right as i32 + 1 {
                alpha_numer += by - cy;
                beta_numer  += ay - cy;

                let alpha =  alpha_numer / denom;
                let beta  = -beta_numer  / denom;

                if alpha >= 0f32 && beta >= 0f32 && alpha + beta <= 1f32 {
                    let i = (xi + yi * w as i32) as usize;

                    self.bitmap.pixels[i] = if src_a == 255 {
                        srcpx
                    } else {
                        util::blend(&srcpx, &self.bitmap.pixels[i])
                    };
                }
            }
        }
    }

    pub fn write(&self, path: &Path) {
        self.bitmap.write(&path);
    }
}
