mod args;
use args::Args;
use image::{imageops::FilterType::Triangle, DynamicImage, GenericImageView, ImageFormat, ImageReader};


#[derive(Debug)]
enum ImageErrors {
    DifferentImageFormat,
    BufferOverflow
}


struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = (width * height * 4) as usize;
        let buffer = Vec::with_capacity(buffer_capacity);
        FloatingImage { 
            width: width, 
            height: height, 
            data: buffer, 
            name: name 
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageErrors> {
        print!("CC {}-{}", data.len(), self.data.capacity());
        if data.len() > self.data.capacity() {
            return Err(ImageErrors::BufferOverflow);
        }
        self.data = data;

        Ok(())
    }
}


fn main() -> Result<(), ImageErrors> {
    let args: Args = Args::new();

    let (image_1, image_format_1) = image_from_path(args.image_1);
    let (image_2, image_format_2) = image_from_path(args.image_2);

    if image_format_1 != image_format_2 {
        return Err(ImageErrors::DifferentImageFormat)
    }

    let (image_1, image_2) = standardize_image(image_1, image_2);

    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

    let combined = combine_images(image_1, image_2);

    output.set_data(combined)?;

    print!("ERR {}-{}\n", output.width, output.height);
    print!("ERR2 {:?}\n", output.data.len());


    image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8, image_format_1).unwrap();

    Ok(())
}


fn image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader = ImageReader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();

    (image, image_format)
}

fn get_smallest_dimension(dimension1: (u32, u32), dimension2: (u32, u32)) -> (u32, u32) {
    let pixel1 = dimension1.0 * dimension1.1;
    let pixel2 = dimension2.0 * dimension2.1;

    if pixel1 < pixel2 {
        return dimension1;
    } else {
        return dimension2
    }
}


fn standardize_image(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimension(image_1.dimensions(), image_2.dimensions());

    print!("Width: {} and Height {}\n", width, height);

    if image_1.dimensions() == (width, height) {
        (image_1, image_2.resize_exact(width, height, Triangle))
    } else {
        (image_1.resize_exact(width, height, Triangle), image_2)
    }
}

fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vector_1 = image_1.to_rgba8().to_vec();
    let vector_2 = image_2.to_rgba8().to_vec();

    swap_pixels(vector_1, vector_2)
}

fn swap_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];

    let mut i = 0;
    while i < vec_1.len() {
        if  i%8 == 0 {
            combined_data.splice(i..=i+3, replace_rgba(&vec_1, i, i+3));
        } else {
            combined_data.splice(i..=i+3, replace_rgba(&vec_2, i, i+3));
        }
        i += 4;
    }

    combined_data
}

fn replace_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();

    for i in start..=end {
        let pix = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bound"),
        };
        rgba.push(pix);
    }

    rgba
}


