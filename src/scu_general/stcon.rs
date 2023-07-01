#[doc = "Register `STCON` reader"]
pub struct R(crate::R<STCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCON` writer"]
pub struct W(crate::W<STCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCON_SPEC>;
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
impl From<crate::W<STCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWCON` reader - HW Configuration"]
pub type HWCON_R = crate::FieldReader<HWCON_A>;
#[doc = "HW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HWCON_A {
    #[doc = "0: Normal mode, JTAG"]
    CONST_00 = 0,
    #[doc = "1: ASC BSL enabled"]
    CONST_01 = 1,
    #[doc = "2: BMI customized boot enabled"]
    CONST_10 = 2,
    #[doc = "3: CAN BSL enabled"]
    CONST_11 = 3,
}
impl From<HWCON_A> for u8 {
    #[inline(always)]
    fn from(variant: HWCON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HWCON_A {
    type Ux = u8;
}
impl HWCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWCON_A {
        match self.bits {
            0 => HWCON_A::CONST_00,
            1 => HWCON_A::CONST_01,
            2 => HWCON_A::CONST_10,
            3 => HWCON_A::CONST_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONST_00`"]
    #[inline(always)]
    pub fn is_const_00(&self) -> bool {
        *self == HWCON_A::CONST_00
    }
    #[doc = "Checks if the value of the field is `CONST_01`"]
    #[inline(always)]
    pub fn is_const_01(&self) -> bool {
        *self == HWCON_A::CONST_01
    }
    #[doc = "Checks if the value of the field is `CONST_10`"]
    #[inline(always)]
    pub fn is_const_10(&self) -> bool {
        *self == HWCON_A::CONST_10
    }
    #[doc = "Checks if the value of the field is `CONST_11`"]
    #[inline(always)]
    pub fn is_const_11(&self) -> bool {
        *self == HWCON_A::CONST_11
    }
}
#[doc = "Field `SWCON` reader - SW Configuration"]
pub type SWCON_R = crate::FieldReader<SWCON_A>;
#[doc = "SW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWCON_A {
    #[doc = "0: Normal mode, boot from Boot ROM"]
    CONST_0000 = 0,
    #[doc = "1: ASC BSL enabled"]
    CONST_0001 = 1,
    #[doc = "2: BMI customized boot enabled"]
    CONST_0010 = 2,
    #[doc = "3: CAN BSL enabled"]
    CONST_0011 = 3,
    #[doc = "4: Boot from Code SRAM"]
    CONST_0100 = 4,
    #[doc = "8: Boot from alternate Flash Address 0"]
    CONST_1000 = 8,
    #[doc = "12: Boot from alternate Flash Address 1"]
    CONST_1100 = 12,
    #[doc = "14: Enable fallback Alternate Boot Mode (ABM)"]
    CONST_1110 = 14,
}
impl From<SWCON_A> for u8 {
    #[inline(always)]
    fn from(variant: SWCON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWCON_A {
    type Ux = u8;
}
impl SWCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWCON_A> {
        match self.bits {
            0 => Some(SWCON_A::CONST_0000),
            1 => Some(SWCON_A::CONST_0001),
            2 => Some(SWCON_A::CONST_0010),
            3 => Some(SWCON_A::CONST_0011),
            4 => Some(SWCON_A::CONST_0100),
            8 => Some(SWCON_A::CONST_1000),
            12 => Some(SWCON_A::CONST_1100),
            14 => Some(SWCON_A::CONST_1110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONST_0000`"]
    #[inline(always)]
    pub fn is_const_0000(&self) -> bool {
        *self == SWCON_A::CONST_0000
    }
    #[doc = "Checks if the value of the field is `CONST_0001`"]
    #[inline(always)]
    pub fn is_const_0001(&self) -> bool {
        *self == SWCON_A::CONST_0001
    }
    #[doc = "Checks if the value of the field is `CONST_0010`"]
    #[inline(always)]
    pub fn is_const_0010(&self) -> bool {
        *self == SWCON_A::CONST_0010
    }
    #[doc = "Checks if the value of the field is `CONST_0011`"]
    #[inline(always)]
    pub fn is_const_0011(&self) -> bool {
        *self == SWCON_A::CONST_0011
    }
    #[doc = "Checks if the value of the field is `CONST_0100`"]
    #[inline(always)]
    pub fn is_const_0100(&self) -> bool {
        *self == SWCON_A::CONST_0100
    }
    #[doc = "Checks if the value of the field is `CONST_1000`"]
    #[inline(always)]
    pub fn is_const_1000(&self) -> bool {
        *self == SWCON_A::CONST_1000
    }
    #[doc = "Checks if the value of the field is `CONST_1100`"]
    #[inline(always)]
    pub fn is_const_1100(&self) -> bool {
        *self == SWCON_A::CONST_1100
    }
    #[doc = "Checks if the value of the field is `CONST_1110`"]
    #[inline(always)]
    pub fn is_const_1110(&self) -> bool {
        *self == SWCON_A::CONST_1110
    }
}
#[doc = "Field `SWCON` writer - SW Configuration"]
pub type SWCON_W<'a, const O: u8> = crate::FieldWriter<'a, STCON_SPEC, 4, O, SWCON_A>;
impl<'a, const O: u8> SWCON_W<'a, O> {
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline(always)]
    pub fn const_0000(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_0000)
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn const_0001(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_0001)
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn const_0010(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_0010)
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn const_0011(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_0011)
    }
    #[doc = "Boot from Code SRAM"]
    #[inline(always)]
    pub fn const_0100(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_0100)
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline(always)]
    pub fn const_1000(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_1000)
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline(always)]
    pub fn const_1100(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_1100)
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline(always)]
    pub fn const_1110(self) -> &'a mut W {
        self.variant(SWCON_A::CONST_1110)
    }
}
impl R {
    #[doc = "Bits 0:1 - HW Configuration"]
    #[inline(always)]
    pub fn hwcon(&self) -> HWCON_R {
        HWCON_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    pub fn swcon(&self) -> SWCON_R {
        SWCON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn swcon(&mut self) -> SWCON_W<8> {
        SWCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Startup Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcon](index.html) module"]
pub struct STCON_SPEC;
impl crate::RegisterSpec for STCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcon::R](R) reader structure"]
impl crate::Readable for STCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcon::W](W) writer structure"]
impl crate::Writable for STCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCON to value 0"]
impl crate::Resettable for STCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
