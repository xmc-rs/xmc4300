#[doc = "Reader of register ESC_DL_STATUS"]
pub type R = crate::R<u16, super::ESC_DL_STATUS>;
#[doc = "PDI operational/EEPROM loaded correctly\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI_EEPROM_A {
    #[doc = "0: EEPROM not loaded, PDI not operational (no access to Process Data RAM)"]
    VALUE1 = 0,
    #[doc = "1: EEPROM loaded correctly, PDI operational (access to Process Data RAM)"]
    VALUE2 = 1,
}
impl From<PDI_EEPROM_A> for bool {
    #[inline(always)]
    fn from(variant: PDI_EEPROM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDI_EEPROM`"]
pub type PDI_EEPROM_R = crate::R<bool, PDI_EEPROM_A>;
impl PDI_EEPROM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI_EEPROM_A {
        match self.bits {
            false => PDI_EEPROM_A::VALUE1,
            true => PDI_EEPROM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDI_EEPROM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDI_EEPROM_A::VALUE2
    }
}
#[doc = "PDI Watchdog Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDI_WDT_S_A {
    #[doc = "0: Watchdog expired"]
    VALUE1 = 0,
    #[doc = "1: Watchdog reloaded"]
    VALUE2 = 1,
}
impl From<PDI_WDT_S_A> for bool {
    #[inline(always)]
    fn from(variant: PDI_WDT_S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDI_WDT_S`"]
pub type PDI_WDT_S_R = crate::R<bool, PDI_WDT_S_A>;
impl PDI_WDT_S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI_WDT_S_A {
        match self.bits {
            false => PDI_WDT_S_A::VALUE1,
            true => PDI_WDT_S_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDI_WDT_S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDI_WDT_S_A::VALUE2
    }
}
#[doc = "Enhanced Link detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELD_A {
    #[doc = "0: Deactivated for all ports"]
    VALUE1 = 0,
    #[doc = "1: Activated for at least one port"]
    VALUE2 = 1,
}
impl From<ELD_A> for bool {
    #[inline(always)]
    fn from(variant: ELD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ELD`"]
pub type ELD_R = crate::R<bool, ELD_A>;
impl ELD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELD_A {
        match self.bits {
            false => ELD_A::VALUE1,
            true => ELD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ELD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ELD_A::VALUE2
    }
}
#[doc = "Physical link on Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINK_P0_A {
    #[doc = "0: No link"]
    VALUE1 = 0,
    #[doc = "1: Link detected"]
    VALUE2 = 1,
}
impl From<LINK_P0_A> for bool {
    #[inline(always)]
    fn from(variant: LINK_P0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINK_P0`"]
pub type LINK_P0_R = crate::R<bool, LINK_P0_A>;
impl LINK_P0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINK_P0_A {
        match self.bits {
            false => LINK_P0_A::VALUE1,
            true => LINK_P0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P0_A::VALUE2
    }
}
#[doc = "Physical link on Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINK_P1_A {
    #[doc = "0: No link"]
    VALUE1 = 0,
    #[doc = "1: Link detected"]
    VALUE2 = 1,
}
impl From<LINK_P1_A> for bool {
    #[inline(always)]
    fn from(variant: LINK_P1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINK_P1`"]
pub type LINK_P1_R = crate::R<bool, LINK_P1_A>;
impl LINK_P1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINK_P1_A {
        match self.bits {
            false => LINK_P1_A::VALUE1,
            true => LINK_P1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P1_A::VALUE2
    }
}
#[doc = "Physical link on Port 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINK_P2_A {
    #[doc = "0: No link"]
    VALUE1 = 0,
    #[doc = "1: Link detected"]
    VALUE2 = 1,
}
impl From<LINK_P2_A> for bool {
    #[inline(always)]
    fn from(variant: LINK_P2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINK_P2`"]
pub type LINK_P2_R = crate::R<bool, LINK_P2_A>;
impl LINK_P2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINK_P2_A {
        match self.bits {
            false => LINK_P2_A::VALUE1,
            true => LINK_P2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P2_A::VALUE2
    }
}
#[doc = "Physical link on Port 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINK_P3_A {
    #[doc = "0: No link"]
    VALUE1 = 0,
    #[doc = "1: Link detected"]
    VALUE2 = 1,
}
impl From<LINK_P3_A> for bool {
    #[inline(always)]
    fn from(variant: LINK_P3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINK_P3`"]
pub type LINK_P3_R = crate::R<bool, LINK_P3_A>;
impl LINK_P3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINK_P3_A {
        match self.bits {
            false => LINK_P3_A::VALUE1,
            true => LINK_P3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P3_A::VALUE2
    }
}
#[doc = "Loop Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP0_A {
    #[doc = "0: Open"]
    VALUE1 = 0,
    #[doc = "1: Closed"]
    VALUE2 = 1,
}
impl From<LP0_A> for bool {
    #[inline(always)]
    fn from(variant: LP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LP0`"]
pub type LP0_R = crate::R<bool, LP0_A>;
impl LP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP0_A {
        match self.bits {
            false => LP0_A::VALUE1,
            true => LP0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP0_A::VALUE2
    }
}
#[doc = "Communication on Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_P0_A {
    #[doc = "0: No stable communication"]
    VALUE1 = 0,
    #[doc = "1: Communication established"]
    VALUE2 = 1,
}
impl From<COM_P0_A> for bool {
    #[inline(always)]
    fn from(variant: COM_P0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COM_P0`"]
pub type COM_P0_R = crate::R<bool, COM_P0_A>;
impl COM_P0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM_P0_A {
        match self.bits {
            false => COM_P0_A::VALUE1,
            true => COM_P0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COM_P0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COM_P0_A::VALUE2
    }
}
#[doc = "Loop Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP1_A {
    #[doc = "0: Open"]
    VALUE1 = 0,
    #[doc = "1: Closed"]
    VALUE2 = 1,
}
impl From<LP1_A> for bool {
    #[inline(always)]
    fn from(variant: LP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LP1`"]
pub type LP1_R = crate::R<bool, LP1_A>;
impl LP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP1_A {
        match self.bits {
            false => LP1_A::VALUE1,
            true => LP1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP1_A::VALUE2
    }
}
#[doc = "Communication on Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_P1_A {
    #[doc = "0: No stable communication"]
    VALUE1 = 0,
    #[doc = "1: Communication established"]
    VALUE2 = 1,
}
impl From<COM_P1_A> for bool {
    #[inline(always)]
    fn from(variant: COM_P1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COM_P1`"]
pub type COM_P1_R = crate::R<bool, COM_P1_A>;
impl COM_P1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM_P1_A {
        match self.bits {
            false => COM_P1_A::VALUE1,
            true => COM_P1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COM_P1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COM_P1_A::VALUE2
    }
}
#[doc = "Loop Port 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP2_A {
    #[doc = "0: Open"]
    VALUE1 = 0,
    #[doc = "1: Closed"]
    VALUE2 = 1,
}
impl From<LP2_A> for bool {
    #[inline(always)]
    fn from(variant: LP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LP2`"]
pub type LP2_R = crate::R<bool, LP2_A>;
impl LP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP2_A {
        match self.bits {
            false => LP2_A::VALUE1,
            true => LP2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP2_A::VALUE2
    }
}
#[doc = "Communication on Port 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_P2_A {
    #[doc = "0: No stable communication"]
    VALUE1 = 0,
    #[doc = "1: Communication established"]
    VALUE2 = 1,
}
impl From<COM_P2_A> for bool {
    #[inline(always)]
    fn from(variant: COM_P2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COM_P2`"]
pub type COM_P2_R = crate::R<bool, COM_P2_A>;
impl COM_P2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM_P2_A {
        match self.bits {
            false => COM_P2_A::VALUE1,
            true => COM_P2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COM_P2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COM_P2_A::VALUE2
    }
}
#[doc = "Loop Port 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LP3_A {
    #[doc = "0: Open"]
    VALUE1 = 0,
    #[doc = "1: Closed"]
    VALUE2 = 1,
}
impl From<LP3_A> for bool {
    #[inline(always)]
    fn from(variant: LP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LP3`"]
pub type LP3_R = crate::R<bool, LP3_A>;
impl LP3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP3_A {
        match self.bits {
            false => LP3_A::VALUE1,
            true => LP3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP3_A::VALUE2
    }
}
#[doc = "Communication on Port 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COM_P3_A {
    #[doc = "0: No stable communication"]
    VALUE1 = 0,
    #[doc = "1: Communication established"]
    VALUE2 = 1,
}
impl From<COM_P3_A> for bool {
    #[inline(always)]
    fn from(variant: COM_P3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COM_P3`"]
pub type COM_P3_R = crate::R<bool, COM_P3_A>;
impl COM_P3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COM_P3_A {
        match self.bits {
            false => COM_P3_A::VALUE1,
            true => COM_P3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COM_P3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COM_P3_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - PDI operational/EEPROM loaded correctly"]
    #[inline(always)]
    pub fn pdi_eeprom(&self) -> PDI_EEPROM_R {
        PDI_EEPROM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDI Watchdog Status"]
    #[inline(always)]
    pub fn pdi_wdt_s(&self) -> PDI_WDT_S_R {
        PDI_WDT_S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enhanced Link detection"]
    #[inline(always)]
    pub fn eld(&self) -> ELD_R {
        ELD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Physical link on Port 0"]
    #[inline(always)]
    pub fn link_p0(&self) -> LINK_P0_R {
        LINK_P0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Physical link on Port 1"]
    #[inline(always)]
    pub fn link_p1(&self) -> LINK_P1_R {
        LINK_P1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Physical link on Port 2"]
    #[inline(always)]
    pub fn link_p2(&self) -> LINK_P2_R {
        LINK_P2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Physical link on Port 3"]
    #[inline(always)]
    pub fn link_p3(&self) -> LINK_P3_R {
        LINK_P3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Loop Port 0"]
    #[inline(always)]
    pub fn lp0(&self) -> LP0_R {
        LP0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Communication on Port 0"]
    #[inline(always)]
    pub fn com_p0(&self) -> COM_P0_R {
        COM_P0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Loop Port 1"]
    #[inline(always)]
    pub fn lp1(&self) -> LP1_R {
        LP1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Communication on Port 1"]
    #[inline(always)]
    pub fn com_p1(&self) -> COM_P1_R {
        COM_P1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Loop Port 2"]
    #[inline(always)]
    pub fn lp2(&self) -> LP2_R {
        LP2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Communication on Port 2"]
    #[inline(always)]
    pub fn com_p2(&self) -> COM_P2_R {
        COM_P2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Loop Port 3"]
    #[inline(always)]
    pub fn lp3(&self) -> LP3_R {
        LP3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Communication on Port 3"]
    #[inline(always)]
    pub fn com_p3(&self) -> COM_P3_R {
        COM_P3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
