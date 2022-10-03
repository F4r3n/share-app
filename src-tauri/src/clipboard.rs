
use arboard::{Clipboard};
use uuid::Uuid;

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
        let temp_image = path::get_config_dir_temp().unwrap().join(Uuid::new_v4().to_string() + ".png");
    
        path::create_config_temp()?;
        match imgbuf.save(&temp_image) {
            Ok(_) => Ok(String::from(temp_image.to_str().unwrap())),
            Err(e) => Err(e.into())
        }
    }
    Err(e) => Err(e.into())
  }

}