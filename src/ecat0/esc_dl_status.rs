#[doc = "Register `ESC_DL_STATUS` reader"]
pub struct R(crate::R<ESC_DL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESC_DL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESC_DL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESC_DL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `PDI_EEPROM` reader - PDI operational/EEPROM loaded correctly"]
pub struct PDI_EEPROM_R(crate::FieldReader<bool, PDI_EEPROM_A>);
impl PDI_EEPROM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDI_EEPROM_R(crate::FieldReader::new(bits))
    }
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
        **self == PDI_EEPROM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PDI_EEPROM_A::VALUE2
    }
}
impl core::ops::Deref for PDI_EEPROM_R {
    type Target = crate::FieldReader<bool, PDI_EEPROM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `PDI_WDT_S` reader - PDI Watchdog Status"]
pub struct PDI_WDT_S_R(crate::FieldReader<bool, PDI_WDT_S_A>);
impl PDI_WDT_S_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDI_WDT_S_R(crate::FieldReader::new(bits))
    }
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
        **self == PDI_WDT_S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PDI_WDT_S_A::VALUE2
    }
}
impl core::ops::Deref for PDI_WDT_S_R {
    type Target = crate::FieldReader<bool, PDI_WDT_S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `ELD` reader - Enhanced Link detection"]
pub struct ELD_R(crate::FieldReader<bool, ELD_A>);
impl ELD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELD_R(crate::FieldReader::new(bits))
    }
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
        **self == ELD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ELD_A::VALUE2
    }
}
impl core::ops::Deref for ELD_R {
    type Target = crate::FieldReader<bool, ELD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LINK_P0` reader - Physical link on Port 0"]
pub struct LINK_P0_R(crate::FieldReader<bool, LINK_P0_A>);
impl LINK_P0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINK_P0_R(crate::FieldReader::new(bits))
    }
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
        **self == LINK_P0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LINK_P0_A::VALUE2
    }
}
impl core::ops::Deref for LINK_P0_R {
    type Target = crate::FieldReader<bool, LINK_P0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LINK_P1` reader - Physical link on Port 1"]
pub struct LINK_P1_R(crate::FieldReader<bool, LINK_P1_A>);
impl LINK_P1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINK_P1_R(crate::FieldReader::new(bits))
    }
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
        **self == LINK_P1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LINK_P1_A::VALUE2
    }
}
impl core::ops::Deref for LINK_P1_R {
    type Target = crate::FieldReader<bool, LINK_P1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LINK_P2` reader - Physical link on Port 2"]
pub struct LINK_P2_R(crate::FieldReader<bool, LINK_P2_A>);
impl LINK_P2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINK_P2_R(crate::FieldReader::new(bits))
    }
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
        **self == LINK_P2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LINK_P2_A::VALUE2
    }
}
impl core::ops::Deref for LINK_P2_R {
    type Target = crate::FieldReader<bool, LINK_P2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LINK_P3` reader - Physical link on Port 3"]
pub struct LINK_P3_R(crate::FieldReader<bool, LINK_P3_A>);
impl LINK_P3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINK_P3_R(crate::FieldReader::new(bits))
    }
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
        **self == LINK_P3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LINK_P3_A::VALUE2
    }
}
impl core::ops::Deref for LINK_P3_R {
    type Target = crate::FieldReader<bool, LINK_P3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LP0` reader - Loop Port 0"]
pub struct LP0_R(crate::FieldReader<bool, LP0_A>);
impl LP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LP0_R(crate::FieldReader::new(bits))
    }
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
        **self == LP0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LP0_A::VALUE2
    }
}
impl core::ops::Deref for LP0_R {
    type Target = crate::FieldReader<bool, LP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `COM_P0` reader - Communication on Port 0"]
pub struct COM_P0_R(crate::FieldReader<bool, COM_P0_A>);
impl COM_P0_R {
    pub(crate) fn new(bits: bool) -> Self {
        COM_P0_R(crate::FieldReader::new(bits))
    }
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
        **self == COM_P0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == COM_P0_A::VALUE2
    }
}
impl core::ops::Deref for COM_P0_R {
    type Target = crate::FieldReader<bool, COM_P0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LP1` reader - Loop Port 1"]
pub struct LP1_R(crate::FieldReader<bool, LP1_A>);
impl LP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LP1_R(crate::FieldReader::new(bits))
    }
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
        **self == LP1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LP1_A::VALUE2
    }
}
impl core::ops::Deref for LP1_R {
    type Target = crate::FieldReader<bool, LP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `COM_P1` reader - Communication on Port 1"]
pub struct COM_P1_R(crate::FieldReader<bool, COM_P1_A>);
impl COM_P1_R {
    pub(crate) fn new(bits: bool) -> Self {
        COM_P1_R(crate::FieldReader::new(bits))
    }
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
        **self == COM_P1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == COM_P1_A::VALUE2
    }
}
impl core::ops::Deref for COM_P1_R {
    type Target = crate::FieldReader<bool, COM_P1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LP2` reader - Loop Port 2"]
pub struct LP2_R(crate::FieldReader<bool, LP2_A>);
impl LP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LP2_R(crate::FieldReader::new(bits))
    }
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
        **self == LP2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LP2_A::VALUE2
    }
}
impl core::ops::Deref for LP2_R {
    type Target = crate::FieldReader<bool, LP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `COM_P2` reader - Communication on Port 2"]
pub struct COM_P2_R(crate::FieldReader<bool, COM_P2_A>);
impl COM_P2_R {
    pub(crate) fn new(bits: bool) -> Self {
        COM_P2_R(crate::FieldReader::new(bits))
    }
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
        **self == COM_P2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == COM_P2_A::VALUE2
    }
}
impl core::ops::Deref for COM_P2_R {
    type Target = crate::FieldReader<bool, COM_P2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LP3` reader - Loop Port 3"]
pub struct LP3_R(crate::FieldReader<bool, LP3_A>);
impl LP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LP3_R(crate::FieldReader::new(bits))
    }
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
        **self == LP3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LP3_A::VALUE2
    }
}
impl core::ops::Deref for LP3_R {
    type Target = crate::FieldReader<bool, LP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `COM_P3` reader - Communication on Port 3"]
pub struct COM_P3_R(crate::FieldReader<bool, COM_P3_A>);
impl COM_P3_R {
    pub(crate) fn new(bits: bool) -> Self {
        COM_P3_R(crate::FieldReader::new(bits))
    }
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
        **self == COM_P3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == COM_P3_A::VALUE2
    }
}
impl core::ops::Deref for COM_P3_R {
    type Target = crate::FieldReader<bool, COM_P3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "ESC DL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esc_dl_status](index.html) module"]
pub struct ESC_DL_STATUS_SPEC;
impl crate::RegisterSpec for ESC_DL_STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [esc_dl_status::R](R) reader structure"]
impl crate::Readable for ESC_DL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ESC_DL_STATUS to value 0x5000"]
impl crate::Resettable for ESC_DL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5000
    }
}
