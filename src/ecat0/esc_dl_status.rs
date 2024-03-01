#[doc = "Register `ESC_DL_STATUS` reader"]
pub type R = crate::R<EscDlStatusSpec>;
#[doc = "PDI operational/EEPROM loaded correctly\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdiEeprom {
    #[doc = "0: EEPROM not loaded, PDI not operational (no access to Process Data RAM)"]
    Value1 = 0,
    #[doc = "1: EEPROM loaded correctly, PDI operational (access to Process Data RAM)"]
    Value2 = 1,
}
impl From<PdiEeprom> for bool {
    #[inline(always)]
    fn from(variant: PdiEeprom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDI_EEPROM` reader - PDI operational/EEPROM loaded correctly"]
pub type PdiEepromR = crate::BitReader<PdiEeprom>;
impl PdiEepromR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdiEeprom {
        match self.bits {
            false => PdiEeprom::Value1,
            true => PdiEeprom::Value2,
        }
    }
    #[doc = "EEPROM not loaded, PDI not operational (no access to Process Data RAM)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PdiEeprom::Value1
    }
    #[doc = "EEPROM loaded correctly, PDI operational (access to Process Data RAM)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PdiEeprom::Value2
    }
}
#[doc = "PDI Watchdog Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdiWdtS {
    #[doc = "0: Watchdog expired"]
    Value1 = 0,
    #[doc = "1: Watchdog reloaded"]
    Value2 = 1,
}
impl From<PdiWdtS> for bool {
    #[inline(always)]
    fn from(variant: PdiWdtS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDI_WDT_S` reader - PDI Watchdog Status"]
pub type PdiWdtSR = crate::BitReader<PdiWdtS>;
impl PdiWdtSR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdiWdtS {
        match self.bits {
            false => PdiWdtS::Value1,
            true => PdiWdtS::Value2,
        }
    }
    #[doc = "Watchdog expired"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PdiWdtS::Value1
    }
    #[doc = "Watchdog reloaded"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PdiWdtS::Value2
    }
}
#[doc = "Enhanced Link detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eld {
    #[doc = "0: Deactivated for all ports"]
    Value1 = 0,
    #[doc = "1: Activated for at least one port"]
    Value2 = 1,
}
impl From<Eld> for bool {
    #[inline(always)]
    fn from(variant: Eld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELD` reader - Enhanced Link detection"]
pub type EldR = crate::BitReader<Eld>;
impl EldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eld {
        match self.bits {
            false => Eld::Value1,
            true => Eld::Value2,
        }
    }
    #[doc = "Deactivated for all ports"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eld::Value1
    }
    #[doc = "Activated for at least one port"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eld::Value2
    }
}
#[doc = "Physical link on Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkP0 {
    #[doc = "0: No link"]
    Value1 = 0,
    #[doc = "1: Link detected"]
    Value2 = 1,
}
impl From<LinkP0> for bool {
    #[inline(always)]
    fn from(variant: LinkP0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINK_P0` reader - Physical link on Port 0"]
pub type LinkP0R = crate::BitReader<LinkP0>;
impl LinkP0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinkP0 {
        match self.bits {
            false => LinkP0::Value1,
            true => LinkP0::Value2,
        }
    }
    #[doc = "No link"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LinkP0::Value1
    }
    #[doc = "Link detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LinkP0::Value2
    }
}
#[doc = "Physical link on Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkP1 {
    #[doc = "0: No link"]
    Value1 = 0,
    #[doc = "1: Link detected"]
    Value2 = 1,
}
impl From<LinkP1> for bool {
    #[inline(always)]
    fn from(variant: LinkP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINK_P1` reader - Physical link on Port 1"]
pub type LinkP1R = crate::BitReader<LinkP1>;
impl LinkP1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinkP1 {
        match self.bits {
            false => LinkP1::Value1,
            true => LinkP1::Value2,
        }
    }
    #[doc = "No link"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LinkP1::Value1
    }
    #[doc = "Link detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LinkP1::Value2
    }
}
#[doc = "Physical link on Port 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkP2 {
    #[doc = "0: No link"]
    Value1 = 0,
    #[doc = "1: Link detected"]
    Value2 = 1,
}
impl From<LinkP2> for bool {
    #[inline(always)]
    fn from(variant: LinkP2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINK_P2` reader - Physical link on Port 2"]
pub type LinkP2R = crate::BitReader<LinkP2>;
impl LinkP2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinkP2 {
        match self.bits {
            false => LinkP2::Value1,
            true => LinkP2::Value2,
        }
    }
    #[doc = "No link"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LinkP2::Value1
    }
    #[doc = "Link detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LinkP2::Value2
    }
}
#[doc = "Physical link on Port 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkP3 {
    #[doc = "0: No link"]
    Value1 = 0,
    #[doc = "1: Link detected"]
    Value2 = 1,
}
impl From<LinkP3> for bool {
    #[inline(always)]
    fn from(variant: LinkP3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINK_P3` reader - Physical link on Port 3"]
pub type LinkP3R = crate::BitReader<LinkP3>;
impl LinkP3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinkP3 {
        match self.bits {
            false => LinkP3::Value1,
            true => LinkP3::Value2,
        }
    }
    #[doc = "No link"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LinkP3::Value1
    }
    #[doc = "Link detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LinkP3::Value2
    }
}
#[doc = "Loop Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lp0 {
    #[doc = "0: Open"]
    Value1 = 0,
    #[doc = "1: Closed"]
    Value2 = 1,
}
impl From<Lp0> for bool {
    #[inline(always)]
    fn from(variant: Lp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP0` reader - Loop Port 0"]
pub type Lp0R = crate::BitReader<Lp0>;
impl Lp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lp0 {
        match self.bits {
            false => Lp0::Value1,
            true => Lp0::Value2,
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lp0::Value1
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lp0::Value2
    }
}
#[doc = "Communication on Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComP0 {
    #[doc = "0: No stable communication"]
    Value1 = 0,
    #[doc = "1: Communication established"]
    Value2 = 1,
}
impl From<ComP0> for bool {
    #[inline(always)]
    fn from(variant: ComP0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COM_P0` reader - Communication on Port 0"]
pub type ComP0R = crate::BitReader<ComP0>;
impl ComP0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ComP0 {
        match self.bits {
            false => ComP0::Value1,
            true => ComP0::Value2,
        }
    }
    #[doc = "No stable communication"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ComP0::Value1
    }
    #[doc = "Communication established"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ComP0::Value2
    }
}
#[doc = "Loop Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lp1 {
    #[doc = "0: Open"]
    Value1 = 0,
    #[doc = "1: Closed"]
    Value2 = 1,
}
impl From<Lp1> for bool {
    #[inline(always)]
    fn from(variant: Lp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP1` reader - Loop Port 1"]
pub type Lp1R = crate::BitReader<Lp1>;
impl Lp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lp1 {
        match self.bits {
            false => Lp1::Value1,
            true => Lp1::Value2,
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lp1::Value1
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lp1::Value2
    }
}
#[doc = "Communication on Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComP1 {
    #[doc = "0: No stable communication"]
    Value1 = 0,
    #[doc = "1: Communication established"]
    Value2 = 1,
}
impl From<ComP1> for bool {
    #[inline(always)]
    fn from(variant: ComP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COM_P1` reader - Communication on Port 1"]
pub type ComP1R = crate::BitReader<ComP1>;
impl ComP1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ComP1 {
        match self.bits {
            false => ComP1::Value1,
            true => ComP1::Value2,
        }
    }
    #[doc = "No stable communication"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ComP1::Value1
    }
    #[doc = "Communication established"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ComP1::Value2
    }
}
#[doc = "Loop Port 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lp2 {
    #[doc = "0: Open"]
    Value1 = 0,
    #[doc = "1: Closed"]
    Value2 = 1,
}
impl From<Lp2> for bool {
    #[inline(always)]
    fn from(variant: Lp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP2` reader - Loop Port 2"]
pub type Lp2R = crate::BitReader<Lp2>;
impl Lp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lp2 {
        match self.bits {
            false => Lp2::Value1,
            true => Lp2::Value2,
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lp2::Value1
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lp2::Value2
    }
}
#[doc = "Communication on Port 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComP2 {
    #[doc = "0: No stable communication"]
    Value1 = 0,
    #[doc = "1: Communication established"]
    Value2 = 1,
}
impl From<ComP2> for bool {
    #[inline(always)]
    fn from(variant: ComP2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COM_P2` reader - Communication on Port 2"]
pub type ComP2R = crate::BitReader<ComP2>;
impl ComP2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ComP2 {
        match self.bits {
            false => ComP2::Value1,
            true => ComP2::Value2,
        }
    }
    #[doc = "No stable communication"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ComP2::Value1
    }
    #[doc = "Communication established"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ComP2::Value2
    }
}
#[doc = "Loop Port 3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lp3 {
    #[doc = "0: Open"]
    Value1 = 0,
    #[doc = "1: Closed"]
    Value2 = 1,
}
impl From<Lp3> for bool {
    #[inline(always)]
    fn from(variant: Lp3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP3` reader - Loop Port 3"]
pub type Lp3R = crate::BitReader<Lp3>;
impl Lp3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lp3 {
        match self.bits {
            false => Lp3::Value1,
            true => Lp3::Value2,
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lp3::Value1
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lp3::Value2
    }
}
#[doc = "Communication on Port 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComP3 {
    #[doc = "0: No stable communication"]
    Value1 = 0,
    #[doc = "1: Communication established"]
    Value2 = 1,
}
impl From<ComP3> for bool {
    #[inline(always)]
    fn from(variant: ComP3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COM_P3` reader - Communication on Port 3"]
pub type ComP3R = crate::BitReader<ComP3>;
impl ComP3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ComP3 {
        match self.bits {
            false => ComP3::Value1,
            true => ComP3::Value2,
        }
    }
    #[doc = "No stable communication"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ComP3::Value1
    }
    #[doc = "Communication established"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ComP3::Value2
    }
}
impl R {
    #[doc = "Bit 0 - PDI operational/EEPROM loaded correctly"]
    #[inline(always)]
    pub fn pdi_eeprom(&self) -> PdiEepromR {
        PdiEepromR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDI Watchdog Status"]
    #[inline(always)]
    pub fn pdi_wdt_s(&self) -> PdiWdtSR {
        PdiWdtSR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enhanced Link detection"]
    #[inline(always)]
    pub fn eld(&self) -> EldR {
        EldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Physical link on Port 0"]
    #[inline(always)]
    pub fn link_p0(&self) -> LinkP0R {
        LinkP0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Physical link on Port 1"]
    #[inline(always)]
    pub fn link_p1(&self) -> LinkP1R {
        LinkP1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Physical link on Port 2"]
    #[inline(always)]
    pub fn link_p2(&self) -> LinkP2R {
        LinkP2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Physical link on Port 3"]
    #[inline(always)]
    pub fn link_p3(&self) -> LinkP3R {
        LinkP3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Loop Port 0"]
    #[inline(always)]
    pub fn lp0(&self) -> Lp0R {
        Lp0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Communication on Port 0"]
    #[inline(always)]
    pub fn com_p0(&self) -> ComP0R {
        ComP0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Loop Port 1"]
    #[inline(always)]
    pub fn lp1(&self) -> Lp1R {
        Lp1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Communication on Port 1"]
    #[inline(always)]
    pub fn com_p1(&self) -> ComP1R {
        ComP1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loop Port 2"]
    #[inline(always)]
    pub fn lp2(&self) -> Lp2R {
        Lp2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Communication on Port 2"]
    #[inline(always)]
    pub fn com_p2(&self) -> ComP2R {
        ComP2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Loop Port 3"]
    #[inline(always)]
    pub fn lp3(&self) -> Lp3R {
        Lp3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Communication on Port 3"]
    #[inline(always)]
    pub fn com_p3(&self) -> ComP3R {
        ComP3R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "ESC DL Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_dl_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EscDlStatusSpec;
impl crate::RegisterSpec for EscDlStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`esc_dl_status::R`](R) reader structure"]
impl crate::Readable for EscDlStatusSpec {}
#[doc = "`reset()` method sets ESC_DL_STATUS to value 0x5000"]
impl crate::Resettable for EscDlStatusSpec {
    const RESET_VALUE: u16 = 0x5000;
}
