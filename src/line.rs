use super::char_buffer::CharBuffer;


pub fn draw_line(char: char, buf: &mut CharBuffer, mut start_coords: (usize, usize), mut end_coords: (usize, usize)) {

    if start_coords.0 > end_coords.0 { //Eliminates left-pointing lines
        let b = start_coords;
        start_coords = end_coords;
        end_coords = b;
    }

    let slope = (end_coords.1 as f32 - start_coords.1 as f32)  / (end_coords.0 as f32 - start_coords.0 as f32);

    if slope < -1.0 {
        //Down vertical
        draw_vertical(char, buf, start_coords, end_coords, false);
    } else if slope > 1.0 {
        //up vertical
        draw_vertical(char, buf, start_coords, end_coords, true);
    } else {
        //right horizontal
        draw_horizontal(char, buf, start_coords, end_coords);
    }

}


fn draw_vertical(char: char, buf: &mut CharBuffer, start_coords: (usize, usize), end_coords: (usize, usize), is_up: bool) {
    
    let inv_slope = (end_coords.0 as f32 - start_coords.0 as f32) / (end_coords.1 as f32 - start_coords.1 as f32);

    if is_up {
        let equation = |y: usize| (inv_slope * (y - start_coords.1) as f32 + start_coords.0 as f32) as usize;
        for y in start_coords.1..=end_coords.1 {
            buf.set_char(equation(y), y, char);
        }
    } else {
        let equation = |y: usize| (inv_slope * (y - end_coords.1) as f32 + end_coords.0 as f32) as usize;
        for y in end_coords.1..=start_coords.1 {
            buf.set_char(equation(y), y, char);
        }
    }

}


fn draw_horizontal(char: char, buf: &mut CharBuffer, start_coords: (usize, usize), end_coords: (usize, usize)) {
    
    let slope = (end_coords.1 as f32 - start_coords.1 as f32)  / (end_coords.0 as f32 - start_coords.0 as f32);

    let equation = |x: usize| (slope * (x - start_coords.0) as f32 + start_coords.1 as f32) as usize;

    for x in start_coords.0..=end_coords.0 {
        buf.set_char(x, equation(x), char);
    }
}

