#[doc = "Register `GPCHK` reader"]
pub struct R(crate::R<GPCHK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPCHK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPCHK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPCHK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPCHK` writer"]
pub struct W(crate::W<GPCHK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCHK_SPEC>;
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
impl From<crate::W<GPCHK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCHK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PASE` reader - Parity Checker Automatic start/stop"]
pub type PASE_R = crate::BitReader<bool>;
#[doc = "Field `PASE` writer - Parity Checker Automatic start/stop"]
pub type PASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHK_SPEC, bool, O>;
#[doc = "Field `PACS` reader - Parity Checker Automatic start/stop selector"]
pub type PACS_R = crate::FieldReader<u8, PACS_A>;
#[doc = "Parity Checker Automatic start/stop selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PACS_A {
    #[doc = "0: CC80"]
    VALUE1 = 0,
    #[doc = "1: CC81"]
    VALUE2 = 1,
    #[doc = "2: CC82"]
    VALUE3 = 2,
    #[doc = "3: CC83"]
    VALUE4 = 3,
}
impl From<PACS_A> for u8 {
    #[inline(always)]
    fn from(variant: PACS_A) -> Self {
        variant as _
    }
}
impl PACS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PACS_A {
        match self.bits {
            0 => PACS_A::VALUE1,
            1 => PACS_A::VALUE2,
            2 => PACS_A::VALUE3,
            3 => PACS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PACS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PACS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PACS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PACS_A::VALUE4
    }
}
#[doc = "Field `PACS` writer - Parity Checker Automatic start/stop selector"]
pub type PACS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GPCHK_SPEC, u8, PACS_A, 2, O>;
impl<'a, const O: u8> PACS_W<'a, O> {
    #[doc = "CC80"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PACS_A::VALUE1)
    }
    #[doc = "CC81"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PACS_A::VALUE2)
    }
    #[doc = "CC82"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PACS_A::VALUE3)
    }
    #[doc = "CC83"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PACS_A::VALUE4)
    }
}
#[doc = "Field `PISEL` reader - Driver Input signal selector"]
pub type PISEL_R = crate::FieldReader<u8, PISEL_A>;
#[doc = "Driver Input signal selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PISEL_A {
    #[doc = "0: CC8x.GP01 - driver output is connected to event 1 of slice 0"]
    VALUE1 = 0,
    #[doc = "1: CC8x.GP11 - drive output is connected to event 1 of slice 1"]
    VALUE2 = 1,
    #[doc = "2: CC8x.GP21 - driver output is connected to event 1 of slice 2"]
    VALUE3 = 2,
    #[doc = "3: CC8x.GP31 - driver output is connected to event 1 of slice 3"]
    VALUE4 = 3,
}
impl From<PISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PISEL_A) -> Self {
        variant as _
    }
}
impl PISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PISEL_A {
        match self.bits {
            0 => PISEL_A::VALUE1,
            1 => PISEL_A::VALUE2,
            2 => PISEL_A::VALUE3,
            3 => PISEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PISEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PISEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PISEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PISEL_A::VALUE4
    }
}
#[doc = "Field `PISEL` writer - Driver Input signal selector"]
pub type PISEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GPCHK_SPEC, u8, PISEL_A, 2, O>;
impl<'a, const O: u8> PISEL_W<'a, O> {
    #[doc = "CC8x.GP01 - driver output is connected to event 1 of slice 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PISEL_A::VALUE1)
    }
    #[doc = "CC8x.GP11 - drive output is connected to event 1 of slice 1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PISEL_A::VALUE2)
    }
    #[doc = "CC8x.GP21 - driver output is connected to event 1 of slice 2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PISEL_A::VALUE3)
    }
    #[doc = "CC8x.GP31 - driver output is connected to event 1 of slice 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PISEL_A::VALUE4)
    }
}
#[doc = "Field `PCDS` reader - Parity Checker Delay Input Selector"]
pub type PCDS_R = crate::FieldReader<u8, PCDS_A>;
#[doc = "Parity Checker Delay Input Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCDS_A {
    #[doc = "0: CCU8x.IGBTA"]
    VALUE1 = 0,
    #[doc = "1: CCU8x.IGBTB"]
    VALUE2 = 1,
    #[doc = "2: CCU8x.IGBTC"]
    VALUE3 = 2,
    #[doc = "3: CCU8x.IGBTD"]
    VALUE4 = 3,
}
impl From<PCDS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCDS_A) -> Self {
        variant as _
    }
}
impl PCDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCDS_A {
        match self.bits {
            0 => PCDS_A::VALUE1,
            1 => PCDS_A::VALUE2,
            2 => PCDS_A::VALUE3,
            3 => PCDS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCDS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCDS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PCDS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PCDS_A::VALUE4
    }
}
#[doc = "Field `PCDS` writer - Parity Checker Delay Input Selector"]
pub type PCDS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GPCHK_SPEC, u8, PCDS_A, 2, O>;
impl<'a, const O: u8> PCDS_W<'a, O> {
    #[doc = "CCU8x.IGBTA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCDS_A::VALUE1)
    }
    #[doc = "CCU8x.IGBTB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCDS_A::VALUE2)
    }
    #[doc = "CCU8x.IGBTC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PCDS_A::VALUE3)
    }
    #[doc = "CCU8x.IGBTD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PCDS_A::VALUE4)
    }
}
#[doc = "Field `PCTS` reader - Parity Checker type selector"]
pub type PCTS_R = crate::BitReader<PCTS_A>;
#[doc = "Parity Checker type selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCTS_A {
    #[doc = "0: Even parity enabled"]
    VALUE1 = 0,
    #[doc = "1: Odd parity enabled"]
    VALUE2 = 1,
}
impl From<PCTS_A> for bool {
    #[inline(always)]
    fn from(variant: PCTS_A) -> Self {
        variant as u8 != 0
    }
}
impl PCTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCTS_A {
        match self.bits {
            false => PCTS_A::VALUE1,
            true => PCTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCTS_A::VALUE2
    }
}
#[doc = "Field `PCTS` writer - Parity Checker type selector"]
pub type PCTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHK_SPEC, PCTS_A, O>;
impl<'a, const O: u8> PCTS_W<'a, O> {
    #[doc = "Even parity enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCTS_A::VALUE1)
    }
    #[doc = "Odd parity enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCTS_A::VALUE2)
    }
}
#[doc = "Field `PCST` reader - Parity Checker XOR status"]
pub type PCST_R = crate::BitReader<bool>;
#[doc = "Field `PCSEL0` reader - Parity Checker Slice 0 output selection"]
pub type PCSEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCSEL0` writer - Parity Checker Slice 0 output selection"]
pub type PCSEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPCHK_SPEC, u8, u8, 4, O>;
#[doc = "Field `PCSEL1` reader - Parity Checker Slice 1 output selection"]
pub type PCSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCSEL1` writer - Parity Checker Slice 1 output selection"]
pub type PCSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPCHK_SPEC, u8, u8, 4, O>;
#[doc = "Field `PCSEL2` reader - Parity Checker Slice 2 output selection"]
pub type PCSEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCSEL2` writer - Parity Checker Slice 2 output selection"]
pub type PCSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPCHK_SPEC, u8, u8, 4, O>;
#[doc = "Field `PCSEL3` reader - Parity Checker Slice 3 output selection"]
pub type PCSEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCSEL3` writer - Parity Checker Slice 3 output selection"]
pub type PCSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPCHK_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Parity Checker Automatic start/stop"]
    #[inline(always)]
    pub fn pase(&self) -> PASE_R {
        PASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Parity Checker Automatic start/stop selector"]
    #[inline(always)]
    pub fn pacs(&self) -> PACS_R {
        PACS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Driver Input signal selector"]
    #[inline(always)]
    pub fn pisel(&self) -> PISEL_R {
        PISEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Parity Checker Delay Input Selector"]
    #[inline(always)]
    pub fn pcds(&self) -> PCDS_R {
        PCDS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Parity Checker type selector"]
    #[inline(always)]
    pub fn pcts(&self) -> PCTS_R {
        PCTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Parity Checker XOR status"]
    #[inline(always)]
    pub fn pcst(&self) -> PCST_R {
        PCST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Parity Checker Slice 0 output selection"]
    #[inline(always)]
    pub fn pcsel0(&self) -> PCSEL0_R {
        PCSEL0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Parity Checker Slice 1 output selection"]
    #[inline(always)]
    pub fn pcsel1(&self) -> PCSEL1_R {
        PCSEL1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Parity Checker Slice 2 output selection"]
    #[inline(always)]
    pub fn pcsel2(&self) -> PCSEL2_R {
        PCSEL2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Parity Checker Slice 3 output selection"]
    #[inline(always)]
    pub fn pcsel3(&self) -> PCSEL3_R {
        PCSEL3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Checker Automatic start/stop"]
    #[inline(always)]
    #[must_use]
    pub fn pase(&mut self) -> PASE_W<0> {
        PASE_W::new(self)
    }
    #[doc = "Bits 1:2 - Parity Checker Automatic start/stop selector"]
    #[inline(always)]
    #[must_use]
    pub fn pacs(&mut self) -> PACS_W<1> {
        PACS_W::new(self)
    }
    #[doc = "Bits 3:4 - Driver Input signal selector"]
    #[inline(always)]
    #[must_use]
    pub fn pisel(&mut self) -> PISEL_W<3> {
        PISEL_W::new(self)
    }
    #[doc = "Bits 5:6 - Parity Checker Delay Input Selector"]
    #[inline(always)]
    #[must_use]
    pub fn pcds(&mut self) -> PCDS_W<5> {
        PCDS_W::new(self)
    }
    #[doc = "Bit 7 - Parity Checker type selector"]
    #[inline(always)]
    #[must_use]
    pub fn pcts(&mut self) -> PCTS_W<7> {
        PCTS_W::new(self)
    }
    #[doc = "Bits 16:19 - Parity Checker Slice 0 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel0(&mut self) -> PCSEL0_W<16> {
        PCSEL0_W::new(self)
    }
    #[doc = "Bits 20:23 - Parity Checker Slice 1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel1(&mut self) -> PCSEL1_W<20> {
        PCSEL1_W::new(self)
    }
    #[doc = "Bits 24:27 - Parity Checker Slice 2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel2(&mut self) -> PCSEL2_W<24> {
        PCSEL2_W::new(self)
    }
    #[doc = "Bits 28:31 - Parity Checker Slice 3 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel3(&mut self) -> PCSEL3_W<28> {
        PCSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Checker Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpchk](index.html) module"]
pub struct GPCHK_SPEC;
impl crate::RegisterSpec for GPCHK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpchk::R](R) reader structure"]
impl crate::Readable for GPCHK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpchk::W](W) writer structure"]
impl crate::Writable for GPCHK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPCHK to value 0"]
impl crate::Resettable for GPCHK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
