use crate::ozuqa::{Kunjara, Kepak};


pub trait Hisoblash {
    fn klogramm(&self) -> u64;
    fn klogramm_narxi(&self) -> u64;
}
impl Hisoblash for Kunjara<'_> {
    fn klogramm(&self) -> u64 {
        self.kunjara_kg * self.umumiy_data.nechi_mahal_beriladi * self.umumiy_data.hayvon_soni * self.umumiy_data.kun_soni
    }
    fn klogramm_narxi(&self) -> u64 {
        self.klogramm() * self.kunjara_narxi
    }

}
impl Hisoblash for Kepak<'_> {
    fn klogramm(&self) -> u64 {
        self.kepak_kg * self.umumiy_data.nechi_mahal_beriladi * self.umumiy_data.hayvon_soni * self.umumiy_data.kun_soni
    }

    fn klogramm_narxi(&self) -> u64 {
        self.klogramm() * self.kepak_narxi
    }

}