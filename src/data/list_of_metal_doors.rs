use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MetalDoor {
    pub name: &'static str,
    pub description: &'static str,
    pub img_src: &'static str,
}

pub const LIST_OF_METAL_DOORS: &[MetalDoor] = &[
    MetalDoor {
        name: "DRZWI AIR GLASS",
        description: "Seria THERMO PRIME",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/Salzburg.jpeg",
    },
    MetalDoor {
        name: "DRZWI QUADRO",
        description: "Seria THERMO PRIME",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/Bamberg-3.jpeg",
    },
    MetalDoor {
        name: "DRZWI ELITE 3D",
        description: "Seria THERMO PRIME",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/Thermoprime-Trin-winchester.jpeg",
    },
    MetalDoor {
        name: "DRZWI TEMPO",
        description: "Seria THERMO PRIME",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/Hanover-6.jpeg",
    },
    MetalDoor {
        name: "DRZWI AIR GLASS",
        description: "Seria THERMO PREMIUM",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/Graz-1.jpeg",
    },
    MetalDoor {
        name: "DRZWI QUADRO",
        description: "Seria THERMO PREMIUM",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/KRONACH-1-Orzech-alpejski.jpeg",
    },
    MetalDoor {
        name: "DRZWI ELITE 3D",
        description: "Seria THERMO PREMIUM",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/Berno.jpeg",
    },
    MetalDoor {
        name: "DRZWI TEMPO",
        description: "Seria THERMO PREMIUM",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/Milano-8-1.jpeg",
    },
    MetalDoor {
        name: "DRZWI TEMPO",
        description: "Seria OPTIMA",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/Milano-4B-1.jpeg",
    },
    MetalDoor {
        name: "DRZWI CLASSIC",
        description: "Seria OPTIMA",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/Londyn-1.jpeg",
    },
    MetalDoor {
        name: "DRZWI ALTUS",
        description: "Drzwi aluminiowe ALTUS",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/GE5-kolor-antracyt-ciemny-szafir.jpeg",
    },
    MetalDoor {
        name: "DRZWI DUO 60",
        description: "Seria OPTIMA",
        img_src: "https://admal-drzwi.pl/wp-content/uploads/2023/12/ST8-RYGA-P00-mahon.jpeg",
    },
];
