// 类别
pub enum Typo {
    Kinfe,
    Pistol,
    Rifle,
    SubmachineGun,
    Shotgun,
    MachineGun,
    Sticker,
    Gloves,
    Agent,
    Other,
}

// 磨损种类
pub enum WearType {
    NoWare,
    FactoryNew,
    MinimalWare,
    FieldTested,
    WellWorn,
    BattleScarred,
}

// 品质
pub enum Quality {
    ConsumerGrade,
    IndustrialGrade,
    MilSpec,
    Restricted,
    Classified,
    Covert,
    MeleeWeapons,
    ContrabandItems,
}

// 稀有
pub enum Rarity {
    Common,
    Advanced,
    Rare,
    Mythical,
    Legendary,
    Ancient,
    DevastatinglyRare,
    Immortal,
}

pub struct Item {
    id: u32,
    typo: Typo,
    category: String,
    name: String,
    ware_type: WearType,
    quality: Quality,
    rarity: Rarity,
    stat_trak: bool,
}

#[derive(Debug)]
pub struct PriceInfo {
    id: usize,
    item_id: u32,
    date: String,
    price: f32,
}

impl PriceInfo {
    pub fn new(id: usize, item_id: u32, date: String, price: f32) -> Self {
        PriceInfo {
            id,
            item_id,
            date,
            price,
        }
    }
}