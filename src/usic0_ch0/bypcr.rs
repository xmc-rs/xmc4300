#[doc = "Register `BYPCR` reader"]
pub struct R(crate::R<BYPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BYPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BYPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BYPCR` writer"]
pub struct W(crate::W<BYPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYPCR_SPEC>;
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
impl From<crate::W<BYPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BYPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BWLE` reader - Bypass Word Length"]
pub type BWLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BWLE` writer - Bypass Word Length"]
pub type BWLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BYPCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BDSSM` reader - Bypass Data Single Shot Mode"]
pub type BDSSM_R = crate::BitReader<BDSSM_A>;
#[doc = "Bypass Data Single Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDSSM_A {
    #[doc = "0: The bypass data is still considered as valid after it has been loaded into TBUF. The loading of the data into TBUF does not clear BDV."]
    VALUE1 = 0,
    #[doc = "1: The bypass data is considered as invalid after it has been loaded into TBUF. The loading of the data into TBUF clears BDV."]
    VALUE2 = 1,
}
impl From<BDSSM_A> for bool {
    #[inline(always)]
    fn from(variant: BDSSM_A) -> Self {
        variant as u8 != 0
    }
}
impl BDSSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDSSM_A {
        match self.bits {
            false => BDSSM_A::VALUE1,
            true => BDSSM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BDSSM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BDSSM_A::VALUE2
    }
}
#[doc = "Field `BDSSM` writer - Bypass Data Single Shot Mode"]
pub type BDSSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BYPCR_SPEC, BDSSM_A, O>;
impl<'a, const O: u8> BDSSM_W<'a, O> {
    #[doc = "The bypass data is still considered as valid after it has been loaded into TBUF. The loading of the data into TBUF does not clear BDV."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BDSSM_A::VALUE1)
    }
    #[doc = "The bypass data is considered as invalid after it has been loaded into TBUF. The loading of the data into TBUF clears BDV."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BDSSM_A::VALUE2)
    }
}
#[doc = "Field `BDEN` reader - Bypass Data Enable"]
pub type BDEN_R = crate::FieldReader<u8, BDEN_A>;
#[doc = "Bypass Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BDEN_A {
    #[doc = "0: The transfer of bypass data is disabled."]
    VALUE1 = 0,
    #[doc = "1: The transfer of bypass data to TBUF is possible. Bypass data will be transferred to TBUF according to its priority if BDV = 1."]
    VALUE2 = 1,
    #[doc = "2: Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 0."]
    VALUE3 = 2,
    #[doc = "3: Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 1."]
    VALUE4 = 3,
}
impl From<BDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: BDEN_A) -> Self {
        variant as _
    }
}
impl BDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDEN_A {
        match self.bits {
            0 => BDEN_A::VALUE1,
            1 => BDEN_A::VALUE2,
            2 => BDEN_A::VALUE3,
            3 => BDEN_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BDEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BDEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BDEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BDEN_A::VALUE4
    }
}
#[doc = "Field `BDEN` writer - Bypass Data Enable"]
pub type BDEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BYPCR_SPEC, u8, BDEN_A, 2, O>;
impl<'a, const O: u8> BDEN_W<'a, O> {
    #[doc = "The transfer of bypass data is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BDEN_A::VALUE1)
    }
    #[doc = "The transfer of bypass data to TBUF is possible. Bypass data will be transferred to TBUF according to its priority if BDV = 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BDEN_A::VALUE2)
    }
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 0."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BDEN_A::VALUE3)
    }
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 1."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BDEN_A::VALUE4)
    }
}
#[doc = "Field `BDVTR` reader - Bypass Data Valid Trigger"]
pub type BDVTR_R = crate::BitReader<BDVTR_A>;
#[doc = "Bypass Data Valid Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDVTR_A {
    #[doc = "0: Bit BDV is not influenced by DX2T."]
    VALUE1 = 0,
    #[doc = "1: Bit BDV is set if DX2T is active."]
    VALUE2 = 1,
}
impl From<BDVTR_A> for bool {
    #[inline(always)]
    fn from(variant: BDVTR_A) -> Self {
        variant as u8 != 0
    }
}
impl BDVTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDVTR_A {
        match self.bits {
            false => BDVTR_A::VALUE1,
            true => BDVTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BDVTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BDVTR_A::VALUE2
    }
}
#[doc = "Field `BDVTR` writer - Bypass Data Valid Trigger"]
pub type BDVTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BYPCR_SPEC, BDVTR_A, O>;
impl<'a, const O: u8> BDVTR_W<'a, O> {
    #[doc = "Bit BDV is not influenced by DX2T."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BDVTR_A::VALUE1)
    }
    #[doc = "Bit BDV is set if DX2T is active."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BDVTR_A::VALUE2)
    }
}
#[doc = "Field `BPRIO` reader - Bypass Priority"]
pub type BPRIO_R = crate::BitReader<BPRIO_A>;
#[doc = "Bypass Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPRIO_A {
    #[doc = "0: The transmit FIFO data has a higher priority than the bypass data."]
    VALUE1 = 0,
    #[doc = "1: The bypass data has a higher priority than the transmit FIFO data."]
    VALUE2 = 1,
}
impl From<BPRIO_A> for bool {
    #[inline(always)]
    fn from(variant: BPRIO_A) -> Self {
        variant as u8 != 0
    }
}
impl BPRIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPRIO_A {
        match self.bits {
            false => BPRIO_A::VALUE1,
            true => BPRIO_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BPRIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BPRIO_A::VALUE2
    }
}
#[doc = "Field `BPRIO` writer - Bypass Priority"]
pub type BPRIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, BYPCR_SPEC, BPRIO_A, O>;
impl<'a, const O: u8> BPRIO_W<'a, O> {
    #[doc = "The transmit FIFO data has a higher priority than the bypass data."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPRIO_A::VALUE1)
    }
    #[doc = "The bypass data has a higher priority than the transmit FIFO data."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPRIO_A::VALUE2)
    }
}
#[doc = "Field `BDV` reader - Bypass Data Valid"]
pub type BDV_R = crate::BitReader<BDV_A>;
#[doc = "Bypass Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDV_A {
    #[doc = "0: The bypass data is not valid."]
    VALUE1 = 0,
    #[doc = "1: The bypass data is valid."]
    VALUE2 = 1,
}
impl From<BDV_A> for bool {
    #[inline(always)]
    fn from(variant: BDV_A) -> Self {
        variant as u8 != 0
    }
}
impl BDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDV_A {
        match self.bits {
            false => BDV_A::VALUE1,
            true => BDV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BDV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BDV_A::VALUE2
    }
}
#[doc = "Field `BSELO` reader - Bypass Select Outputs"]
pub type BSELO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSELO` writer - Bypass Select Outputs"]
pub type BSELO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BYPCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `BHPC` reader - Bypass Hardware Port Control"]
pub type BHPC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BHPC` writer - Bypass Hardware Port Control"]
pub type BHPC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BYPCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3 - Bypass Word Length"]
    #[inline(always)]
    pub fn bwle(&self) -> BWLE_R {
        BWLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Bypass Data Single Shot Mode"]
    #[inline(always)]
    pub fn bdssm(&self) -> BDSSM_R {
        BDSSM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Bypass Data Enable"]
    #[inline(always)]
    pub fn bden(&self) -> BDEN_R {
        BDEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Bypass Data Valid Trigger"]
    #[inline(always)]
    pub fn bdvtr(&self) -> BDVTR_R {
        BDVTR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bypass Priority"]
    #[inline(always)]
    pub fn bprio(&self) -> BPRIO_R {
        BPRIO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bypass Data Valid"]
    #[inline(always)]
    pub fn bdv(&self) -> BDV_R {
        BDV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Bypass Select Outputs"]
    #[inline(always)]
    pub fn bselo(&self) -> BSELO_R {
        BSELO_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Bypass Hardware Port Control"]
    #[inline(always)]
    pub fn bhpc(&self) -> BHPC_R {
        BHPC_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bypass Word Length"]
    #[inline(always)]
    #[must_use]
    pub fn bwle(&mut self) -> BWLE_W<0> {
        BWLE_W::new(self)
    }
    #[doc = "Bit 8 - Bypass Data Single Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bdssm(&mut self) -> BDSSM_W<8> {
        BDSSM_W::new(self)
    }
    #[doc = "Bits 10:11 - Bypass Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bden(&mut self) -> BDEN_W<10> {
        BDEN_W::new(self)
    }
    #[doc = "Bit 12 - Bypass Data Valid Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn bdvtr(&mut self) -> BDVTR_W<12> {
        BDVTR_W::new(self)
    }
    #[doc = "Bit 13 - Bypass Priority"]
    #[inline(always)]
    #[must_use]
    pub fn bprio(&mut self) -> BPRIO_W<13> {
        BPRIO_W::new(self)
    }
    #[doc = "Bits 16:20 - Bypass Select Outputs"]
    #[inline(always)]
    #[must_use]
    pub fn bselo(&mut self) -> BSELO_W<16> {
        BSELO_W::new(self)
    }
    #[doc = "Bits 21:23 - Bypass Hardware Port Control"]
    #[inline(always)]
    #[must_use]
    pub fn bhpc(&mut self) -> BHPC_W<21> {
        BHPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bypass Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bypcr](index.html) module"]
pub struct BYPCR_SPEC;
impl crate::RegisterSpec for BYPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bypcr::R](R) reader structure"]
impl crate::Readable for BYPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bypcr::W](W) writer structure"]
impl crate::Writable for BYPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BYPCR to value 0"]
impl crate::Resettable for BYPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
