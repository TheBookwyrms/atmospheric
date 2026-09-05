#[derive(Clone, Copy)]
pub struct MaterialLightQualities {
    ambient_reflected_light  : [f32; 3],
    diffuse_reflected_light  : [f32; 3],
    specular_reflected_light : [f32; 3],
    shininess:f32,
}
impl MaterialLightQualities {
    pub fn get_ambient(&self)   -> [f32; 3] { self.ambient_reflected_light }
    pub fn get_diffuse(&self)   -> [f32; 3] { self.diffuse_reflected_light }
    pub fn get_specular(&self)  -> [f32; 3] { self.specular_reflected_light }
    pub fn get_shininess(&self) ->       f32       { self.shininess }
    pub fn get_components_array(&self) -> [f32; 10] {
        [
            self.ambient_reflected_light[0], self.ambient_reflected_light[1], self.ambient_reflected_light[2],
            self.diffuse_reflected_light[0], self.diffuse_reflected_light[1], self.diffuse_reflected_light[2],
            self.specular_reflected_light[0], self.specular_reflected_light[1], self.specular_reflected_light[2],
            self.shininess
        ]
    }
    pub fn assign(ambient:[f32;3], diffuse:[f32;3], specular:[f32;3], shininess:f32) -> Self {
        Self {
            ambient_reflected_light:ambient,
            diffuse_reflected_light:diffuse,
            specular_reflected_light:specular,
            shininess
        }
    }
}



#[derive(Clone, Copy)]
pub enum Material {
    Default,
    Brass,
    Bronze,
    PolishedBronze,
    Chrome,
    Copper,
    PolishedCopper,
    Gold,
    PolishedGold,
    Pewter,
    Silver,
    PolishedSilver,
    Emerald,
    Jade,
    Obsidian,
    Pearl,
    Ruby,
    Turquoise,
    BlackPlastic,
    CyanPlastic,
    GreenPlastic,
    RedPlastic,
    WhitePlastic,
    YellowPlastic,
    BlackRubber,
    CyanRubber,
    GreenRubber,
    RedRubber,
    WhiteRubber,
    YellowRubber,
    Custom(MaterialLightQualities),
}
impl Material {
    pub fn get_material_qualities(&self) -> MaterialLightQualities {
        match self {
            Material::Default => MaterialLightQualities {
                ambient_reflected_light:  [1.0, 1.0, 1.0],
                diffuse_reflected_light:  [1.0, 1.0, 1.0],
                specular_reflected_light: [1.0, 1.0, 1.0],
                shininess: 32.0
            },
            Material::Brass => BRASS,
            Material::Bronze => BRONZE,
            Material::PolishedBronze => POLISHED_BRONZE,
            Material::Chrome => CHROME,
            Material::Copper => COPPER,
            Material::PolishedCopper => POLISHED_COPPER,
            Material::Gold => GOLD,
            Material::PolishedGold => POLISHED_GOLD,
            Material::Pewter => PEWTER,
            Material::Silver => SILVER,
            Material::PolishedSilver => POLISHED_SILVER,
            Material::Emerald => EMERALD,
            Material::Jade => JADE,
            Material::Obsidian => OBSIDIAN,
            Material::Pearl => PEARL,
            Material::Ruby => RUBY,
            Material::Turquoise => TURQUOISE,
            Material::BlackPlastic => BLACK_PLASTIC,
            Material::CyanPlastic => CYAN_PLASTIC,
            Material::GreenPlastic => GREEN_PLASTIC,
            Material::RedPlastic => RED_PLASTIC,
            Material::WhitePlastic => WHITE_PLASTIC,
            Material::YellowPlastic => YELLOW_PLASTIC,
            Material::BlackRubber => BLACK_RUBBER,
            Material::CyanRubber => CYAN_RUBBER,
            Material::GreenRubber => GREEN_RUBBER,
            Material::RedRubber => RED_RUBBER,
            Material::WhiteRubber => WHITE_RUBBER,
            Material::YellowRubber => YELLOW_RUBBER,
            Material::Custom(qualities) => *qualities,
        }
    }
}





