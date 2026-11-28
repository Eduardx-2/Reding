mod modules;
use crate::modules::reding::redingWork as red;
use crate::modules::reding::read_config_json as read;

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let d = read::archivo_config()?;
    red::grafos_(Some(d.0),Some(d.1));
    Ok(())
    //red::grafos_();
}
