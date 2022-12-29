#[doc = "Register `NPCR` reader"]
pub struct R(crate::R<NPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NPCR` writer"]
pub struct W(crate::W<NPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NPCR_SPEC>;
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
impl From<crate::W<NPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXSEL` reader - Receive Select"]
pub type RXSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXSEL` writer - Receive Select"]
pub type RXSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NPCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `LBM` reader - Loop-Back Mode"]
pub type LBM_R = crate::BitReader<LBM_A>;
#[doc = "Loop-Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBM_A {
    #[doc = "0: Loop-Back Mode is disabled."]
    VALUE1 = 0,
    #[doc = "1: Loop-Back Mode is enabled. This node is connected to an internal (virtual) loop-back CAN bus. All CAN nodes which are in Loop-Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop-Back Mode."]
    VALUE2 = 1,
}
impl From<LBM_A> for bool {
    #[inline(always)]
    fn from(variant: LBM_A) -> Self {
        variant as u8 != 0
    }
}
impl LBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBM_A {
        match self.bits {
            false => LBM_A::VALUE1,
            true => LBM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LBM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LBM_A::VALUE2
    }
}
#[doc = "Field `LBM` writer - Loop-Back Mode"]
pub type LBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, NPCR_SPEC, LBM_A, O>;
impl<'a, const O: u8> LBM_W<'a, O> {
    #[doc = "Loop-Back Mode is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LBM_A::VALUE1)
    }
    #[doc = "Loop-Back Mode is enabled. This node is connected to an internal (virtual) loop-back CAN bus. All CAN nodes which are in Loop-Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop-Back Mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LBM_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    pub fn rxsel(&self) -> RXSEL_R {
        RXSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxsel(&mut self) -> RXSEL_W<0> {
        RXSEL_W::new(self)
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LBM_W<8> {
        LBM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [npcr](index.html) module"]
pub struct NPCR_SPEC;
impl crate::RegisterSpec for NPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [npcr::R](R) reader structure"]
impl crate::Readable for NPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [npcr::W](W) writer structure"]
impl crate::Writable for NPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NPCR to value 0"]
impl crate::Resettable for NPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
