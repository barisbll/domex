use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct StalDoors {
    pub name: &'static str,
    pub big_img_src: &'static str,
    pub small_img_src: &'static str,
}

pub const LIST_OF_STAL_DOORS: &[StalDoors] = &[
    StalDoors {
        name: "01",
        big_img_src: "static/img/door-with-slat/01-1-B.png",
        small_img_src: "static/img/door-with-slat/01-1-S.png",
    },
    StalDoors {
        name: "01",
        big_img_src: "static/img/door-with-slat/01-2-B.png",
        small_img_src: "static/img/door-with-slat/01-2-S.png",
    },
    StalDoors {
        name: "01",
        big_img_src: "static/img/door-with-slat/01-3-B.jpg",
        small_img_src: "static/img/door-with-slat/01-3-S.jpg",
    },
    StalDoors {
        name: "02",
        big_img_src: "static/img/door-with-slat/02-1-B.png",
        small_img_src: "static/img/door-with-slat/02-1-S.png",
    },
    StalDoors {
        name: "02",
        big_img_src: "static/img/door-with-slat/02-2-B.jpg",
        small_img_src: "static/img/door-with-slat/02-2-S.jpg",
    },
    StalDoors {
        name: "02",
        big_img_src: "static/img/door-with-slat/02-3-B.jpg",
        small_img_src: "static/img/door-with-slat/02-3-S.jpg",
    },
    StalDoors {
        name: "02",
        big_img_src: "static/img/door-with-slat/02-4-B.jpg",
        small_img_src: "static/img/door-with-slat/02-4-S.jpg",
    },
    StalDoors {
        name: "02",
        big_img_src: "static/img/door-with-slat/02-5-B.jpg",
        small_img_src: "static/img/door-with-slat/02-5-S.jpg",
    },
    StalDoors {
        name: "03",
        big_img_src: "static/img/door-with-slat/03-1-B.png",
        small_img_src: "static/img/door-with-slat/03-1-S.png",
    },
    StalDoors {
        name: "03",
        big_img_src: "static/img/door-with-slat/03-2-B.png",
        small_img_src: "static/img/door-with-slat/03-2-S.png",
    },
    StalDoors {
        name: "03",
        big_img_src: "static/img/door-with-slat/03-3-B.jpg",
        small_img_src: "static/img/door-with-slat/03-3-S.jpg",
    },
    StalDoors {
        name: "04",
        big_img_src: "static/img/door-with-slat/04-1-B.png",
        small_img_src: "static/img/door-with-slat/04-1-S.png",
    },
    StalDoors {
        name: "04",
        big_img_src: "static/img/door-with-slat/04-2-B.jpg",
        small_img_src: "static/img/door-with-slat/04-2-S.jpg",
    },
    StalDoors {
        name: "04",
        big_img_src: "static/img/door-with-slat/04-3-B.jpg",
        small_img_src: "static/img/door-with-slat/04-3-S.jpg",
    },
    StalDoors {
        name: "04",
        big_img_src: "static/img/door-with-slat/04-4-B.jpg",
        small_img_src: "static/img/door-with-slat/04-4-S.jpg",
    },
    StalDoors {
        name: "04",
        big_img_src: "static/img/door-with-slat/04-5-B.jpg",
        small_img_src: "static/img/door-with-slat/04-5-S.jpg",
    },
    StalDoors {
        name: "JODŁA 01",
        big_img_src: "static/img/door-with-slat/JODLA-1-B.png",
        small_img_src: "static/img/door-with-slat/JODLA-1-S.png",
    },
    StalDoors {
        name: "KRESKI 02",
        big_img_src: "static/img/door-with-slat/KRESKI-1-B.png",
        small_img_src: "static/img/door-with-slat/KRESKI-1-S.png",
    },
];
