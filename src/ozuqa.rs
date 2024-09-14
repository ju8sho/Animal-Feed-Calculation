pub struct UmumiyData {
    pub hayvon_soni: u64,
    // pub ozuqa_kg: u64,
    pub nechi_mahal_beriladi: u64,
    pub kun_soni: u64
}

pub struct Kunjara<'a> {
    pub kunjara_narxi: u64,
    pub kunjara_kg: u64,
    pub umumiy_data: &'a UmumiyData
}
pub struct Kepak<'a> {
    pub kepak_narxi: u64,
    pub kepak_kg: u64,
    pub umumiy_data: &'a UmumiyData
}