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
pub struct RXSEL_R(crate::FieldReader<u8, u8>);
impl RXSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSEL` writer - Receive Select"]
pub struct RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Loop-Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LBM` reader - Loop-Back Mode"]
pub struct LBM_R(crate::FieldReader<bool, LBM_A>);
impl LBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == LBM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LBM_A::VALUE2
    }
}
impl core::ops::Deref for LBM_R {
    type Target = crate::FieldReader<bool, LBM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBM` writer - Loop-Back Mode"]
pub struct LBM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBM_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
impl R {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    pub fn rxsel(&self) -> RXSEL_R {
        RXSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive Select"]
    #[inline(always)]
    pub fn rxsel(&mut self) -> RXSEL_W {
        RXSEL_W { w: self }
    }
    #[doc = "Bit 8 - Loop-Back Mode"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LBM_W {
        LBM_W { w: self }
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
}
#[doc = "`reset()` method sets NPCR to value 0"]
impl crate::Resettable for NPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
