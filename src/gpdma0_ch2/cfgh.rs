#[doc = "Register `CFGH` reader"]
pub struct R(crate::R<CFGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGH` writer"]
pub struct W(crate::W<CFGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGH_SPEC>;
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
impl From<crate::W<CFGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCMODE` reader - Flow Control Mode"]
pub type FCMODE_R = crate::BitReader<FCMODE_A>;
#[doc = "Flow Control Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCMODE_A {
    #[doc = "0: Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    VALUE1 = 0,
    #[doc = "1: Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    VALUE2 = 1,
}
impl From<FCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: FCMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCMODE_A {
        match self.bits {
            false => FCMODE_A::VALUE1,
            true => FCMODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FCMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FCMODE_A::VALUE2
    }
}
#[doc = "Field `FCMODE` writer - Flow Control Mode"]
pub type FCMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGH_SPEC, FCMODE_A, O>;
impl<'a, const O: u8> FCMODE_W<'a, O> {
    #[doc = "Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCMODE_A::VALUE1)
    }
    #[doc = "Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCMODE_A::VALUE2)
    }
}
#[doc = "Field `FIFO_MODE` reader - FIFO Mode Select"]
pub type FIFO_MODE_R = crate::BitReader<FIFO_MODE_A>;
#[doc = "FIFO Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_MODE_A {
    #[doc = "0: Space/data available for single AHB transfer of the specified transfer width."]
    VALUE1 = 0,
    #[doc = "1: Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    VALUE2 = 1,
}
impl From<FIFO_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_MODE_A {
        match self.bits {
            false => FIFO_MODE_A::VALUE1,
            true => FIFO_MODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FIFO_MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FIFO_MODE_A::VALUE2
    }
}
#[doc = "Field `FIFO_MODE` writer - FIFO Mode Select"]
pub type FIFO_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGH_SPEC, FIFO_MODE_A, O>;
impl<'a, const O: u8> FIFO_MODE_W<'a, O> {
    #[doc = "Space/data available for single AHB transfer of the specified transfer width."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FIFO_MODE_A::VALUE1)
    }
    #[doc = "Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FIFO_MODE_A::VALUE2)
    }
}
#[doc = "Field `PROTCTL` reader - Protection Control"]
pub type PROTCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROTCTL` writer - Protection Control"]
pub type PROTCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGH_SPEC, u8, u8, 3, O>;
#[doc = "Field `SRC_PER` reader - Source Peripheral"]
pub type SRC_PER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRC_PER` writer - Source Peripheral"]
pub type SRC_PER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGH_SPEC, u8, u8, 4, O>;
#[doc = "Field `DEST_PER` reader - Destination Peripheral"]
pub type DEST_PER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEST_PER` writer - Destination Peripheral"]
pub type DEST_PER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGH_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline(always)]
    pub fn fcmode(&self) -> FCMODE_R {
        FCMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FIFO_MODE_R {
        FIFO_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    pub fn protctl(&self) -> PROTCTL_R {
        PROTCTL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline(always)]
    pub fn src_per(&self) -> SRC_PER_R {
        SRC_PER_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline(always)]
    pub fn dest_per(&self) -> DEST_PER_R {
        DEST_PER_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fcmode(&mut self) -> FCMODE_W<0> {
        FCMODE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_mode(&mut self) -> FIFO_MODE_W<1> {
        FIFO_MODE_W::new(self)
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    #[must_use]
    pub fn protctl(&mut self) -> PROTCTL_W<2> {
        PROTCTL_W::new(self)
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn src_per(&mut self) -> SRC_PER_W<7> {
        SRC_PER_W::new(self)
    }
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn dest_per(&mut self) -> DEST_PER_W<11> {
        DEST_PER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgh](index.html) module"]
pub struct CFGH_SPEC;
impl crate::RegisterSpec for CFGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgh::R](R) reader structure"]
impl crate::Readable for CFGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgh::W](W) writer structure"]
impl crate::Writable for CFGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGH to value 0x04"]
impl crate::Resettable for CFGH_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