// sources for material colour properties:
// https://web.archive.org/web/20100725103839/http://www.cs.utk.edu/~kuck/materials_ogl.htm
// http://www.it.hiof.no/~borres/j3d/explain/light/p-materials.html
// http://devernay.free.fr/cours/opengl/materials.html
// https://www.opengl.org/archives/resources/code/samples/sig99/advanced99/notes/node153.html




pub const BRASS:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.329412, 0.223529, 0.027451],
    diffuse_reflected_light  : [0.780392, 0.568627, 0.113725],
    specular_reflected_light : [0.992157, 0.941176, 0.807843],
    shininess                : 27.8974,
};

pub const BRONZE:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.2125, 0.1275, 0.054],
    diffuse_reflected_light  : [0.714, 0.4284, 0.18144],
    specular_reflected_light : [0.393548, 0.271906, 0.166721],
    shininess                : 25.6,
};


pub const POLISHED_BRONZE:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.25, 0.148, 0.06475],
    diffuse_reflected_light  : [0.4, 0.2368, 0.1036,],
    specular_reflected_light : [0.774597, 0.458561, 0.200621],
    shininess                :  76.8,
};

pub const CHROME:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.25, 0.25, 0.25],
    diffuse_reflected_light  : [0.4, 0.4, 0.4],
    specular_reflected_light : [0.774597, 0.774597, 0.774597],
    shininess                :  76.8,
};


pub const COPPER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.19125, 0.0735, 0.0225],
    diffuse_reflected_light  : [0.7038, 0.27048, 0.0828],
    specular_reflected_light : [0.256777, 0.137622, 0.086014],
    shininess                :  12.8,
};


pub const POLISHED_COPPER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.2295, 0.08825, 0.0275],
    diffuse_reflected_light  : [0.5508, 0.2118, 0.066, ],
    specular_reflected_light : [0.580594, 0.223257, 0.0695701],
    shininess                :  51.2,
};


pub const GOLD:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.24725, 0.1995, 0.0745],
    diffuse_reflected_light  : [0.75164, 0.60648, 0.22648],
    specular_reflected_light : [0.628281, 0.555802, 0.366065],
    shininess                :  51.2,
};


pub const POLISHED_GOLD:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.24725, 0.2245, 0.0645],
    diffuse_reflected_light  : [0.34615, 0.3143, 0.0903],
    specular_reflected_light : [0.797357, 0.723991, 0.208006],
    shininess                :  83.2,
};


pub const PEWTER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.105882, 0.058824, 0.113725],
    diffuse_reflected_light  : [0.427451, 0.470588, 0.541176],
    specular_reflected_light : [0.333333, 0.333333, 0.521569],
    shininess                :  9.84615,
};


pub const SILVER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.19225, 0.19225, 0.19225],
    diffuse_reflected_light  : [0.50754, 0.50754, 0.50754],
    specular_reflected_light : [0.508273, 0.508273, 0.508273],
    shininess                :  51.2,
};


pub const POLISHED_SILVER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.23125, 0.23125, 0.23125],
    diffuse_reflected_light  : [0.2775, 0.2775, 0.2775],
    specular_reflected_light : [0.773911, 0.773911, 0.773911],
    shininess                :  89.6,
};


pub const EMERALD:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.0215, 0.1745, 0.0215],
    diffuse_reflected_light  : [0.07568, 0.61424, 0.07568],
    specular_reflected_light : [0.633, 0.727811, 0.633],
    shininess                :  76.8,
};


pub const JADE:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.135, 0.2225, 0.1575],
    diffuse_reflected_light  : [0.54, 0.89, 0.63],
    specular_reflected_light : [0.316228, 0.316228, 0.316228],
    shininess                :  12.8,
};


