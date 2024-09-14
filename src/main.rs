mod ozuqa;
mod hisobot;
mod umumiy;

use num_format::{Locale, ToFormattedString};
use ozuqa::{Kunjara, Kepak};
use hisobot::Hisoblash;

fn main() {
    umumiy::salomlashuv();

    let umumiy_data = ozuqa::UmumiyData {
        hayvon_soni: 3,
        nechi_mahal_beriladi: 2,
        kun_soni: 30,
    };
    let kunjara = Kunjara {
        kunjara_narxi: 5200,
        kunjara_kg: 1,
        umumiy_data: &umumiy_data
    };
    let kepak = Kepak {
        kepak_narxi: 2400,
        kepak_kg: 2,
        umumiy_data: &umumiy_data
    };

    let umumiykg = kunjara.klogramm() + kepak.klogramm();
    let umumiy_sum = kunjara.klogramm_narxi() + kepak.klogramm_narxi();

    let kunjara_kg = kunjara.klogramm();
    let kepak_kg = kepak.klogramm();
    let kun = umumiy_data.kun_soni;

    println!("{} kg uchun {} so'm",umumiykg, umumiy_sum.to_formatted_string(&Locale::en));
    println!("{}kg kepak, {}kg kunjara {} kun davomida",kepak_kg, kunjara_kg, kun);
}
