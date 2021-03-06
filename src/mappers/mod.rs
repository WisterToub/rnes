mod nrom;
mod mmc1;
mod uxrom;
mod mmc3;
mod axrom;

use self::{
    nrom::*,
    mmc1::*,
    uxrom::*,
    mmc3::*,
    axrom::*
};

pub trait Mapper {
    fn read(&self, u16) -> u8 {
        unimplemented!();
    }

    fn write(&mut self, u16, u8) {
        unimplemented!();
    }
}

pub fn create_mapper(number: u8, prg_rom: &[u8]) -> Box<dyn Mapper> {
    match number {
        0 => Box::new(Nrom::new(prg_rom)),
        1 => Box::new(Mmc1::new(prg_rom)),
        2 => Box::new(Uxrom::new(prg_rom)),
        4 => Box::new(Mmc3::new(prg_rom)),
        7 => Box::new(Axrom::new(prg_rom)),
        _ => unimplemented!()
    }
}
