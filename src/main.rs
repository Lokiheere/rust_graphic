mod renderer;

use renderer::window::create;
use renderer::graphic::graphic;

fn main ()
{
    env_logger::init();
    create();
    graphic();
}
