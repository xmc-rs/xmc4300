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
#[doc = "Field `DevSpd` reader - Device Speed"]
pub type DEV_SPD_R = crate::FieldReader<u8, DEV_SPD_A>;
#[doc = "Device Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEV_SPD_A {
    #[doc = "3: Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    VALUE4 = 3,
}
impl From<DEV_SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: DEV_SPD_A) -> Self {
        variant as _
    }
}
impl DEV_SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEV_SPD_A> {
        match self.bits {
            3 => Some(DEV_SPD_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DEV_SPD_A::VALUE4
    }
}
#[doc = "Field `DevSpd` writer - Device Speed"]
pub type DEV_SPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, DEV_SPD_A, 2, O>;
impl<'a, const O: u8> DEV_SPD_W<'a, O> {
    #[doc = "Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DEV_SPD_A::VALUE4)
    }
}
#[doc = "Field `NZStsOUTHShk` reader - Non-Zero-Length Status OUT Handshake"]
pub type NZSTS_OUTHSHK_R = crate::BitReader<NZSTS_OUTHSHK_A>;
#[doc = "Non-Zero-Length Status OUT Handshake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NZSTS_OUTHSHK_A {
    #[doc = "1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    VALUE1 = 1,
    #[doc = "0: Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    VALUE2 = 0,
}
impl From<NZSTS_OUTHSHK_A> for bool {
    #[inline(always)]
    fn from(variant: NZSTS_OUTHSHK_A) -> Self {
        variant as u8 != 0
    }
}
impl NZSTS_OUTHSHK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NZSTS_OUTHSHK_A {
        match self.bits {
            true => NZSTS_OUTHSHK_A::VALUE1,
            false => NZSTS_OUTHSHK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NZSTS_OUTHSHK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NZSTS_OUTHSHK_A::VALUE2
    }
}
#[doc = "Field `NZStsOUTHShk` writer - Non-Zero-Length Status OUT Handshake"]
pub type NZSTS_OUTHSHK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFG_SPEC, NZSTS_OUTHSHK_A, O>;
impl<'a, const O: u8> NZSTS_OUTHSHK_W<'a, O> {
    #[doc = "Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NZSTS_OUTHSHK_A::VALUE1)
    }
    #[doc = "Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NZSTS_OUTHSHK_A::VALUE2)
    }
}
#[doc = "Field `DevAddr` reader - Device Address"]
pub type DEV_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DevAddr` writer - Device Address"]
pub type DEV_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 7, O>;
#[doc = "Field `PerFrInt` reader - Periodic Frame Interval"]
pub type PER_FR_INT_R = crate::FieldReader<u8, PER_FR_INT_A>;
#[doc = "Periodic Frame Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PER_FR_INT_A {
    #[doc = "0: 80% of the frame interval"]
    VALUE1 = 0,
    #[doc = "1: 85%"]
    VALUE2 = 1,
    #[doc = "2: 90%"]
    VALUE3 = 2,
    #[doc = "3: 95%"]
    VALUE4 = 3,
}
impl From<PER_FR_INT_A> for u8 {
    #[inline(always)]
    fn from(variant: PER_FR_INT_A) -> Self {
        variant as _
    }
}
impl PER_FR_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_FR_INT_A {
        match self.bits {
            0 => PER_FR_INT_A::VALUE1,
            1 => PER_FR_INT_A::VALUE2,
            2 => PER_FR_INT_A::VALUE3,
            3 => PER_FR_INT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PER_FR_INT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PER_FR_INT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PER_FR_INT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PER_FR_INT_A::VALUE4
    }
}
#[doc = "Field `PerFrInt` writer - Periodic Frame Interval"]
pub type PER_FR_INT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCFG_SPEC, u8, PER_FR_INT_A, 2, O>;
impl<'a, const O: u8> PER_FR_INT_W<'a, O> {
    #[doc = "80% of the frame interval"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PER_FR_INT_A::VALUE1)
    }
    #[doc = "85%"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PER_FR_INT_A::VALUE2)
    }
    #[doc = "90%"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PER_FR_INT_A::VALUE3)
    }
    #[doc = "95%"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PER_FR_INT_A::VALUE4)
    }
}
#[doc = "Field `DescDMA` reader - Enable Scatter/Gather DMA in Device mode."]
pub type DESC_DMA_R = crate::BitReader<bool>;
#[doc = "Field `DescDMA` writer - Enable Scatter/Gather DMA in Device mode."]
pub type DESC_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFG_SPEC, bool, O>;
#[doc = "Field `PerSchIntvl` reader - Periodic Scheduling Interval"]
pub type PER_SCH_INTVL_R = crate::FieldReader<u8, PER_SCH_INTVL_A>;
#[doc = "Periodic Scheduling Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PER_SCH_INTVL_A {
    #[doc = "0: 25% of frame."]
    VALUE1 = 0,
    #[doc = "1: 50% of frame."]
    VALUE2 = 1,
    #[doc = "2: 75% of frame."]
    VALUE3 = 2,
}
impl From<PER_SCH_INTVL_A> for u8 {
    #[inline(always)]
    fn from(variant: PER_SCH_INTVL_A) -> Self {
        variant as _
    }
}
impl PER_SCH_INTVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PER_SCH_INTVL_A> {
        match self.bits {
            0 => Some(PER_SCH_INTVL_A::VALUE1),
            1 => Some(PER_SCH_INTVL_A::VALUE2),
            2 => Some(PER_SCH_INTVL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PER_SCH_INTVL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PER_SCH_INTVL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PER_SCH_INTVL_A::VALUE3
    }
}
#[doc = "Field `PerSchIntvl` writer - Periodic Scheduling Interval"]
pub type PER_SCH_INTVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, PER_SCH_INTVL_A, 2, O>;
impl<'a, const O: u8> PER_SCH_INTVL_W<'a, O> {
    #[doc = "25% of frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PER_SCH_INTVL_A::VALUE1)
    }
    #[doc = "50% of frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PER_SCH_INTVL_A::VALUE2)
    }
    #[doc = "75% of frame."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PER_SCH_INTVL_A::VALUE3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn dev_spd(&self) -> DEV_SPD_R {
        DEV_SPD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzsts_outhshk(&self) -> NZSTS_OUTHSHK_R {
        NZSTS_OUTHSHK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn per_fr_int(&self) -> PER_FR_INT_R {
        PER_FR_INT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline(always)]
    pub fn desc_dma(&self) -> DESC_DMA_R {
        DESC_DMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    pub fn per_sch_intvl(&self) -> PER_SCH_INTVL_R {
        PER_SCH_INTVL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    #[must_use]
    pub fn dev_spd(&mut self) -> DEV_SPD_W<0> {
        DEV_SPD_W::new(self)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn nzsts_outhshk(&mut self) -> NZSTS_OUTHSHK_W<2> {
        NZSTS_OUTHSHK_W::new(self)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W<4> {
        DEV_ADDR_W::new(self)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    #[must_use]
    pub fn per_fr_int(&mut self) -> PER_FR_INT_W<11> {
        PER_FR_INT_W::new(self)
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline(always)]
    #[must_use]
    pub fn desc_dma(&mut self) -> DESC_DMA_W<23> {
        DESC_DMA_W::new(self)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    #[must_use]
    pub fn per_sch_intvl(&mut self) -> PER_SCH_INTVL_W<24> {
        PER_SCH_INTVL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCFG to value 0x0820_0000"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0820_0000;
}
