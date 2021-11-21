#[doc = "Register `INTERRUPT_ENABLE` reader"]
pub struct R(crate::R<INTERRUPT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT_ENABLE` writer"]
pub struct W(crate::W<INTERRUPT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_ENABLE_SPEC>;
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
impl From<crate::W<INTERRUPT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub struct TIE_R(crate::FieldReader<bool, bool>);
impl TIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TSE` reader - Transmit Stopped Enable"]
pub struct TSE_R(crate::FieldReader<bool, bool>);
impl TSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSE` writer - Transmit Stopped Enable"]
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TUE` reader - Transmit Buffer Unvailable Enable"]
pub struct TUE_R(crate::FieldReader<bool, bool>);
impl TUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUE` writer - Transmit Buffer Unvailable Enable"]
pub struct TUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TUE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TJE` reader - Transmit Jabber Timeout Enable"]
pub struct TJE_R(crate::FieldReader<bool, bool>);
impl TJE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TJE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TJE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TJE` writer - Transmit Jabber Timeout Enable"]
pub struct TJE_W<'a> {
    w: &'a mut W,
}
impl<'a> TJE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `OVE` reader - Overflow Interrupt Enable"]
pub struct OVE_R(crate::FieldReader<bool, bool>);
impl OVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVE` writer - Overflow Interrupt Enable"]
pub struct OVE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `UNE` reader - Underflow Interrupt Enable"]
pub struct UNE_R(crate::FieldReader<bool, bool>);
impl UNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNE` writer - Underflow Interrupt Enable"]
pub struct UNE_W<'a> {
    w: &'a mut W,
}
impl<'a> UNE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub struct RIE_R(crate::FieldReader<bool, bool>);
impl RIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub struct RIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RUE` reader - Receive Buffer Unavailable Enable"]
pub struct RUE_R(crate::FieldReader<bool, bool>);
impl RUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUE` writer - Receive Buffer Unavailable Enable"]
pub struct RUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RUE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RSE` reader - Receive Stopped Enable"]
pub struct RSE_R(crate::FieldReader<bool, bool>);
impl RSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSE` writer - Receive Stopped Enable"]
pub struct RSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSE_W<'a> {
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
#[doc = "Field `RWE` reader - Receive Watchdog Timeout Enable"]
pub struct RWE_R(crate::FieldReader<bool, bool>);
impl RWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWE` writer - Receive Watchdog Timeout Enable"]
pub struct RWE_W<'a> {
    w: &'a mut W,
}
impl<'a> RWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ETE` reader - Early Transmit Interrupt Enable"]
pub struct ETE_R(crate::FieldReader<bool, bool>);
impl ETE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETE` writer - Early Transmit Interrupt Enable"]
pub struct ETE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETE_W<'a> {
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
#[doc = "Field `FBE` reader - Fatal Bus Error Enable"]
pub struct FBE_R(crate::FieldReader<bool, bool>);
impl FBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBE` writer - Fatal Bus Error Enable"]
pub struct FBE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ERE` reader - Early Receive Interrupt Enable"]
pub struct ERE_R(crate::FieldReader<bool, bool>);
impl ERE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERE` writer - Early Receive Interrupt Enable"]
pub struct ERE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable"]
pub struct AIE_R(crate::FieldReader<bool, bool>);
impl AIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable"]
pub struct AIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AIE_W<'a> {
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
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable"]
pub struct NIE_R(crate::FieldReader<bool, bool>);
impl NIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable"]
pub struct NIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unvailable Enable"]
    #[inline(always)]
    pub fn tue(&self) -> TUE_R {
        TUE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    pub fn tje(&self) -> TJE_R {
        TJE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ove(&self) -> OVE_R {
        OVE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn une(&self) -> UNE_R {
        UNE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwe(&self) -> RWE_R {
        RWE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn ete(&self) -> ETE_R {
        ETE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ere(&self) -> ERE_R {
        ERE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    #[doc = "Bit 2 - Transmit Buffer Unvailable Enable"]
    #[inline(always)]
    pub fn tue(&mut self) -> TUE_W {
        TUE_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    pub fn tje(&mut self) -> TJE_W {
        TJE_W { w: self }
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ove(&mut self) -> OVE_W {
        OVE_W { w: self }
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn une(&mut self) -> UNE_W {
        UNE_W { w: self }
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W { w: self }
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W {
        RUE_W { w: self }
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&mut self) -> RSE_W {
        RSE_W { w: self }
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwe(&mut self) -> RWE_W {
        RWE_W { w: self }
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn ete(&mut self) -> ETE_W {
        ETE_W { w: self }
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W {
        FBE_W { w: self }
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ere(&mut self) -> ERE_W {
        ERE_W { w: self }
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&mut self) -> AIE_W {
        AIE_W { w: self }
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&mut self) -> NIE_W {
        NIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_enable](index.html) module"]
pub struct INTERRUPT_ENABLE_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_enable::R](R) reader structure"]
impl crate::Readable for INTERRUPT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_enable::W](W) writer structure"]
impl crate::Writable for INTERRUPT_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERRUPT_ENABLE to value 0"]
impl crate::Resettable for INTERRUPT_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
