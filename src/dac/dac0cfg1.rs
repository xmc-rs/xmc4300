#[doc = "Register `DAC0CFG1` reader"]
pub struct R(crate::R<DAC0CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC0CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC0CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC0CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC0CFG1` writer"]
pub struct W(crate::W<DAC0CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC0CFG1_SPEC>;
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
impl From<crate::W<DAC0CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC0CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCALE` reader - Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)"]
pub type SCALE_R = crate::FieldReader<u8, SCALE_A>;
#[doc = "Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCALE_A {
    #[doc = "0: no shift = multiplication/division by 1"]
    VALUE1 = 0,
    #[doc = "1: shift by 1 = multiplication/division by 2"]
    VALUE2 = 1,
    #[doc = "2: shift by 2 = multiplication/division by 4"]
    VALUE3 = 2,
    #[doc = "3: shift left by 3 = multiplication/division by 8"]
    VALUE4 = 3,
    #[doc = "4: shift left by 4 = multiplication/division by 16"]
    VALUE5 = 4,
    #[doc = "5: shift left by 5 = multiplication/division by 32"]
    VALUE6 = 5,
    #[doc = "6: shift left by 6 = multiplication/division by 64"]
    VALUE7 = 6,
    #[doc = "7: shift left by 7 = multiplication/division by 128"]
    VALUE8 = 7,
}
impl From<SCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCALE_A) -> Self {
        variant as _
    }
}
impl SCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCALE_A {
        match self.bits {
            0 => SCALE_A::VALUE1,
            1 => SCALE_A::VALUE2,
            2 => SCALE_A::VALUE3,
            3 => SCALE_A::VALUE4,
            4 => SCALE_A::VALUE5,
            5 => SCALE_A::VALUE6,
            6 => SCALE_A::VALUE7,
            7 => SCALE_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCALE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCALE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SCALE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SCALE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SCALE_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SCALE_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == SCALE_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == SCALE_A::VALUE8
    }
}
#[doc = "Field `SCALE` writer - Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)"]
pub type SCALE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DAC0CFG1_SPEC, u8, SCALE_A, 3, O>;
impl<'a, const O: u8> SCALE_W<'a, O> {
    #[doc = "no shift = multiplication/division by 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE1)
    }
    #[doc = "shift by 1 = multiplication/division by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE2)
    }
    #[doc = "shift by 2 = multiplication/division by 4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE3)
    }
    #[doc = "shift left by 3 = multiplication/division by 8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE4)
    }
    #[doc = "shift left by 4 = multiplication/division by 16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE5)
    }
    #[doc = "shift left by 5 = multiplication/division by 32"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE6)
    }
    #[doc = "shift left by 6 = multiplication/division by 64"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE7)
    }
    #[doc = "shift left by 7 = multiplication/division by 128"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(SCALE_A::VALUE8)
    }
}
#[doc = "Field `MULDIV` reader - Switch between up- and downscale of the DAC0 input data values"]
pub type MULDIV_R = crate::BitReader<MULDIV_A>;
#[doc = "Switch between up- and downscale of the DAC0 input data values\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MULDIV_A {
    #[doc = "0: downscale = division (shift SCALE positions to the right)"]
    VALUE1 = 0,
    #[doc = "1: upscale = multiplication (shift SCALE positions to the left)"]
    VALUE2 = 1,
}
impl From<MULDIV_A> for bool {
    #[inline(always)]
    fn from(variant: MULDIV_A) -> Self {
        variant as u8 != 0
    }
}
impl MULDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MULDIV_A {
        match self.bits {
            false => MULDIV_A::VALUE1,
            true => MULDIV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MULDIV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MULDIV_A::VALUE2
    }
}
#[doc = "Field `MULDIV` writer - Switch between up- and downscale of the DAC0 input data values"]
pub type MULDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC0CFG1_SPEC, MULDIV_A, O>;
impl<'a, const O: u8> MULDIV_W<'a, O> {
    #[doc = "downscale = division (shift SCALE positions to the right)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MULDIV_A::VALUE1)
    }
    #[doc = "upscale = multiplication (shift SCALE positions to the left)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MULDIV_A::VALUE2)
    }
}
#[doc = "Field `OFFS` reader - 8-bit offset value addition"]
pub type OFFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFS` writer - 8-bit offset value addition"]
pub type OFFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC0CFG1_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRIGSEL` reader - Selects one of the eight external trigger sources for DAC0"]
pub type TRIGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGSEL` writer - Selects one of the eight external trigger sources for DAC0"]
pub type TRIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC0CFG1_SPEC, u8, u8, 3, O>;
#[doc = "Field `DATMOD` reader - Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1"]
pub type DATMOD_R = crate::BitReader<DATMOD_A>;
#[doc = "Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATMOD_A {
    #[doc = "0: independent data handling - process data from DATA0 register (bits 11:0) to DAC0 and data from DATA1 register (bits 11:0) to DAC1"]
    VALUE1 = 0,
    #[doc = "1: simultaneous data handling - process data from DAC01 register to both DACs (bits 11:0 to DAC0 and bits 23:12 to DAC1)."]
    VALUE2 = 1,
}
impl From<DATMOD_A> for bool {
    #[inline(always)]
    fn from(variant: DATMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl DATMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMOD_A {
        match self.bits {
            false => DATMOD_A::VALUE1,
            true => DATMOD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DATMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DATMOD_A::VALUE2
    }
}
#[doc = "Field `DATMOD` writer - Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1"]
pub type DATMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC0CFG1_SPEC, DATMOD_A, O>;
impl<'a, const O: u8> DATMOD_W<'a, O> {
    #[doc = "independent data handling - process data from DATA0 register (bits 11:0) to DAC0 and data from DATA1 register (bits 11:0) to DAC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATMOD_A::VALUE1)
    }
    #[doc = "simultaneous data handling - process data from DAC01 register to both DACs (bits 11:0 to DAC0 and bits 23:12 to DAC1)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATMOD_A::VALUE2)
    }
}
#[doc = "Field `SWTRIG` reader - Software Trigger"]
pub type SWTRIG_R = crate::BitReader<bool>;
#[doc = "Field `SWTRIG` writer - Software Trigger"]
pub type SWTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC0CFG1_SPEC, bool, O>;
#[doc = "Field `TRIGMOD` reader - Select the trigger source for channel 0"]
pub type TRIGMOD_R = crate::FieldReader<u8, TRIGMOD_A>;
#[doc = "Select the trigger source for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGMOD_A {
    #[doc = "0: internal Trigger (integer divided clock - see FREQ parameter)"]
    VALUE1 = 0,
    #[doc = "1: external Trigger (preselected trigger by TRIGSEL parameter)"]
    VALUE2 = 1,
    #[doc = "2: software Trigger (see SWTRIG parameter)"]
    VALUE3 = 2,
}
impl From<TRIGMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGMOD_A) -> Self {
        variant as _
    }
}
impl TRIGMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGMOD_A> {
        match self.bits {
            0 => Some(TRIGMOD_A::VALUE1),
            1 => Some(TRIGMOD_A::VALUE2),
            2 => Some(TRIGMOD_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRIGMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRIGMOD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRIGMOD_A::VALUE3
    }
}
#[doc = "Field `TRIGMOD` writer - Select the trigger source for channel 0"]
pub type TRIGMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC0CFG1_SPEC, u8, TRIGMOD_A, 2, O>;
impl<'a, const O: u8> TRIGMOD_W<'a, O> {
    #[doc = "internal Trigger (integer divided clock - see FREQ parameter)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRIGMOD_A::VALUE1)
    }
    #[doc = "external Trigger (preselected trigger by TRIGSEL parameter)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRIGMOD_A::VALUE2)
    }
    #[doc = "software Trigger (see SWTRIG parameter)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRIGMOD_A::VALUE3)
    }
}
#[doc = "Field `ANACFG` reader - DAC0 analog configuration/calibration parameters"]
pub type ANACFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANACFG` writer - DAC0 analog configuration/calibration parameters"]
pub type ANACFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC0CFG1_SPEC, u8, u8, 5, O>;
#[doc = "Field `ANAEN` reader - Enable analog DAC for channel 0"]
pub type ANAEN_R = crate::BitReader<ANAEN_A>;
#[doc = "Enable analog DAC for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANAEN_A {
    #[doc = "0: DAC0 is set to standby (analog output only)"]
    VALUE1 = 0,
    #[doc = "1: enable DAC0 (analog output only)"]
    VALUE2 = 1,
}
impl From<ANAEN_A> for bool {
    #[inline(always)]
    fn from(variant: ANAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ANAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANAEN_A {
        match self.bits {
            false => ANAEN_A::VALUE1,
            true => ANAEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ANAEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ANAEN_A::VALUE2
    }
}
#[doc = "Field `ANAEN` writer - Enable analog DAC for channel 0"]
pub type ANAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC0CFG1_SPEC, ANAEN_A, O>;
impl<'a, const O: u8> ANAEN_W<'a, O> {
    #[doc = "DAC0 is set to standby (analog output only)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ANAEN_A::VALUE1)
    }
    #[doc = "enable DAC0 (analog output only)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ANAEN_A::VALUE2)
    }
}
#[doc = "Field `REFCFGL` reader - Lower 4 band-gap configuration/calibration parameters"]
pub type REFCFGL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFCFGL` writer - Lower 4 band-gap configuration/calibration parameters"]
pub type REFCFGL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAC0CFG1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2 - Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Switch between up- and downscale of the DAC0 input data values"]
    #[inline(always)]
    pub fn muldiv(&self) -> MULDIV_R {
        MULDIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - 8-bit offset value addition"]
    #[inline(always)]
    pub fn offs(&self) -> OFFS_R {
        OFFS_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14 - Selects one of the eight external trigger sources for DAC0"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1"]
    #[inline(always)]
    pub fn datmod(&self) -> DATMOD_R {
        DATMOD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Select the trigger source for channel 0"]
    #[inline(always)]
    pub fn trigmod(&self) -> TRIGMOD_R {
        TRIGMOD_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - DAC0 analog configuration/calibration parameters"]
    #[inline(always)]
    pub fn anacfg(&self) -> ANACFG_R {
        ANACFG_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Enable analog DAC for channel 0"]
    #[inline(always)]
    pub fn anaen(&self) -> ANAEN_R {
        ANAEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Lower 4 band-gap configuration/calibration parameters"]
    #[inline(always)]
    pub fn refcfgl(&self) -> REFCFGL_R {
        REFCFGL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Scale value for up- or downscale of the DAC0 input data in steps by the power of 2 (=shift operation)"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<0> {
        SCALE_W::new(self)
    }
    #[doc = "Bit 3 - Switch between up- and downscale of the DAC0 input data values"]
    #[inline(always)]
    #[must_use]
    pub fn muldiv(&mut self) -> MULDIV_W<3> {
        MULDIV_W::new(self)
    }
    #[doc = "Bits 4:11 - 8-bit offset value addition"]
    #[inline(always)]
    #[must_use]
    pub fn offs(&mut self) -> OFFS_W<4> {
        OFFS_W::new(self)
    }
    #[doc = "Bits 12:14 - Selects one of the eight external trigger sources for DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<12> {
        TRIGSEL_W::new(self)
    }
    #[doc = "Bit 15 - Switch between independent or simultaneous DAC mode and select the input data register for DAC0 and DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn datmod(&mut self) -> DATMOD_W<15> {
        DATMOD_W::new(self)
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SWTRIG_W<16> {
        SWTRIG_W::new(self)
    }
    #[doc = "Bits 17:18 - Select the trigger source for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn trigmod(&mut self) -> TRIGMOD_W<17> {
        TRIGMOD_W::new(self)
    }
    #[doc = "Bits 19:23 - DAC0 analog configuration/calibration parameters"]
    #[inline(always)]
    #[must_use]
    pub fn anacfg(&mut self) -> ANACFG_W<19> {
        ANACFG_W::new(self)
    }
    #[doc = "Bit 24 - Enable analog DAC for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn anaen(&mut self) -> ANAEN_W<24> {
        ANAEN_W::new(self)
    }
    #[doc = "Bits 28:31 - Lower 4 band-gap configuration/calibration parameters"]
    #[inline(always)]
    #[must_use]
    pub fn refcfgl(&mut self) -> REFCFGL_W<28> {
        REFCFGL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC0 Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0cfg1](index.html) module"]
pub struct DAC0CFG1_SPEC;
impl crate::RegisterSpec for DAC0CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac0cfg1::R](R) reader structure"]
impl crate::Readable for DAC0CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac0cfg1::W](W) writer structure"]
impl crate::Writable for DAC0CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC0CFG1 to value 0"]
impl crate::Resettable for DAC0CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
