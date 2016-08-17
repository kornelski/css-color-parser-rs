// (c) Dean McNamee <dean@gmail.com>, 2012.
// (c) Rust port by Katkov Oleksandr <alexx.katkoff@gmail.com>, 2016.
//
// https://github.com/deanm/css-color-parser-js
// https://github.com/7thSigil/css-color-parser-rs
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use std::str;
use std::error;
use std::fmt;
use std::num;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
	pub r : u8,
	pub g : u8,
	pub b : u8,
	pub a : f32
}

struct NamedColor {
	name : &'static str,
	color: Color
}

// http://www.w3.org/TR/css3-color/
//TODO: finish & recheck this list
static NAMED_COLORS: [NamedColor; 18] = [
	// "transparent": [0,0,0,0], 
	NamedColor { name: "transparent", color: Color { r: 0, g: 0, b: 0, a: 0.0 } },
	// "aliceblue": [240,248,255,1],
	NamedColor { name: "aliceblue", color: Color { r: 240, g: 248, b: 255, a: 1.0 } },
	// "antiquewhite": [250,235,215,1],
	NamedColor { name: "antiquewhite", color: Color { r: 250, g: 235, b: 215, a: 1.0 } },
	// "aqua": [0,255,255,1],
	NamedColor { name: "aqua", color: Color { r: 0, g: 255, b: 255, a: 1.0 } },
	// "aquamarine": [127,255,212,1], 
	NamedColor { name: "aquamarine", color: Color { r: 127, g: 255, b: 212, a: 1.0 } },
	// "azure": [240,255,255,1],
	NamedColor { name: "azure", color: Color { r: 240, g: 255, b: 255, a: 1.0 } },
	// "beige": [245,245,220,1], 
	NamedColor { name: "beige", color: Color { r: 245, g: 245, b: 220, a: 1.0 } },
	// "bisque": [255,228,196,1],
	NamedColor { name: "bisque", color: Color { r: 255, g: 228, b: 196, a: 1.0 } },
	// "black": [0,0,0,1], 
	NamedColor { name: "black", color: Color { r: 0, g: 0, b: 0, a: 1.0 } },
	// "blanchedalmond": [255,235,205,1],
	NamedColor { name: "blanchedalmond", color: Color { r: 255, g: 235, b: 205, a: 1.0 } },
	// "blue": [0,0,255,1],
	NamedColor { name: "blue", color: Color { r: 0, g: 0, b: 255, a: 1.0 } },
	// "blueviolet": [138,43,226,1],
	NamedColor { name: "blueviolet", color: Color { r: 138, g: 43, b: 226, a: 1.0 } },
	// "brown": [165,42,42,1], 
	NamedColor { name: "brown", color: Color { r: 165, g: 42, b: 42, a: 1.0 } },
	// "burlywood": [222,184,135,1],
	NamedColor { name: "burlywood", color: Color { r: 222, g: 184, b: 135, a: 1.0 } },
	// "cadetblue": [95,158,160,1], 
	NamedColor { name: "cadetblue", color: Color { r: 95, g: 158, b: 160, a: 1.0 } },
	// "chartreuse": [127,255,0,1],
	NamedColor { name: "chartreuse", color: Color { r: 127, g: 255, b: 0, a: 1.0 } },
	// "chocolate": [210,105,30,1], 
	NamedColor { name: "chocolate", color: Color { r: 210, g: 105, b: 30, a: 1.0 } },
	// "coral": [255,127,80,1],
	NamedColor { name: "coral", color: Color { r: 255, g: 127, b: 80, a: 1.0 } },  
	// "cornflowerblue": [100,149,237,1], 
	// "cornsilk": [255,248,220,1],
	// "crimson": [220,20,60,1], 
	// "cyan": [0,255,255,1],
	// "darkblue": [0,0,139,1], 
	// "darkcyan": [0,139,139,1],
	// "darkgoldenrod": [184,134,11,1], 
	// "darkgray": [169,169,169,1],
	// "darkgreen": [0,100,0,1], 
	// "darkgrey": [169,169,169,1],
	// "darkkhaki": [189,183,107,1], 
	// "darkmagenta": [139,0,139,1],
	// "darkolivegreen": [85,107,47,1], 
	// "darkorange": [255,140,0,1],
	// "darkorchid": [153,50,204,1], 
	// "darkred": [139,0,0,1],
	// "darksalmon": [233,150,122,1], 
	// "darkseagreen": [143,188,143,1],
	// "darkslateblue": [72,61,139,1], 
	// "darkslategray": [47,79,79,1],
	// "darkslategrey": [47,79,79,1], 
	// "darkturquoise": [0,206,209,1],
	// "darkviolet": [148,0,211,1], 
	// "deeppink": [255,20,147,1],
	// "deepskyblue": [0,191,255,1], 
	// "dimgray": [105,105,105,1],
	// "dimgrey": [105,105,105,1], 
	// "dodgerblue": [30,144,255,1],
	// "firebrick": [178,34,34,1], 
	// "floralwhite": [255,250,240,1],
	// "forestgreen": [34,139,34,1], 
	// "fuchsia": [255,0,255,1],
	// "gainsboro": [220,220,220,1], 
	// "ghostwhite": [248,248,255,1],
	// "gold": [255,215,0,1], 
	// "goldenrod": [218,165,32,1],
	// "gray": [128,128,128,1], 
	// "green": [0,128,0,1],
	// "greenyellow": [173,255,47,1], 
	// "grey": [128,128,128,1],
	// "honeydew": [240,255,240,1], 
	// "hotpink": [255,105,180,1],
	// "indianred": [205,92,92,1], 
	// "indigo": [75,0,130,1],
	// "ivory": [255,255,240,1], 
	// "khaki": [240,230,140,1],
	// "lavender": [230,230,250,1], 
	// "lavenderblush": [255,240,245,1],
	// "lawngreen": [124,252,0,1], 
	// "lemonchiffon": [255,250,205,1],
	// "lightblue": [173,216,230,1], 
	// "lightcoral": [240,128,128,1],
	// "lightcyan": [224,255,255,1], 
	// "lightgoldenrodyellow": [250,250,210,1],
	// "lightgray": [211,211,211,1], 
	// "lightgreen": [144,238,144,1],
	// "lightgrey": [211,211,211,1], 
	// "lightpink": [255,182,193,1],
	// "lightsalmon": [255,160,122,1], 
	// "lightseagreen": [32,178,170,1],
	// "lightskyblue": [135,206,250,1], 
	// "lightslategray": [119,136,153,1],
	// "lightslategrey": [119,136,153,1], 
	// "lightsteelblue": [176,196,222,1],
	// "lightyellow": [255,255,224,1], 
	// "lime": [0,255,0,1],
	// "limegreen": [50,205,50,1], 
	// "linen": [250,240,230,1],
	// "magenta": [255,0,255,1], 
	// "maroon": [128,0,0,1],
	// "mediumaquamarine": [102,205,170,1], 
	// "mediumblue": [0,0,205,1],
	// "mediumorchid": [186,85,211,1], 
	// "mediumpurple": [147,112,219,1],
	// "mediumseagreen": [60,179,113,1], 
	// "mediumslateblue": [123,104,238,1],
	// "mediumspringgreen": [0,250,154,1], 
	// "mediumturquoise": [72,209,204,1],
	// "mediumvioletred": [199,21,133,1], 
	// "midnightblue": [25,25,112,1],
	// "mintcream": [245,255,250,1], 
	// "mistyrose": [255,228,225,1],
	// "moccasin": [255,228,181,1], 
	// "navajowhite": [255,222,173,1],
	// "navy": [0,0,128,1], 
	// "oldlace": [253,245,230,1],
	// "olive": [128,128,0,1], 
	// "olivedrab": [107,142,35,1],
	// "orange": [255,165,0,1], 
	// "orangered": [255,69,0,1],
	// "orchid": [218,112,214,1], 
	// "palegoldenrod": [238,232,170,1],
	// "palegreen": [152,251,152,1], 
	// "paleturquoise": [175,238,238,1],
	// "palevioletred": [219,112,147,1], 
	// "papayawhip": [255,239,213,1],
	// "peachpuff": [255,218,185,1], 
	// "peru": [205,133,63,1],
	// "pink": [255,192,203,1], 
	// "plum": [221,160,221,1],
	// "powderblue": [176,224,230,1], 
	// "purple": [128,0,128,1],
	// "red": [255,0,0,1], 
	// "rosybrown": [188,143,143,1],
	// "royalblue": [65,105,225,1], 
	// "saddlebrown": [139,69,19,1],
	// "salmon": [250,128,114,1], 
	// "sandybrown": [244,164,96,1],
	// "seagreen": [46,139,87,1], 
	// "seashell": [255,245,238,1],
	// "sienna": [160,82,45,1], 
	// "silver": [192,192,192,1],
	// "skyblue": [135,206,235,1], 
	// "slateblue": [106,90,205,1],
	// "slategray": [112,128,144,1], 
	// "slategrey": [112,128,144,1],
	// "snow": [255,250,250,1], 
	// "springgreen": [0,255,127,1],
	// "steelblue": [70,130,180,1], 
	// "tan": [210,180,140,1],
	// "teal": [0,128,128,1], 
	// "thistle": [216,191,216,1],
	// "tomato": [255,99,71,1], 
	// "turquoise": [64,224,208,1],
	// "violet": [238,130,238,1], 
	// "wheat": [245,222,179,1],
	// "white": [255,255,255,1], 
	// "whitesmoke": [245,245,245,1],
	// "yellow": [255,255,0,1], 
	// "yellowgreen": [154,205,50,1]}
];

