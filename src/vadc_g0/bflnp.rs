#[doc = "Register `BFLNP` reader"]
pub struct R(crate::R<BFLNP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFLNP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFLNP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFLNP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFLNP` writer"]
pub struct W(crate::W<BFLNP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFLNP_SPEC>;
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
impl From<crate::W<BFLNP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFLNP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BFL0NP` reader - Boundary Flag y Node Pointer"]
pub type BFL0NP_R = crate::FieldReader<u8, BFL0NP_A>;
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BFL0NP_A {
    #[doc = "0: Select common bondary flag output 0"]
    VALUE1 = 0,
    #[doc = "3: Select common bondary flag output 3"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
    #[doc = "15: Disabled, no common output signal"]
    VALUE5 = 15,
}
impl From<BFL0NP_A> for u8 {
    #[inline(always)]
    fn from(variant: BFL0NP_A) -> Self {
        variant as _
    }
}
impl BFL0NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BFL0NP_A> {
        match self.bits {
            0 => Some(BFL0NP_A::VALUE1),
            3 => Some(BFL0NP_A::VALUE2),
            4 => Some(BFL0NP_A::VALUE3),
            7 => Some(BFL0NP_A::VALUE4),
            15 => Some(BFL0NP_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL0NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL0NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFL0NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFL0NP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BFL0NP_A::VALUE5
    }
}
#[doc = "Field `BFL0NP` writer - Boundary Flag y Node Pointer"]
pub type BFL0NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BFLNP_SPEC, u8, BFL0NP_A, 4, O>;
impl<'a, const O: u8> BFL0NP_W<'a, O> {
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL0NP_A::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL0NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL0NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL0NP_A::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL0NP_A::VALUE5)
    }
}
#[doc = "Field `BFL1NP` reader - Boundary Flag y Node Pointer"]
pub type BFL1NP_R = crate::FieldReader<u8, BFL1NP_A>;
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BFL1NP_A {
    #[doc = "0: Select common bondary flag output 0"]
    VALUE1 = 0,
    #[doc = "3: Select common bondary flag output 3"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
    #[doc = "15: Disabled, no common output signal"]
    VALUE5 = 15,
}
impl From<BFL1NP_A> for u8 {
    #[inline(always)]
    fn from(variant: BFL1NP_A) -> Self {
        variant as _
    }
}
impl BFL1NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BFL1NP_A> {
        match self.bits {
            0 => Some(BFL1NP_A::VALUE1),
            3 => Some(BFL1NP_A::VALUE2),
            4 => Some(BFL1NP_A::VALUE3),
            7 => Some(BFL1NP_A::VALUE4),
            15 => Some(BFL1NP_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL1NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL1NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFL1NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFL1NP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BFL1NP_A::VALUE5
    }
}
#[doc = "Field `BFL1NP` writer - Boundary Flag y Node Pointer"]
pub type BFL1NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BFLNP_SPEC, u8, BFL1NP_A, 4, O>;
impl<'a, const O: u8> BFL1NP_W<'a, O> {
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL1NP_A::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL1NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL1NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL1NP_A::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL1NP_A::VALUE5)
    }
}
#[doc = "Field `BFL2NP` reader - Boundary Flag y Node Pointer"]
pub type BFL2NP_R = crate::FieldReader<u8, BFL2NP_A>;
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BFL2NP_A {
    #[doc = "0: Select common bondary flag output 0"]
    VALUE1 = 0,
    #[doc = "3: Select common bondary flag output 3"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
    #[doc = "15: Disabled, no common output signal"]
    VALUE5 = 15,
}
impl From<BFL2NP_A> for u8 {
    #[inline(always)]
    fn from(variant: BFL2NP_A) -> Self {
        variant as _
    }
}
impl BFL2NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BFL2NP_A> {
        match self.bits {
            0 => Some(BFL2NP_A::VALUE1),
            3 => Some(BFL2NP_A::VALUE2),
            4 => Some(BFL2NP_A::VALUE3),
            7 => Some(BFL2NP_A::VALUE4),
            15 => Some(BFL2NP_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL2NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL2NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFL2NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFL2NP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BFL2NP_A::VALUE5
    }
}
#[doc = "Field `BFL2NP` writer - Boundary Flag y Node Pointer"]
pub type BFL2NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BFLNP_SPEC, u8, BFL2NP_A, 4, O>;
impl<'a, const O: u8> BFL2NP_W<'a, O> {
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL2NP_A::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL2NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL2NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL2NP_A::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL2NP_A::VALUE5)
    }
}
#[doc = "Field `BFL3NP` reader - Boundary Flag y Node Pointer"]
pub type BFL3NP_R = crate::FieldReader<u8, BFL3NP_A>;
#[doc = "Boundary Flag y Node Pointer\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BFL3NP_A {
    #[doc = "0: Select common bondary flag output 0"]
    VALUE1 = 0,
    #[doc = "3: Select common bondary flag output 3"]
    VALUE2 = 3,
    #[doc = "4: Select shared service request line 0"]
    VALUE3 = 4,
    #[doc = "7: Select shared service request line 3"]
    VALUE4 = 7,
    #[doc = "15: Disabled, no common output signal"]
    VALUE5 = 15,
}
impl From<BFL3NP_A> for u8 {
    #[inline(always)]
    fn from(variant: BFL3NP_A) -> Self {
        variant as _
    }
}
impl BFL3NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BFL3NP_A> {
        match self.bits {
            0 => Some(BFL3NP_A::VALUE1),
            3 => Some(BFL3NP_A::VALUE2),
            4 => Some(BFL3NP_A::VALUE3),
            7 => Some(BFL3NP_A::VALUE4),
            15 => Some(BFL3NP_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFL3NP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFL3NP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BFL3NP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BFL3NP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BFL3NP_A::VALUE5
    }
}
#[doc = "Field `BFL3NP` writer - Boundary Flag y Node Pointer"]
pub type BFL3NP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BFLNP_SPEC, u8, BFL3NP_A, 4, O>;
impl<'a, const O: u8> BFL3NP_W<'a, O> {
    #[doc = "Select common bondary flag output 0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFL3NP_A::VALUE1)
    }
    #[doc = "Select common bondary flag output 3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFL3NP_A::VALUE2)
    }
    #[doc = "Select shared service request line 0"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BFL3NP_A::VALUE3)
    }
    #[doc = "Select shared service request line 3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(BFL3NP_A::VALUE4)
    }
    #[doc = "Disabled, no common output signal"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(BFL3NP_A::VALUE5)
    }
}
impl R {
    #[doc = "Bits 0:3 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl0np(&self) -> BFL0NP_R {
        BFL0NP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl1np(&self) -> BFL1NP_R {
        BFL1NP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl2np(&self) -> BFL2NP_R {
        BFL2NP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    pub fn bfl3np(&self) -> BFL3NP_R {
        BFL3NP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bfl0np(&mut self) -> BFL0NP_W<0> {
        BFL0NP_W::new(self)
    }
    #[doc = "Bits 4:7 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bfl1np(&mut self) -> BFL1NP_W<4> {
        BFL1NP_W::new(self)
    }
    #[doc = "Bits 8:11 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bfl2np(&mut self) -> BFL2NP_W<8> {
        BFL2NP_W::new(self)
    }
    #[doc = "Bits 12:15 - Boundary Flag y Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn bfl3np(&mut self) -> BFL3NP_W<12> {
        BFL3NP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boundary Flag Node Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bflnp](index.html) module"]
pub struct BFLNP_SPEC;
impl crate::RegisterSpec for BFLNP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bflnp::R](R) reader structure"]
impl crate::Readable for BFLNP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bflnp::W](W) writer structure"]
impl crate::Writable for BFLNP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFLNP to value 0xffff"]
impl crate::Resettable for BFLNP_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
