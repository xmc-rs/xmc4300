#[doc = "Register `HCCHAR` reader"]
pub struct R(crate::R<HCCHAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCHAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCHAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCHAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCHAR` writer"]
pub struct W(crate::W<HCCHAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCHAR_SPEC>;
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
impl From<crate::W<HCCHAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCHAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub struct MPS_R(crate::FieldReader<u16, u16>);
impl MPS_R {
    pub(crate) fn new(bits: u16) -> Self {
        MPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub struct MPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `EPNum` reader - Endpoint Number"]
pub struct EPNUM_R(crate::FieldReader<u8, u8>);
impl EPNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNum` writer - Endpoint Number"]
pub struct EPNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Endpoint Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDIR_A {
    #[doc = "0: OUT"]
    VALUE1 = 0,
    #[doc = "1: IN"]
    VALUE2 = 1,
}
impl From<EPDIR_A> for bool {
    #[inline(always)]
    fn from(variant: EPDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPDir` reader - Endpoint Direction"]
pub struct EPDIR_R(crate::FieldReader<bool, EPDIR_A>);
impl EPDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPDIR_A {
        match self.bits {
            false => EPDIR_A::VALUE1,
            true => EPDIR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EPDIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EPDIR_A::VALUE2
    }
}
impl core::ops::Deref for EPDIR_R {
    type Target = crate::FieldReader<bool, EPDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDir` writer - Endpoint Direction"]
pub struct EPDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPDIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPDIR_A::VALUE1)
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPDIR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control"]
    VALUE1 = 0,
    #[doc = "1: Isochronous"]
    VALUE2 = 1,
    #[doc = "2: Bulk"]
    VALUE3 = 2,
    #[doc = "3: Interrupt"]
    VALUE4 = 3,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPType` reader - Endpoint Type"]
pub struct EPTYPE_R(crate::FieldReader<u8, EPTYPE_A>);
impl EPTYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPTYPE_A {
        match self.bits {
            0 => EPTYPE_A::VALUE1,
            1 => EPTYPE_A::VALUE2,
            2 => EPTYPE_A::VALUE3,
            3 => EPTYPE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EPTYPE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EPTYPE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == EPTYPE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == EPTYPE_A::VALUE4
    }
}
impl core::ops::Deref for EPTYPE_R {
    type Target = crate::FieldReader<u8, EPTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPType` writer - Endpoint Type"]
pub struct EPTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPTYPE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPTYPE_A::VALUE1)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPTYPE_A::VALUE2)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EPTYPE_A::VALUE3)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EPTYPE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Multi Count / Error Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MC_EC_A {
    #[doc = "1: 1 transaction"]
    VALUE2 = 1,
    #[doc = "2: 2 transactions to be issued for this endpoint per frame"]
    VALUE3 = 2,
    #[doc = "3: 3 transactions to be issued for this endpoint per frame"]
    VALUE4 = 3,
}
impl From<MC_EC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_EC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MC_EC` reader - Multi Count / Error Count"]
pub struct MC_EC_R(crate::FieldReader<u8, MC_EC_A>);
impl MC_EC_R {
    pub(crate) fn new(bits: u8) -> Self {
        MC_EC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MC_EC_A> {
        match self.bits {
            1 => Some(MC_EC_A::VALUE2),
            2 => Some(MC_EC_A::VALUE3),
            3 => Some(MC_EC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MC_EC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == MC_EC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == MC_EC_A::VALUE4
    }
}
impl core::ops::Deref for MC_EC_R {
    type Target = crate::FieldReader<u8, MC_EC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MC_EC` writer - Multi Count / Error Count"]
pub struct MC_EC_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_EC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MC_EC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 transaction"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MC_EC_A::VALUE2)
    }
    #[doc = "2 transactions to be issued for this endpoint per frame"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MC_EC_A::VALUE3)
    }
    #[doc = "3 transactions to be issued for this endpoint per frame"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MC_EC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `DevAddr` reader - Device Address"]