#[derive(Debug)]
pub struct ColorParseError;

impl From<num::ParseIntError> for ColorParseError {
	fn from(err: num::ParseIntError) -> ColorParseError {
		return ColorParseError;
	}
}

impl From<num::ParseFloatError> for ColorParseError {
	fn from(err: num::ParseFloatError) -> ColorParseError {
		return ColorParseError;
	}
}

impl error::Error for ColorParseError {
    fn description(&self) -> &str {
        panic!("NotImplemented!");
    }

    fn cause(&self) -> Option<&error::Error> {
    	panic!("NotImplemented!");
    }
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	panic!("NotImplemented!");
    }
}


//TODO(7thSigil): check if platform byte order affects parsing
//TODO(7thSigil): maybe rewrite error handling into something more informative?
impl str::FromStr for Color {

	type Err = ColorParseError;

	fn from_str(s: &str) -> Result<Self, ColorParseError>
	{
		let mut string = s.to_string();

		// Remove all whitespace, not compliant, but should just be more accepting.
		string = string.replace(" ", "")
					   .to_lowercase();

		if string.is_empty()
		{
			return Err(ColorParseError);
		}

		let mut iterator = NAMED_COLORS.iter();

		// Color keywords (and transparent) lookup.
		while let Some(named_color) = iterator.next() {
			if named_color.name == string { 
				return Ok ( named_color.color ); 
			}
		}

		if string.starts_with("#") {
			let string_char_count = string.chars().count();

			if string_char_count == 4 {
				let (_, value_string) = string.split_at(1);

				let iv = try!(u64::from_str_radix(value_string, 16));

				//unlike original js code, NaN is impossible ()
				if !(iv <= 0xfff) {
					return Err(ColorParseError); 
				}

				return Ok (Color { r: (((iv & 0xf00) >> 4) | ((iv & 0xf00) >> 8)) as u8,
								   g: ((iv & 0xf0) | ((iv & 0xf0) >> 4)) as u8,
								   b: ((iv & 0xf) | ((iv & 0xf) << 4)) as u8,
								   a: 1.0 
								});
			} 
			else if string_char_count == 7 {
				let (_, value_string) = string.split_at(1);

				let iv = try!(u64::from_str_radix(value_string, 16));

				//(7thSigil) unlike original js code, NaN is impossible
				if !(iv <= 0xffffff) {
					return Err(ColorParseError); 
				}

				return Ok (Color { r: ((iv & 0xff0000) >> 16) as u8,
								   g: ((iv & 0xff00) >> 8) as u8,
								   b: (iv & 0xff) as u8,
								   a: 1.0 
								});
			}

			return Err(ColorParseError);
		}

		let op = try!(string.find("(").ok_or(ColorParseError));
		let ep = try!(string.find(")").ok_or(ColorParseError));

		//(7thSigil) validating format
		//')' bracket should be at the end
		//and always after the opening bracket
		if (ep + 1) != string.len() || ep < op {
			return Err(ColorParseError);
		}

		//(7thSigil) extracting format
		let (fmt, right_string_half) = string.split_at(op);

		//(7thSigil) validating format
		if fmt.is_empty() {
			return Err(ColorParseError);
		}

		//removing brackets
		let mut filtered_right_string_half = right_string_half.to_string();

		//removing brackets
		filtered_right_string_half.remove(0);
		filtered_right_string_half.pop();

		let params: Vec<&str> = filtered_right_string_half.split(",").collect();

		//(7thSigil) validating format
		if params.len() < 3 || params.len() > 4 {
			return Err(ColorParseError);
		} 

		if fmt == "rgba"
		{
			return parse_rgba(params);
		} 
		else if fmt == "rgb" {
			return parse_rgb(params);
		}
		else if fmt == "hsla" {
			return parse_hsla(params);
		} 
		else if fmt == "hsl" {
			return parse_hsl(params);
		}

		return Err(ColorParseError);
	}
}

