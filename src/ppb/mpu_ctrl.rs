#[doc = "Register `MPU_CTRL` reader"]
pub struct R(crate::R<MPU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_CTRL` writer"]
pub struct W(crate::W<MPU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_CTRL_SPEC>;
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
impl From<crate::W<MPU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable MPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: MPU disabled"]
    VALUE1 = 0,
    #[doc = "1: MPU enabled."]
    VALUE2 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable MPU"]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::VALUE1,
            true => ENABLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ENABLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ENABLE_A::VALUE2
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable MPU"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MPU disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENABLE_A::VALUE1)
    }
    #[doc = "MPU enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENABLE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFNMIENA_A {
    #[doc = "0: MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    VALUE1 = 0,
    #[doc = "1: the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    VALUE2 = 1,
}
impl From<HFNMIENA_A> for bool {
    #[inline(always)]
    fn from(variant: HFNMIENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFNMIENA` reader - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
pub struct HFNMIENA_R(crate::FieldReader<bool, HFNMIENA_A>);
impl HFNMIENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFNMIENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFNMIENA_A {
        match self.bits {
            false => HFNMIENA_A::VALUE1,
            true => HFNMIENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HFNMIENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HFNMIENA_A::VALUE2
    }
}
impl core::ops::Deref for HFNMIENA_R {
    type Target = crate::FieldReader<bool, HFNMIENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFNMIENA` writer - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
pub struct HFNMIENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HFNMIENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFNMIENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HFNMIENA_A::VALUE1)
    }
    #[doc = "the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HFNMIENA_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Enables privileged software access to the default memory map\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIVDEFENA_A {
    #[doc = "0: If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    VALUE1 = 0,
    #[doc = "1: If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    VALUE2 = 1,
}
impl From<PRIVDEFENA_A> for bool {
    #[inline(always)]
    fn from(variant: PRIVDEFENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIVDEFENA` reader - Enables privileged software access to the default memory map"]
pub struct PRIVDEFENA_R(crate::FieldReader<bool, PRIVDEFENA_A>);
impl PRIVDEFENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIVDEFENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIVDEFENA_A {
        match self.bits {
            false => PRIVDEFENA_A::VALUE1,
            true => PRIVDEFENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PRIVDEFENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PRIVDEFENA_A::VALUE2
    }
}
impl core::ops::Deref for PRIVDEFENA_R {
    type Target = crate::FieldReader<bool, PRIVDEFENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIVDEFENA` writer - Enables privileged software access to the default memory map"]
pub struct PRIVDEFENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVDEFENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRIVDEFENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRIVDEFENA_A::VALUE1)
    }
    #[doc = "If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIVDEFENA_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Enable MPU"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HFNMIENA_R {
        HFNMIENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline(always)]
    pub fn privdefena(&self) -> PRIVDEFENA_R {
        PRIVDEFENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable MPU"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline(always)]
    pub fn hfnmiena(&mut self) -> HFNMIENA_W {
        HFNMIENA_W { w: self }
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline(always)]
    pub fn privdefena(&mut self) -> PRIVDEFENA_W {
        PRIVDEFENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_ctrl](index.html) module"]
pub struct MPU_CTRL_SPEC;
impl crate::RegisterSpec for MPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_ctrl::R](R) reader structure"]
impl crate::Readable for MPU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_ctrl::W](W) writer structure"]
impl crate::Writable for MPU_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPU_CTRL to value 0"]
impl crate::Resettable for MPU_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
