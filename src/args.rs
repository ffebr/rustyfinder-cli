use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "rastyfinder",
    version,
    about = "Простой HTTP-сервер для локальной раздачи файла с QR-кодом"
)]
pub struct Args {
    #[arg(short = 'f', long = "file", value_name = "PATH")]
    pub path: PathBuf,

    #[arg(short = 'p', long = "port", value_name = "PORT", )]
    pub port: Option<u16>,
}

impl Args {
    pub fn validate(&self) -> std::io::Result<()> {
        if !self.path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Файл не найден: {}", self.path.display()),
            ));
        }
        if !self.path.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Указанный путь не является файлом: {}", self.path.display()),
            ));
        }
        Ok(())
    }
}
