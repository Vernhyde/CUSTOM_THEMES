//! DOC COMMENT (`comment.doc` -> #a8845e Italic - M81 Field Khaki)
// STANDARD COMMENT (`comment` -> #8a8280 Italic - Vasha Grey Fur)

use bevy::prelude::*;

// ATTRIBUTE (`attribute` -> #f09654 Italic - Canyon Ochre)
#[derive(Component, Debug, Clone)]
pub struct TacticalPawn<'a> { // KEYWORD (#e5a93d Italic) | TYPE (#ffe3bb Bone) | LIFETIME (#c97a90 Italic)
    pub callsign: &'a str,    // PROPERTY (#c87a53 Horn)  | BUILTIN TYPE (#ffe3bb Italic)
    pub health: f32,          // PUNCTUATION (#afabaa)    | DELIMITER (#8a8280)
    pub is_cloaked: bool,
}

// CONSTANT (`constant` -> #fee344 Eye Yellow)
pub const MAX_SIGIL_VERTICES: usize = 10; // NUMBER (`number` -> #fee344)

#[derive(PartialEq)]
pub enum CamoPattern {
    M81Woodland, // ENUM VARIANT (`variant` -> #7ec1cf Oasis Slate-Cyan)
    RangerGreen,
}

impl<'a> TacticalPawn<'a> {
    // FUNCTION / METHOD (`function.method` -> #e69870 Bright Ochre)
    pub fn deploy_sigil(&mut self, pattern: CamoPattern) -> bool {
        // SPECIAL VARIABLE (`variable.special` -> #ff5277 Italic - High-Vis Crimson)
        let active_unit = self.callsign; // VARIABLE (`variable` -> #dcdcdc Foreground)

        // STRING (`string` -> #7fa36b M81 Woodland Leaf) & ESCAPE (`string.escape` -> #7ec1cf)
        let status_msg = "Deploying Shieldhold sigil...\n\t[OK]";

        // MACRO (`function.macro` -> #d9385a Flesh Crimson)
        info!("Unit {}: {}", active_unit, status_msg);

        // OPERATOR (`operator` -> #e5a93d Gold) & BOOLEAN (`boolean` -> #fee344 Italic)
        if self.health > 0.0 && pattern == CamoPattern::M81Woodland {
            self.is_cloaked = true;
            return true;
        }
        false
    }
}
