use crate::cli::PngCommand;
use crate::png::Png;
use crate::png_error::PngError;

struct PngHandler {
    command: PngCommand,
    my_png: Png,
}

impl Png {
    pub fn new() {
        todo!()
    }

}
type Result<T> = std::result::Result<T, PngError>;

impl TryFrom<PngCommand> for PngHandler {
    type Error = PngError;

    fn try_from(value: PngCommand) -> Result<Self> {
        todo!()
    }
}