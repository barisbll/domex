use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RoomDoor {
    pub name: &'static str,
    pub img_src: &'static str,
}

pub const LIST_OF_ROOM_DOORS: &[RoomDoor] = &[
    RoomDoor {
        img_src: "static/img/list_of_room_doors/MODERN-07-ORZECH-287x600-1.jpg",
        name: "MODERN 07 ORZECH",
    },
    RoomDoor {
        img_src: "static/img/list_of_room_doors/MODERN-BIALE-287x600-1.jpg",
        name: "MODERN PEŁNE BIAŁE",
    },
    RoomDoor {
        img_src: "static/img/list_of_room_doors/MODERN-JESION-287x600-1.jpg",
        name: "MODERN PEŁNE JESION",
    },
    RoomDoor {
        img_src: "static/img/list_of_room_doors/TRENDY-05-287x600-1.jpg",
        name: "TRENDY 05 JESION",
    },
    RoomDoor {
        img_src: "static/img/list_of_room_doors/TRENDY-05-DAB-SANREMO-287x600-1.jpg",
        name: "TRENDY 05 DĄB SANREMO",
    },
    RoomDoor {
        img_src: "static/img/list_of_room_doors/CATANIA-07-MODERN-287x600-1.png",
        name: "MODERN 07 DĄB CATANIA",
    },
    RoomDoor {
        img_src: "static/img/list_of_room_doors/CATANIA-MODERN-PELNE-287x600-1.png",
        name: "MODERN PEŁNE DĄB CATANIA",
    },
    RoomDoor {
        img_src: "static/img/list_of_room_doors/CATANIA-02N-MODERN-287x600-1.png",
        name: "MODERN 02N (SZYBA 3,4) DĄB CATANIA",
    },
    RoomDoor {
        img_src: "static/img/list_of_room_doors/MODERN-02-DAB-CATANIA-287x600-1.png",
        name: "MODERN 02 DĄB CATANIA",
    },
];
