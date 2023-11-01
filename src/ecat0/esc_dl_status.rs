#[doc = "Register `ESC_DL_STATUS` reader"]
pub type R = crate::R<ESC_DL_STATUS_SPEC>;
#[doc = "Field `PDI_EEPROM` reader - PDI operational/EEPROM loaded correctly"]
pub type PDI_EEPROM_R = crate::BitReader<PDI_EEPROM_A>;
#[doc = "PDI operational/EEPROM loaded correctly\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PDI_EEPROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDI_EEPROM_A {
        match self.bits {
            false => PDI_EEPROM_A::VALUE1,
            true => PDI_EEPROM_A::VALUE2,
        }
    }
    #[doc = "EEPROM not loaded, PDI not operational (no access to Process Data RAM)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDI_EEPROM_A::VALUE1
    }
    #[doc = "EEPROM loaded correctly, PDI operational (access to Process Data RAM)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDI_EEPROM_A::VALUE2
    }
}
#[doc = "Field `PDI_WDT_S` reader - PDI Watchdog Status"]
pub type PDI_WDT_S_R = crate::BitReader<PDI_WDT_S_A>;
#[doc = "PDI Watchdog Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PDI_WDT_S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDI_WDT_S_A {
        match self.bits {
            false => PDI_WDT_S_A::VALUE1,
            true => PDI_WDT_S_A::VALUE2,
        }
    }
    #[doc = "Watchdog expired"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDI_WDT_S_A::VALUE1
    }
    #[doc = "Watchdog reloaded"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDI_WDT_S_A::VALUE2
    }
}
#[doc = "Field `ELD` reader - Enhanced Link detection"]
pub type ELD_R = crate::BitReader<ELD_A>;
#[doc = "Enhanced Link detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ELD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ELD_A {
        match self.bits {
            false => ELD_A::VALUE1,
            true => ELD_A::VALUE2,
        }
    }
    #[doc = "Deactivated for all ports"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ELD_A::VALUE1
    }
    #[doc = "Activated for at least one port"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ELD_A::VALUE2
    }
}
#[doc = "Field `LINK_P0` reader - Physical link on Port 0"]
pub type LINK_P0_R = crate::BitReader<LINK_P0_A>;
#[doc = "Physical link on Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LINK_P0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINK_P0_A {
        match self.bits {
            false => LINK_P0_A::VALUE1,
            true => LINK_P0_A::VALUE2,
        }
    }
    #[doc = "No link"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P0_A::VALUE1
    }
    #[doc = "Link detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P0_A::VALUE2
    }
}
#[doc = "Field `LINK_P1` reader - Physical link on Port 1"]
pub type LINK_P1_R = crate::BitReader<LINK_P1_A>;
#[doc = "Physical link on Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LINK_P1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINK_P1_A {
        match self.bits {
            false => LINK_P1_A::VALUE1,
            true => LINK_P1_A::VALUE2,
        }
    }
    #[doc = "No link"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P1_A::VALUE1
    }
    #[doc = "Link detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P1_A::VALUE2
    }
}
#[doc = "Field `LINK_P2` reader - Physical link on Port 2"]
pub type LINK_P2_R = crate::BitReader<LINK_P2_A>;
#[doc = "Physical link on Port 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LINK_P2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINK_P2_A {
        match self.bits {
            false => LINK_P2_A::VALUE1,
            true => LINK_P2_A::VALUE2,
        }
    }
    #[doc = "No link"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P2_A::VALUE1
    }
    #[doc = "Link detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P2_A::VALUE2
    }
}
#[doc = "Field `LINK_P3` reader - Physical link on Port 3"]
pub type LINK_P3_R = crate::BitReader<LINK_P3_A>;
#[doc = "Physical link on Port 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LINK_P3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINK_P3_A {
        match self.bits {
            false => LINK_P3_A::VALUE1,
            true => LINK_P3_A::VALUE2,
        }
    }
    #[doc = "No link"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LINK_P3_A::VALUE1
    }
    #[doc = "Link detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LINK_P3_A::VALUE2
    }
}
#[doc = "Field `LP0` reader - Loop Port 0"]
pub type LP0_R = crate::BitReader<LP0_A>;
#[doc = "Loop Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LP0_A {
        match self.bits {
            false => LP0_A::VALUE1,
            true => LP0_A::VALUE2,
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP0_A::VALUE1
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP0_A::VALUE2
    }
}
#[doc = "Field `COM_P0` reader - Communication on Port 0"]
pub type COM_P0_R = crate::BitReader<COM_P0_A>;
#[doc = "Communication on Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl COM_P0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM_P0_A {
        match self.bits {
            false => COM_P0_A::VALUE1,
            true => COM_P0_A::VALUE2,
        }
    }
    #[doc = "No stable communication"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COM_P0_A::VALUE1
    }
    #[doc = "Communication established"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COM_P0_A::VALUE2
    }
}
#[doc = "Field `LP1` reader - Loop Port 1"]
pub type LP1_R = crate::BitReader<LP1_A>;
#[doc = "Loop Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LP1_A {
        match self.bits {
            false => LP1_A::VALUE1,
            true => LP1_A::VALUE2,
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP1_A::VALUE1
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP1_A::VALUE2
    }
}
#[doc = "Field `COM_P1` reader - Communication on Port 1"]
pub type COM_P1_R = crate::BitReader<COM_P1_A>;
#[doc = "Communication on Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl COM_P1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM_P1_A {
        match self.bits {
            false => COM_P1_A::VALUE1,
            true => COM_P1_A::VALUE2,
        }
    }
    #[doc = "No stable communication"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COM_P1_A::VALUE1
    }
    #[doc = "Communication established"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COM_P1_A::VALUE2
    }
}
#[doc = "Field `LP2` reader - Loop Port 2"]
pub type LP2_R = crate::BitReader<LP2_A>;
#[doc = "Loop Port 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LP2_A {
        match self.bits {
            false => LP2_A::VALUE1,
            true => LP2_A::VALUE2,
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP2_A::VALUE1
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP2_A::VALUE2
    }
}
#[doc = "Field `COM_P2` reader - Communication on Port 2"]
pub type COM_P2_R = crate::BitReader<COM_P2_A>;
#[doc = "Communication on Port 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl COM_P2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM_P2_A {
        match self.bits {
            false => COM_P2_A::VALUE1,
            true => COM_P2_A::VALUE2,
        }
    }
    #[doc = "No stable communication"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COM_P2_A::VALUE1
    }
    #[doc = "Communication established"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COM_P2_A::VALUE2
    }
}
#[doc = "Field `LP3` reader - Loop Port 3"]
pub type LP3_R = crate::BitReader<LP3_A>;
#[doc = "Loop Port 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LP3_A {
        match self.bits {
            false => LP3_A::VALUE1,
            true => LP3_A::VALUE2,
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP3_A::VALUE1
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP3_A::VALUE2
    }
}
#[doc = "Field `COM_P3` reader - Communication on Port 3"]
pub type COM_P3_R = crate::BitReader<COM_P3_A>;
#[doc = "Communication on Port 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl COM_P3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM_P3_A {
        match self.bits {
            false => COM_P3_A::VALUE1,
            true => COM_P3_A::VALUE2,
        }
    }
    #[doc = "No stable communication"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == COM_P3_A::VALUE1
    }
    #[doc = "Communication established"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == COM_P3_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - PDI operational/EEPROM loaded correctly"]
    #[inline(always)]
    pub fn pdi_eeprom(&self) -> PDI_EEPROM_R {
        PDI_EEPROM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDI Watchdog Status"]
    #[inline(always)]
    pub fn pdi_wdt_s(&self) -> PDI_WDT_S_R {
        PDI_WDT_S_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enhanced Link detection"]
    #[inline(always)]
    pub fn eld(&self) -> ELD_R {
        ELD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Physical link on Port 0"]
    #[inline(always)]
    pub fn link_p0(&self) -> LINK_P0_R {
        LINK_P0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Physical link on Port 1"]
    #[inline(always)]
    pub fn link_p1(&self) -> LINK_P1_R {
        LINK_P1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Physical link on Port 2"]
    #[inline(always)]
    pub fn link_p2(&self) -> LINK_P2_R {
        LINK_P2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Physical link on Port 3"]
    #[inline(always)]
    pub fn link_p3(&self) -> LINK_P3_R {
        LINK_P3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Loop Port 0"]
    #[inline(always)]
    pub fn lp0(&self) -> LP0_R {
        LP0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Communication on Port 0"]
    #[inline(always)]
    pub fn com_p0(&self) -> COM_P0_R {
        COM_P0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Loop Port 1"]
    #[inline(always)]
    pub fn lp1(&self) -> LP1_R {
        LP1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Communication on Port 1"]
    #[inline(always)]
    pub fn com_p1(&self) -> COM_P1_R {
        COM_P1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loop Port 2"]
    #[inline(always)]
    pub fn lp2(&self) -> LP2_R {
        LP2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Communication on Port 2"]
    #[inline(always)]
    pub fn com_p2(&self) -> COM_P2_R {
        COM_P2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Loop Port 3"]
    #[inline(always)]
    pub fn lp3(&self) -> LP3_R {
        LP3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Communication on Port 3"]
    #[inline(always)]
    pub fn com_p3(&self) -> COM_P3_R {
        COM_P3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "ESC DL Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_dl_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESC_DL_STATUS_SPEC;
impl crate::RegisterSpec for ESC_DL_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`esc_dl_status::R`](R) reader structure"]
impl crate::Readable for ESC_DL_STATUS_SPEC {}
#[doc = "`reset()` method sets ESC_DL_STATUS to value 0x5000"]
impl crate::Resettable for ESC_DL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x5000;
}
