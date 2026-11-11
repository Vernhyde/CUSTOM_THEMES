//! DOC COMMENT (`comment.doc` -> Dark: #a8845e | Light: #825c38 Italic - M81 Field Khaki)
// STANDARD COMMENT (`comment` -> Dark: #8a8280 | Light: #6f534c Italic - Grey/Dark Fur)

use bevy::prelude::*;

// ATTRIBUTE & DERIVE TRAITS (`attribute` & `type.interface` -> Dark: #f09654 | Light: #b04818 Italic)
#[derive(Component, Debug, Clone)]
pub struct TacticalPawn<'a> {
    // KEYWORD (#e5a93d / #b55400) | TYPE (#ffe3bb / #16191d) | LIFETIME (#c97a90 / #8c3852)
    pub callsign: &'a str, // PROPERTY (#c87a53 / #6e4228) | BUILTIN TYPE (#ffe3bb / #16191d Italic)
    pub health: f32,       // PUNCTUATION (#afabaa / #53383d) | DELIMITER (#8a8280 / #6f534c)
    pub is_cloaked: bool,
}

// CONSTANT (`constant` -> Dark: #fee344 Eye Yellow | Light: #8a6200 Antique Bronze)
pub const MAX_SIGIL_VERTICES: usize = 10; // NUMBER (`number` -> Dark: #fee344 | Light: #8a6200)

#[derive(PartialEq)]
pub enum CamoPattern {
    M81Woodland, // ENUM VARIANT (`enumMember` / `variant` -> Dark: #7ec1cf | Light: #1f6878 Oasis Cyan)
    RangerGreen,
}

impl<'a> TacticalPawn<'a> {
    // FUNCTION / METHOD (`function.method` -> Dark: #e69870 Bright Ochre | Light: #9c4119 Terracotta Horn)
    pub fn deploy_sigil(&mut self, pattern: CamoPattern) -> bool {
        // SPECIAL VARIABLE (`variable.special` -> Dark: #ff5277 | Light: #cf1f45 Italic - High-Vis Crimson)
        let active_unit = self.callsign; // VARIABLE (`variable` -> Dark: #dcdcdc | Light: #26181c)

        // STRING (`string` -> Dark: #7fa36b | Light: #3b5e2b) & ESCAPE (`string.escape` -> Dark: #7ec1cf | Light: #1f6878)
        let status_msg = "Deploying Shieldhold sigil...\n\t[OK]";

        // MACRO (`function.macro` -> Dark: #d9385a | Light: #b81d3d Flesh Crimson)
        info!("Unit {}: {}", active_unit, status_msg);

        // OPERATOR (`operator` -> Dark: #e5a93d | Light: #b55400) & BOOLEAN (`boolean` -> Dark: #fee344 | Light: #8a6200 Italic)
        if self.health > 0.0 && pattern == CamoPattern::M81Woodland {
            self.is_cloaked = true;
            return true;
        }
        false
    }
}
