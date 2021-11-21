#[doc = "Register `CFGL` reader"]
pub struct R(crate::R<CFGL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGL` writer"]
pub struct W(crate::W<CFGL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFGL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELOAD_DST` reader - Automatic Destination Reload"]
pub struct RELOAD_DST_R(crate::FieldReader<bool, bool>);
impl RELOAD_DST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RELOAD_DST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELOAD_DST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELOAD_DST` writer - Automatic Destination Reload"]
pub struct RELOAD_DST_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_DST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `RELOAD_SRC` reader - Automatic Source Reload"]
pub struct RELOAD_SRC_R(crate::FieldReader<bool, bool>);
impl RELOAD_SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RELOAD_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELOAD_SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELOAD_SRC` writer - Automatic Source Reload"]
pub struct RELOAD_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_SRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `MAX_ABRST` reader - Maximum AMBA Burst Length"]
pub struct MAX_ABRST_R(crate::FieldReader<u16, u16>);
impl MAX_ABRST_R {
    pub(crate) fn new(bits: u16) -> Self {
        MAX_ABRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_ABRST_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_ABRST` writer - Maximum AMBA Burst Length"]
pub struct MAX_ABRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_ABRST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Source Handshaking Interface Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_HS_POL_A {
    #[doc = "0: Active high"]
    VALUE1 = 0,
    #[doc = "1: Active low"]
    VALUE2 = 1,
}
impl From<SRC_HS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_HS_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC_HS_POL` reader - Source Handshaking Interface Polarity"]
pub struct SRC_HS_POL_R(crate::FieldReader<bool, SRC_HS_POL_A>);
impl SRC_HS_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC_HS_POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_HS_POL_A {
        match self.bits {
            false => SRC_HS_POL_A::VALUE1,
            true => SRC_HS_POL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRC_HS_POL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SRC_HS_POL_A::VALUE2
    }
}
impl core::ops::Deref for SRC_HS_POL_R {
    type Target = crate::FieldReader<bool, SRC_HS_POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_HS_POL` writer - Source Handshaking Interface Polarity"]
pub struct SRC_HS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_HS_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_HS_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRC_HS_POL_A::VALUE1)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRC_HS_POL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Destination Handshaking Interface Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_HS_POL_A {
    #[doc = "0: Active high"]
    VALUE1 = 0,
    #[doc = "1: Active low"]
    VALUE2 = 1,
}
impl From<DST_HS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: DST_HS_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DST_HS_POL` reader - Destination Handshaking Interface Polarity"]
pub struct DST_HS_POL_R(crate::FieldReader<bool, DST_HS_POL_A>);
impl DST_HS_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DST_HS_POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_HS_POL_A {
        match self.bits {
            false => DST_HS_POL_A::VALUE1,
            true => DST_HS_POL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DST_HS_POL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DST_HS_POL_A::VALUE2
    }
}
impl core::ops::Deref for DST_HS_POL_R {
    type Target = crate::FieldReader<bool, DST_HS_POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_HS_POL` writer - Destination Handshaking Interface Polarity"]
pub struct DST_HS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_HS_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_HS_POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DST_HS_POL_A::VALUE1)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DST_HS_POL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `LOCK_B` reader - Bus Lock Bit"]
pub struct LOCK_B_R(crate::FieldReader<bool, bool>);
impl LOCK_B_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_B` writer - Bus Lock Bit"]
pub struct LOCK_B_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_B_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `LOCK_CH` reader - Channel Lock Bit"]
pub struct LOCK_CH_R(crate::FieldReader<bool, bool>);
impl LOCK_CH_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_CH` writer - Channel Lock Bit"]
pub struct LOCK_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_CH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Bus Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_B_L_A {
    #[doc = "0: Over complete DMA transfer"]
    VALUE1 = 0,
    #[doc = "1: Over complete DMA block transfer"]
    VALUE2 = 1,
    #[doc = "2: Over complete DMA transaction"]
    VALUE3 = 2,
}
impl From<LOCK_B_L_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_B_L_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOCK_B_L` reader - Bus Lock Level"]
pub struct LOCK_B_L_R(crate::FieldReader<u8, LOCK_B_L_A>);
impl LOCK_B_L_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOCK_B_L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_B_L_A> {
        match self.bits {
            0 => Some(LOCK_B_L_A::VALUE1),
            1 => Some(LOCK_B_L_A::VALUE2),
            2 => Some(LOCK_B_L_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LOCK_B_L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LOCK_B_L_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == LOCK_B_L_A::VALUE3
    }
}
impl core::ops::Deref for LOCK_B_L_R {
    type Target = crate::FieldReader<u8, LOCK_B_L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_B_L` writer - Bus Lock Level"]
pub struct LOCK_B_L_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_B_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_B_L_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCK_B_L_A::VALUE1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCK_B_L_A::VALUE2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LOCK_B_L_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Channel Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_CH_L_A {
    #[doc = "0: Over complete DMA transfer"]
    VALUE1 = 0,
    #[doc = "1: Over complete DMA block transfer"]
    VALUE2 = 1,
    #[doc = "2: Over complete DMA transaction"]
    VALUE3 = 2,
}
impl From<LOCK_CH_L_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_CH_L_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOCK_CH_L` reader - Channel Lock Level"]
pub struct LOCK_CH_L_R(crate::FieldReader<u8, LOCK_CH_L_A>);
impl LOCK_CH_L_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOCK_CH_L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_CH_L_A> {
        match self.bits {
            0 => Some(LOCK_CH_L_A::VALUE1),
            1 => Some(LOCK_CH_L_A::VALUE2),
            2 => Some(LOCK_CH_L_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LOCK_CH_L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LOCK_CH_L_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == LOCK_CH_L_A::VALUE3
    }
}
impl core::ops::Deref for LOCK_CH_L_R {
    type Target = crate::FieldReader<u8, LOCK_CH_L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_CH_L` writer - Channel Lock Level"]
pub struct LOCK_CH_L_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_CH_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_CH_L_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::VALUE1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::VALUE2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Source Software or Hardware Handshaking Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_SEL_SRC_A {
    #[doc = "0: Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1 = 0,
    #[doc = "1: Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    VALUE2 = 1,
}
impl From<HS_SEL_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: HS_SEL_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_SEL_SRC` reader - Source Software or Hardware Handshaking Select"]
pub struct HS_SEL_SRC_R(crate::FieldReader<bool, HS_SEL_SRC_A>);
impl HS_SEL_SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HS_SEL_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_SEL_SRC_A {
        match self.bits {
            false => HS_SEL_SRC_A::VALUE1,
            true => HS_SEL_SRC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HS_SEL_SRC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HS_SEL_SRC_A::VALUE2
    }
}
impl core::ops::Deref for HS_SEL_SRC_R {
    type Target = crate::FieldReader<bool, HS_SEL_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS_SEL_SRC` writer - Source Software or Hardware Handshaking Select"]
pub struct HS_SEL_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_SEL_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_SEL_SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HS_SEL_SRC_A::VALUE1)
    }
    #[doc = "Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HS_SEL_SRC_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Destination Software or Hardware Handshaking Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_SEL_DST_A {
    #[doc = "0: Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1 = 0,
    #[doc = "1: Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    VALUE2 = 1,
}
impl From<HS_SEL_DST_A> for bool {
    #[inline(always)]
    fn from(variant: HS_SEL_DST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_SEL_DST` reader - Destination Software or Hardware Handshaking Select"]
pub struct HS_SEL_DST_R(crate::FieldReader<bool, HS_SEL_DST_A>);
impl HS_SEL_DST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HS_SEL_DST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_SEL_DST_A {
        match self.bits {
            false => HS_SEL_DST_A::VALUE1,
            true => HS_SEL_DST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HS_SEL_DST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HS_SEL_DST_A::VALUE2
    }
}
impl core::ops::Deref for HS_SEL_DST_R {
    type Target = crate::FieldReader<bool, HS_SEL_DST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS_SEL_DST` writer - Destination Software or Hardware Handshaking Select"]
pub struct HS_SEL_DST_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_SEL_DST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_SEL_DST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HS_SEL_DST_A::VALUE1)
    }
    #[doc = "Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HS_SEL_DST_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Indicates if there is data left in the channel FIFO\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_EMPTY_A {
    #[doc = "1: Channel FIFO empty"]
    VALUE1 = 1,
    #[doc = "0: Channel FIFO not empty"]
    VALUE2 = 0,
}
impl From<FIFO_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_EMPTY` reader - Indicates if there is data left in the channel FIFO"]
pub struct FIFO_EMPTY_R(crate::FieldReader<bool, FIFO_EMPTY_A>);
impl FIFO_EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_EMPTY_A {
        match self.bits {
            true => FIFO_EMPTY_A::VALUE1,
            false => FIFO_EMPTY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FIFO_EMPTY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FIFO_EMPTY_A::VALUE2
    }
}
impl core::ops::Deref for FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, FIFO_EMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_SUSP_A {
    #[doc = "0: Not suspended."]
    VALUE1 = 0,
    #[doc = "1: Suspend DMA transfer from the source."]
    VALUE2 = 1,
}
impl From<CH_SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: CH_SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH_SUSP` reader - Channel Suspend"]
pub struct CH_SUSP_R(crate::FieldReader<bool, CH_SUSP_A>);
impl CH_SUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH_SUSP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_SUSP_A {
        match self.bits {
            false => CH_SUSP_A::VALUE1,
            true => CH_SUSP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CH_SUSP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CH_SUSP_A::VALUE2
    }
}
impl core::ops::Deref for CH_SUSP_R {
    type Target = crate::FieldReader<bool, CH_SUSP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH_SUSP` writer - Channel Suspend"]
pub struct CH_SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_SUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH_SUSP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not suspended."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH_SUSP_A::VALUE1)
    }
    #[doc = "Suspend DMA transfer from the source."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH_SUSP_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CH_PRIOR` reader - Channel priority"]
pub struct CH_PRIOR_R(crate::FieldReader<u8, u8>);
impl CH_PRIOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH_PRIOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_PRIOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH_PRIOR` writer - Channel priority"]
pub struct CH_PRIOR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_PRIOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Automatic Destination Reload"]
    #[inline(always)]
    pub fn reload_dst(&self) -> RELOAD_DST_R {
        RELOAD_DST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Automatic Source Reload"]
    #[inline(always)]
    pub fn reload_src(&self) -> RELOAD_SRC_R {
        RELOAD_SRC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline(always)]
    pub fn max_abrst(&self) -> MAX_ABRST_R {
        MAX_ABRST_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&self) -> SRC_HS_POL_R {
        SRC_HS_POL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&self) -> DST_HS_POL_R {
        DST_HS_POL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline(always)]
    pub fn lock_b(&self) -> LOCK_B_R {
        LOCK_B_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline(always)]
    pub fn lock_ch(&self) -> LOCK_CH_R {
        LOCK_CH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline(always)]
    pub fn lock_b_l(&self) -> LOCK_B_L_R {
        LOCK_B_L_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline(always)]
    pub fn lock_ch_l(&self) -> LOCK_CH_L_R {
        LOCK_CH_L_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_src(&self) -> HS_SEL_SRC_R {
        HS_SEL_SRC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_dst(&self) -> HS_SEL_DST_R {
        HS_SEL_DST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates if there is data left in the channel FIFO"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    pub fn ch_susp(&self) -> CH_SUSP_R {
        CH_SUSP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline(always)]
    pub fn ch_prior(&self) -> CH_PRIOR_R {
        CH_PRIOR_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Automatic Destination Reload"]
    #[inline(always)]
    pub fn reload_dst(&mut self) -> RELOAD_DST_W {
        RELOAD_DST_W { w: self }
    }
    #[doc = "Bit 30 - Automatic Source Reload"]
    #[inline(always)]
    pub fn reload_src(&mut self) -> RELOAD_SRC_W {
        RELOAD_SRC_W { w: self }
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline(always)]
    pub fn max_abrst(&mut self) -> MAX_ABRST_W {
        MAX_ABRST_W { w: self }
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&mut self) -> SRC_HS_POL_W {
        SRC_HS_POL_W { w: self }
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&mut self) -> DST_HS_POL_W {
        DST_HS_POL_W { w: self }
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline(always)]
    pub fn lock_b(&mut self) -> LOCK_B_W {
        LOCK_B_W { w: self }
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline(always)]
    pub fn lock_ch(&mut self) -> LOCK_CH_W {
        LOCK_CH_W { w: self }
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline(always)]
    pub fn lock_b_l(&mut self) -> LOCK_B_L_W {
        LOCK_B_L_W { w: self }
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline(always)]
    pub fn lock_ch_l(&mut self) -> LOCK_CH_L_W {
        LOCK_CH_L_W { w: self }
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_src(&mut self) -> HS_SEL_SRC_W {
        HS_SEL_SRC_W { w: self }
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_dst(&mut self) -> HS_SEL_DST_W {
        HS_SEL_DST_W { w: self }
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    pub fn ch_susp(&mut self) -> CH_SUSP_W {
        CH_SUSP_W { w: self }
    }
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline(always)]
    pub fn ch_prior(&mut self) -> CH_PRIOR_W {
        CH_PRIOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgl](index.html) module"]
pub struct CFGL_SPEC;
impl crate::RegisterSpec for CFGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgl::R](R) reader structure"]
impl crate::Readable for CFGL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgl::W](W) writer structure"]
impl crate::Writable for CFGL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGL to value 0x0e00"]
impl crate::Resettable for CFGL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e00
    }
}