pub const OBSIDIAN:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.05375, 0.05, 0.06625],
    diffuse_reflected_light  : [0.18275, 0.17, 0.22525],
    specular_reflected_light : [0.332741, 0.328634, 0.346435],
    shininess                :  38.4,
};


pub const PEARL:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.25, 0.20725, 0.20725],
    diffuse_reflected_light  : [1.0, 0.829, 0.829],
    specular_reflected_light : [0.296648, 0.296648, 0.296648],
    shininess                :  11.264,
};


pub const RUBY:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.1745, 0.01175, 0.01175],
    diffuse_reflected_light  : [0.61424, 0.04136, 0.04136],
    specular_reflected_light : [0.727811, 0.626959, 0.626959],
    shininess                :  76.8,
};


pub const TURQUOISE:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.1, 0.18725, 0.1745],
    diffuse_reflected_light  : [0.396, 0.74151, 0.69102],
    specular_reflected_light : [0.297254, 0.30829, 0.306678],
    shininess                :  12.8,
};


pub const BLACK_PLASTIC:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.0, 0.0, 0.0],
    diffuse_reflected_light  : [0.01, 0.01, 0.01],
    specular_reflected_light : [0.50, 0.50, 0.50],
    shininess                :  32.0,
};

pub const CYAN_PLASTIC:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.0, 0.1, 0.06],
    diffuse_reflected_light  : [0.0, 0.50980392, 0.50980392],
    specular_reflected_light : [0.50196078, 0.50196078, 0.50196078],
    shininess                :  32.0,
};


pub const GREEN_PLASTIC:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.0, 0.0, 0.0],
    diffuse_reflected_light  : [0.1, 0.35, 0.1],
    specular_reflected_light : [0.45, 0.55, 0.45],
    shininess                :  32.0,
};


pub const RED_PLASTIC:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.0, 0.0, 0.0],
    diffuse_reflected_light  : [0.5, 0.0, 0.0],
    specular_reflected_light : [0.7, 0.6, 0.6],
    shininess                :  32.0,
};


pub const WHITE_PLASTIC:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.0, 0.0, 0.0],
    diffuse_reflected_light  : [0.55, 0.55, 0.55],
    specular_reflected_light : [0.70, 0.70, 0.70],
    shininess                :  32.0,
};


pub const YELLOW_PLASTIC:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.0, 0.0, 0.0],
    diffuse_reflected_light  : [0.5, 0.5, 0.0],
    specular_reflected_light : [0.60, 0.60, 0.50],
    shininess                :  32.0,
};


pub const BLACK_RUBBER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.02, 0.02, 0.02],
    diffuse_reflected_light  : [0.01, 0.01, 0.01],
    specular_reflected_light : [0.4, 0.4, 0.4],
    shininess                :  10.0,
};


pub const CYAN_RUBBER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.0, 0.05, 0.05],
    diffuse_reflected_light  : [0.4, 0.5, 0.5],
    specular_reflected_light : [0.04, 0.7, 0.7],
    shininess                :  10.0,
};


pub const GREEN_RUBBER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.0, 0.05, 0.0],
    diffuse_reflected_light  : [0.4, 0.5, 0.4],
    specular_reflected_light : [0.04, 0.7, 0.04],
    shininess                :  10.0,
};


pub const RED_RUBBER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.05, 0.0, 0.0],
    diffuse_reflected_light  : [0.5, 0.4, 0.4],
    specular_reflected_light : [0.7, 0.04, 0.04],
    shininess                :  10.0,
};


pub const WHITE_RUBBER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.05, 0.05, 0.05],
    diffuse_reflected_light  : [0.5, 0.5, 0.5],
    specular_reflected_light : [0.7, 0.7, 0.7],
    shininess                :  10.0,
};


pub const YELLOW_RUBBER:MaterialLightQualities = MaterialLightQualities {
    ambient_reflected_light  : [0.05, 0.05, 0.0],
    diffuse_reflected_light  : [0.5, 0.5, 0.4],
    specular_reflected_light : [0.7, 0.7, 0.04],
    shininess                :  10.0,
};