fn parse_rgba(mut rgba: Vec<&str>) -> Result<Color, ColorParseError> {

	if rgba.len() != 4 {
		return Err(ColorParseError);
	}

	let a_str = try!(rgba.pop().ok_or(ColorParseError));

	let a = try!(parse_css_float(a_str));

	let mut rgb_color = try!(parse_rgb(rgba));

	rgb_color = Color { a: a, .. rgb_color };

	return Ok(rgb_color);
}

fn parse_rgb(mut rgb: Vec<&str>) -> Result<Color, ColorParseError> {

	if rgb.len() != 3 {
		return Err(ColorParseError);
	}

	let b_str = try!(rgb.pop().ok_or(ColorParseError));
	let g_str = try!(rgb.pop().ok_or(ColorParseError));
	let r_str = try!(rgb.pop().ok_or(ColorParseError));

	let r = try!(parse_css_int(r_str));
	let g = try!(parse_css_int(g_str));
	let b = try!(parse_css_int(b_str));

	return Ok(Color { r: r, g: g, b: b, a: 1.0 })
}

fn parse_hsla (mut hsla: Vec<&str>) -> Result<Color, ColorParseError> {

	if hsla.len() != 4 {
		return Err(ColorParseError);
	}

	let a_str = try!(hsla.pop().ok_or(ColorParseError));

	let a = try!(parse_css_float(a_str));

	//(7thSigil) Parsed from hsl to rgb representation
	let mut rgb_color : Color = try!(parse_hsl(hsla));

	rgb_color = Color { a: a, .. rgb_color };

	return Ok(rgb_color);
}