pub struct DEVADDR_R(crate::FieldReader<u8, u8>);
impl DEVADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEVADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DevAddr` writer - Device Address"]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 22)) | ((value as u32 & 0x7f) << 22);
        self.w
    }
}
#[doc = "Odd Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODDFRM_A {
    #[doc = "0: Even frame"]
    VALUE1 = 0,
    #[doc = "1: Odd frame"]
    VALUE2 = 1,
}
impl From<ODDFRM_A> for bool {
    #[inline(always)]
    fn from(variant: ODDFRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OddFrm` reader - Odd Frame"]
pub struct ODDFRM_R(crate::FieldReader<bool, ODDFRM_A>);
impl ODDFRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODDFRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODDFRM_A {
        match self.bits {
            false => ODDFRM_A::VALUE1,
            true => ODDFRM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ODDFRM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ODDFRM_A::VALUE2
    }
}
impl core::ops::Deref for ODDFRM_R {
    type Target = crate::FieldReader<bool, ODDFRM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OddFrm` writer - Odd Frame"]
pub struct ODDFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> ODDFRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODDFRM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Even frame"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ODDFRM_A::VALUE1)
    }
    #[doc = "Odd frame"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ODDFRM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `ChDis` reader - Channel Disable"]
pub struct CHDIS_R(crate::FieldReader<bool, bool>);
impl CHDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ChDis` writer - Channel Disable"]
pub struct CHDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDIS_W<'a> {
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
#[doc = "Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHENA_A {
    #[doc = "0: Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    VALUE1 = 0,
    #[doc = "1: Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    VALUE2 = 1,
}
impl From<CHENA_A> for bool {
    #[inline(always)]
    fn from(variant: CHENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ChEna` reader - Channel Enable"]
pub struct CHENA_R(crate::FieldReader<bool, CHENA_A>);
impl CHENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHENA_A {
        match self.bits {
            false => CHENA_A::VALUE1,
            true => CHENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CHENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CHENA_A::VALUE2
    }
}
impl core::ops::Deref for CHENA_R {
    type Target = crate::FieldReader<bool, CHENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ChEna` writer - Channel Enable"]
pub struct CHENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CHENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure is not yet ready. Scatter/Gather mode disabled: Channel disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHENA_A::VALUE1)
    }
    #[doc = "Scatter/Gather mode enabled: Indicates that the descriptor structure and data buffer with data is setup and this channel can access the descriptor. Scatter/Gather mode disabled: Channel enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHENA_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Multi Count / Error Count"]
    #[inline(always)]
    pub fn mc_ec(&self) -> MC_EC_R {
        MC_EC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn odd_frm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn ch_dis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn ch_ena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W {
        MPS_W { w: self }
    }
    #[doc = "Bits 11:14 - Endpoint Number"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W {
        EPNUM_W { w: self }
    }
    #[doc = "Bit 15 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W {
        EPDIR_W { w: self }
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W {
        EPTYPE_W { w: self }
    }
    #[doc = "Bits 20:21 - Multi Count / Error Count"]
    #[inline(always)]
    pub fn mc_ec(&mut self) -> MC_EC_W {
        MC_EC_W { w: self }
    }
    #[doc = "Bits 22:28 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Bit 29 - Odd Frame"]
    #[inline(always)]
    pub fn odd_frm(&mut self) -> ODDFRM_W {
        ODDFRM_W { w: self }
    }
    #[doc = "Bit 30 - Channel Disable"]
    #[inline(always)]
    pub fn ch_dis(&mut self) -> CHDIS_W {
        CHDIS_W { w: self }
    }
    #[doc = "Bit 31 - Channel Enable"]
    #[inline(always)]
    pub fn ch_ena(&mut self) -> CHENA_W {
        CHENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Channel Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar](index.html) module"]
pub struct HCCHAR_SPEC;
impl crate::RegisterSpec for HCCHAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcchar::R](R) reader structure"]
impl crate::Readable for HCCHAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcchar::W](W) writer structure"]
impl crate::Writable for HCCHAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCCHAR to value 0"]
impl crate::Resettable for HCCHAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
