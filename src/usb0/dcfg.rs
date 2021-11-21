#[doc = "Register `DCFG` reader"]
pub struct R(crate::R<DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFG` writer"]
pub struct W(crate::W<DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFG_SPEC>;
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
impl From<crate::W<DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Device Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEVSPD_A {
    #[doc = "3: Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    VALUE4 = 3,
}
impl From<DEVSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVSPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DevSpd` reader - Device Speed"]
pub struct DEVSPD_R(crate::FieldReader<u8, DEVSPD_A>);
impl DEVSPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEVSPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEVSPD_A> {
        match self.bits {
            3 => Some(DEVSPD_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == DEVSPD_A::VALUE4
    }
}
impl core::ops::Deref for DEVSPD_R {
    type Target = crate::FieldReader<u8, DEVSPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DevSpd` writer - Device Speed"]
pub struct DEVSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVSPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVSPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DEVSPD_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Non-Zero-Length Status OUT Handshake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NZSTSOUTHSHK_A {
    #[doc = "1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    VALUE1 = 1,
    #[doc = "0: Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    VALUE2 = 0,
}
impl From<NZSTSOUTHSHK_A> for bool {
    #[inline(always)]
    fn from(variant: NZSTSOUTHSHK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NZStsOUTHShk` reader - Non-Zero-Length Status OUT Handshake"]
pub struct NZSTSOUTHSHK_R(crate::FieldReader<bool, NZSTSOUTHSHK_A>);
impl NZSTSOUTHSHK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NZSTSOUTHSHK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NZSTSOUTHSHK_A {
        match self.bits {
            true => NZSTSOUTHSHK_A::VALUE1,
            false => NZSTSOUTHSHK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NZSTSOUTHSHK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NZSTSOUTHSHK_A::VALUE2
    }
}
impl core::ops::Deref for NZSTSOUTHSHK_R {
    type Target = crate::FieldReader<bool, NZSTSOUTHSHK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NZStsOUTHShk` writer - Non-Zero-Length Status OUT Handshake"]
pub struct NZSTSOUTHSHK_W<'a> {
    w: &'a mut W,
}
impl<'a> NZSTSOUTHSHK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NZSTSOUTHSHK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NZSTSOUTHSHK_A::VALUE1)
    }
    #[doc = "Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NZSTSOUTHSHK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x7f << 4)) | ((value as u32 & 0x7f) << 4);
        self.w
    }
}
#[doc = "Periodic Frame Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERFRINT_A {
    #[doc = "0: 80% of the frame interval"]
    VALUE1 = 0,
    #[doc = "1: 85%"]
    VALUE2 = 1,
    #[doc = "2: 90%"]
    VALUE3 = 2,
    #[doc = "3: 95%"]
    VALUE4 = 3,
}
impl From<PERFRINT_A> for u8 {
    #[inline(always)]
    fn from(variant: PERFRINT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PerFrInt` reader - Periodic Frame Interval"]
pub struct PERFRINT_R(crate::FieldReader<u8, PERFRINT_A>);
impl PERFRINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERFRINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERFRINT_A {
        match self.bits {
            0 => PERFRINT_A::VALUE1,
            1 => PERFRINT_A::VALUE2,
            2 => PERFRINT_A::VALUE3,
            3 => PERFRINT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PERFRINT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PERFRINT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PERFRINT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PERFRINT_A::VALUE4
    }
}
impl core::ops::Deref for PERFRINT_R {
    type Target = crate::FieldReader<u8, PERFRINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PerFrInt` writer - Periodic Frame Interval"]
pub struct PERFRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFRINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERFRINT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "80% of the frame interval"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PERFRINT_A::VALUE1)
    }
    #[doc = "85%"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PERFRINT_A::VALUE2)
    }
    #[doc = "90%"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PERFRINT_A::VALUE3)
    }
    #[doc = "95%"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PERFRINT_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `DescDMA` reader - Enable Scatter/Gather DMA in Device mode."]
pub struct DESCDMA_R(crate::FieldReader<bool, bool>);
impl DESCDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DESCDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESCDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DescDMA` writer - Enable Scatter/Gather DMA in Device mode."]
pub struct DESCDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Periodic Scheduling Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERSCHINTVL_A {
    #[doc = "0: 25% of frame."]
    VALUE1 = 0,
    #[doc = "1: 50% of frame."]
    VALUE2 = 1,
    #[doc = "2: 75% of frame."]
    VALUE3 = 2,
}
impl From<PERSCHINTVL_A> for u8 {
    #[inline(always)]
    fn from(variant: PERSCHINTVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PerSchIntvl` reader - Periodic Scheduling Interval"]
pub struct PERSCHINTVL_R(crate::FieldReader<u8, PERSCHINTVL_A>);
impl PERSCHINTVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERSCHINTVL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERSCHINTVL_A> {
        match self.bits {
            0 => Some(PERSCHINTVL_A::VALUE1),
            1 => Some(PERSCHINTVL_A::VALUE2),
            2 => Some(PERSCHINTVL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PERSCHINTVL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PERSCHINTVL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PERSCHINTVL_A::VALUE3
    }
}
impl core::ops::Deref for PERSCHINTVL_R {
    type Target = crate::FieldReader<u8, PERSCHINTVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PerSchIntvl` writer - Periodic Scheduling Interval"]
pub struct PERSCHINTVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSCHINTVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERSCHINTVL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "25% of frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PERSCHINTVL_A::VALUE1)
    }
    #[doc = "50% of frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PERSCHINTVL_A::VALUE2)
    }
    #[doc = "75% of frame."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PERSCHINTVL_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn dev_spd(&self) -> DEVSPD_R {
        DEVSPD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzsts_outhshk(&self) -> NZSTSOUTHSHK_R {
        NZSTSOUTHSHK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn per_fr_int(&self) -> PERFRINT_R {
        PERFRINT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline(always)]
    pub fn desc_dma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    pub fn per_sch_intvl(&self) -> PERSCHINTVL_R {
        PERSCHINTVL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn dev_spd(&mut self) -> DEVSPD_W {
        DEVSPD_W { w: self }
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzsts_outhshk(&mut self) -> NZSTSOUTHSHK_W {
        NZSTSOUTHSHK_W { w: self }
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn per_fr_int(&mut self) -> PERFRINT_W {
        PERFRINT_W { w: self }
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline(always)]
    pub fn desc_dma(&mut self) -> DESCDMA_W {
        DESCDMA_W { w: self }
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    pub fn per_sch_intvl(&mut self) -> PERSCHINTVL_W {
        PERSCHINTVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](index.html) module"]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfg::R](R) reader structure"]
impl crate::Readable for DCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfg::W](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCFG to value 0x0820_0000"]
impl crate::Resettable for DCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0820_0000
    }
}