fn parse_hsl (mut hsl: Vec<&str>) -> Result<Color, ColorParseError> {

	if hsl.len() != 3 {
		return Err(ColorParseError);
	}

	let l_str = try!(hsl.pop().ok_or(ColorParseError));
	let s_str = try!(hsl.pop().ok_or(ColorParseError));
	let h_str = try!(hsl.pop().ok_or(ColorParseError));

	let mut h = try!(f32::from_str(h_str));

	// 0 .. 1
	h = (((h % 360.0) + 360.0) % 360.0) / 360.0; 

	// NOTE(deanm): According to the CSS spec s/l should only be
	// percentages, but we don't bother and let float or percentage.

	let s = try!(parse_css_float(s_str));
	let l = try!(parse_css_float(l_str));

	let m2: f32;

	if l <= 0.5 {
		m2 = l * (s + 1.0)
	} else {
		m2= l + s - l * s;
	}

	let m1 = l * 2.0 - m2;

	let r = clamp_css_byte_from_float(css_hue_to_rgb(m1, m2, h+1.0/3.0) * 255.0);
	let g = clamp_css_byte_from_float(css_hue_to_rgb(m1, m2, h) * 255.0);
	let b = clamp_css_byte_from_float(css_hue_to_rgb(m1, m2, h-1.0/3.0) * 255.0);

	return Ok(Color { r: r, g: g, b: b, a: 1.0 });
}

//float or percentage.
fn parse_css_float(fv_str: &str) -> Result<f32, num::ParseFloatError> {

	let fv: f32; 

	if fv_str.ends_with("%") {
		let mut percentage_string = fv_str.to_string();
		percentage_string.pop();
		fv = try!(f32::from_str(&percentage_string));
		return Ok(clamp_css_float(fv / 100.0));
	}

	fv = try!(f32::from_str(fv_str));
	return Ok(clamp_css_float(fv));
}

// int or percentage.
fn parse_css_int(iv_or_percentage_str: &str) -> Result<u8, ColorParseError> {
	if iv_or_percentage_str.ends_with("%") {

		let mut percentage_string = iv_or_percentage_str.to_string();
		percentage_string.pop();
		let fv = try!(f32::from_str(&percentage_string));
		// Seems to be what Chrome does (round vs truncation).
		return Ok(clamp_css_byte_from_float(fv / 100.0 * 255.0));
	}

	let iv = try!(u32::from_str(iv_or_percentage_str));

	return Ok(clamp_css_byte(iv));
}

// Clamp to float 0.0 .. 1.0.
fn clamp_css_float(fv: f32) -> f32 {  
	//return fv < 0 ? 0 : fv > 1 ? 1 : fv;
	if fv < 0.0 {
		0.0 
	} else if fv > 1.0 {
		1.0
	}
	else {
	    fv
	}
}

fn clamp_css_byte_from_float(mut fv: f32) -> u8 {  // Clamp to integer 0 .. 255.
	// Seems to be what Chrome does (vs truncation).
  	fv = fv.round();  

  	// return iv < 0 ? 0 : iv > 255 ? 255 : iv;
	if fv < 0.0 {
		0 
	} else if fv > 255.0 {
		255
	}
	else {
	    fv as u8
	}
}

fn clamp_css_byte(iv: u32) -> u8 {  // Clamp to integer 0 .. 255.
  	// return iv < 0 ? 0 : iv > 255 ? 255 : iv;
  	if iv > 255 {
		255
	}
	else {
	    iv as u8
	}
}

fn css_hue_to_rgb(m1: f32, m2: f32, mut h: f32) -> f32 {
	if h < 0.0 {
		h+=1.0;
	} 
	else if h > 1.0 {
		h-= 1.0;
	}

	if h * 6.0 < 1.0 {
		return m1 + (m2 - m1) * h * 6.0;
	}
	if h * 2.0 < 1.0 {
		return m2;
	}
	if h * 3.0 < 2.0 {
		return m1 + (m2 - m1) * (2.0/3.0 - h) * 6.0;
	}

	return m1;
}