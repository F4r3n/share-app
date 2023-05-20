
use arboard::{Clipboard};
use base64::Engine;

#[path = "./path.rs"]
mod path;


pub fn get_image_clipboard() -> Result<String, anyhow::Error>
{
  let mut ctx = Clipboard::new().unwrap();
  
  match ctx.get_image() {
    Ok(image) => {
        let imgbuf = image::ImageBuffer::from_raw(
            image.width as u32,
            image.height as u32,
            image.bytes.into_owned(),
        )
        .unwrap();
        let imgbuf = image::DynamicImage::ImageRgba8(imgbuf);
        
        let mut w = std::io::Cursor::new(Vec::new());

        imgbuf.write_to(&mut w, image::ImageOutputFormat::Png)?;
        let engine = base64::engine::general_purpose::STANDARD_NO_PAD;
        let res_base64 = engine.encode(&w.into_inner());
        Ok(res_base64)
    }
    Err(e) => Err(e.into())
  }

}