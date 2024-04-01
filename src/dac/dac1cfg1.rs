#[doc = "Register `DAC1CFG1` reader"]
pub type R = crate::R<Dac1cfg1Spec>;
#[doc = "Register `DAC1CFG1` writer"]
pub type W = crate::W<Dac1cfg1Spec>;
#[doc = "Scale value for up- or downscale of the DAC1 input data in steps by the power of 2 (=shift operation)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scale {
    #[doc = "0: no shift = multiplication/division by 1"]
    Value1 = 0,
    #[doc = "1: shift by 1 = multiplication/division by 2"]
    Value2 = 1,
    #[doc = "2: shift by 2 = multiplication/division by 4"]
    Value3 = 2,
    #[doc = "3: shift left by 3 = multiplication/division by 8"]
    Value4 = 3,
    #[doc = "4: shift left by 4 = multiplication/division by 16"]
    Value5 = 4,
    #[doc = "5: shift left by 5 = multiplication/division by 32"]
    Value6 = 5,
    #[doc = "6: shift left by 6 = multiplication/division by 64"]
    Value7 = 6,
    #[doc = "7: shift left by 7 = multiplication/division by 128"]
    Value8 = 7,
}
impl From<Scale> for u8 {
    #[inline(always)]
    fn from(variant: Scale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scale {
    type Ux = u8;
}
impl crate::IsEnum for Scale {}
#[doc = "Field `SCALE` reader - Scale value for up- or downscale of the DAC1 input data in steps by the power of 2 (=shift operation)"]
pub type ScaleR = crate::FieldReader<Scale>;
impl ScaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scale {
        match self.bits {
            0 => Scale::Value1,
            1 => Scale::Value2,
            2 => Scale::Value3,
            3 => Scale::Value4,
            4 => Scale::Value5,
            5 => Scale::Value6,
            6 => Scale::Value7,
            7 => Scale::Value8,
            _ => unreachable!(),
        }
    }
    #[doc = "no shift = multiplication/division by 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Scale::Value1
    }
    #[doc = "shift by 1 = multiplication/division by 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Scale::Value2
    }
    #[doc = "shift by 2 = multiplication/division by 4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Scale::Value3
    }
    #[doc = "shift left by 3 = multiplication/division by 8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Scale::Value4
    }
    #[doc = "shift left by 4 = multiplication/division by 16"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Scale::Value5
    }
    #[doc = "shift left by 5 = multiplication/division by 32"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Scale::Value6
    }
    #[doc = "shift left by 6 = multiplication/division by 64"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Scale::Value7
    }
    #[doc = "shift left by 7 = multiplication/division by 128"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Scale::Value8
    }
}
#[doc = "Field `SCALE` writer - Scale value for up- or downscale of the DAC1 input data in steps by the power of 2 (=shift operation)"]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 3, Scale, crate::Safe>;
impl<'a, REG> ScaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no shift = multiplication/division by 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Value1)
    }
    #[doc = "shift by 1 = multiplication/division by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Value2)
    }
    #[doc = "shift by 2 = multiplication/division by 4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Value3)
    }
    #[doc = "shift left by 3 = multiplication/division by 8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Value4)
    }
    #[doc = "shift left by 4 = multiplication/division by 16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Value5)
    }
    #[doc = "shift left by 5 = multiplication/division by 32"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Value6)
    }
    #[doc = "shift left by 6 = multiplication/division by 64"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Value7)
    }
    #[doc = "shift left by 7 = multiplication/division by 128"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Scale::Value8)
    }
}
#[doc = "Switch between up- and downscale of the DAC1 input data values\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Muldiv {
    #[doc = "0: downscale = division (shift SCALE positions to the right)"]
    Value1 = 0,
    #[doc = "1: upscale = multiplication (shift SCALE positions to the left)"]
    Value2 = 1,
}
impl From<Muldiv> for bool {
    #[inline(always)]
    fn from(variant: Muldiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULDIV` reader - Switch between up- and downscale of the DAC1 input data values"]
pub type MuldivR = crate::BitReader<Muldiv>;
impl MuldivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Muldiv {
        match self.bits {
            false => Muldiv::Value1,
            true => Muldiv::Value2,
        }
    }
    #[doc = "downscale = division (shift SCALE positions to the right)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Muldiv::Value1
    }
    #[doc = "upscale = multiplication (shift SCALE positions to the left)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Muldiv::Value2
    }
}
#[doc = "Field `MULDIV` writer - Switch between up- and downscale of the DAC1 input data values"]
pub type MuldivW<'a, REG> = crate::BitWriter<'a, REG, Muldiv>;
impl<'a, REG> MuldivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "downscale = division (shift SCALE positions to the right)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Muldiv::Value1)
    }
    #[doc = "upscale = multiplication (shift SCALE positions to the left)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Muldiv::Value2)
    }
}
#[doc = "Field `OFFS` reader - 8-bit offset value addition"]
pub type OffsR = crate::FieldReader;
#[doc = "Field `OFFS` writer - 8-bit offset value addition"]
pub type OffsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRIGSEL` reader - Selects one of the eight external trigger sources for DAC1"]
pub type TrigselR = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - Selects one of the eight external trigger sources for DAC1"]
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SWTRIG` reader - Software Trigger"]
pub type SwtrigR = crate::BitReader;
#[doc = "Field `SWTRIG` writer - Software Trigger"]
pub type SwtrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select the trigger source for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigmod {
    #[doc = "0: internal Trigger (integer divided clock - see FREQ parameter)"]
    Value1 = 0,
    #[doc = "1: external Trigger (preselected trigger by TRIGSEL parameter)"]
    Value2 = 1,
    #[doc = "2: software Trigger (see SWTRIG parameter)"]
    Value3 = 2,
}
impl From<Trigmod> for u8 {
    #[inline(always)]
    fn from(variant: Trigmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigmod {
    type Ux = u8;
}
impl crate::IsEnum for Trigmod {}
#[doc = "Field `TRIGMOD` reader - Select the trigger source for channel 1"]
pub type TrigmodR = crate::FieldReader<Trigmod>;
impl TrigmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigmod> {
        match self.bits {
            0 => Some(Trigmod::Value1),
            1 => Some(Trigmod::Value2),
            2 => Some(Trigmod::Value3),
            _ => None,
        }
    }
    #[doc = "internal Trigger (integer divided clock - see FREQ parameter)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trigmod::Value1
    }
    #[doc = "external Trigger (preselected trigger by TRIGSEL parameter)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trigmod::Value2
    }
    #[doc = "software Trigger (see SWTRIG parameter)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Trigmod::Value3
    }
}
#[doc = "Field `TRIGMOD` writer - Select the trigger source for channel 1"]
pub type TrigmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigmod>;
impl<'a, REG> TrigmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "internal Trigger (integer divided clock - see FREQ parameter)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmod::Value1)
    }
    #[doc = "external Trigger (preselected trigger by TRIGSEL parameter)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmod::Value2)
    }
    #[doc = "software Trigger (see SWTRIG parameter)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmod::Value3)
    }
}
#[doc = "Field `ANACFG` reader - DAC1 analog configuration/calibration parameters"]
pub type AnacfgR = crate::FieldReader;
#[doc = "Field `ANACFG` writer - DAC1 analog configuration/calibration parameters"]
pub type AnacfgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Enable analog DAC for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anaen {
    #[doc = "0: DAC1 is set to standby (analog output only)"]
    Value1 = 0,
    #[doc = "1: enable DAC1 (analog output only)"]
    Value2 = 1,
}
impl From<Anaen> for bool {
    #[inline(always)]
    fn from(variant: Anaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANAEN` reader - Enable analog DAC for channel 1"]
pub type AnaenR = crate::BitReader<Anaen>;
impl AnaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anaen {
        match self.bits {
            false => Anaen::Value1,
            true => Anaen::Value2,
        }
    }
    #[doc = "DAC1 is set to standby (analog output only)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Anaen::Value1
    }
    #[doc = "enable DAC1 (analog output only)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Anaen::Value2
    }
}
#[doc = "Field `ANAEN` writer - Enable analog DAC for channel 1"]
pub type AnaenW<'a, REG> = crate::BitWriter<'a, REG, Anaen>;
impl<'a, REG> AnaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC1 is set to standby (analog output only)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Anaen::Value1)
    }
    #[doc = "enable DAC1 (analog output only)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Anaen::Value2)
    }
}
#[doc = "Field `REFCFGH` reader - Higher 4 band-gap configuration/calibration parameters"]
pub type RefcfghR = crate::FieldReader;
#[doc = "Field `REFCFGH` writer - Higher 4 band-gap configuration/calibration parameters"]
pub type RefcfghW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Scale value for up- or downscale of the DAC1 input data in steps by the power of 2 (=shift operation)"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Switch between up- and downscale of the DAC1 input data values"]
    #[inline(always)]
    pub fn muldiv(&self) -> MuldivR {
        MuldivR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - 8-bit offset value addition"]
    #[inline(always)]
    pub fn offs(&self) -> OffsR {
        OffsR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14 - Selects one of the eight external trigger sources for DAC1"]
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    pub fn swtrig(&self) -> SwtrigR {
        SwtrigR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Select the trigger source for channel 1"]
    #[inline(always)]
    pub fn trigmod(&self) -> TrigmodR {
        TrigmodR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - DAC1 analog configuration/calibration parameters"]
    #[inline(always)]
    pub fn anacfg(&self) -> AnacfgR {
        AnacfgR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Enable analog DAC for channel 1"]
    #[inline(always)]
    pub fn anaen(&self) -> AnaenR {
        AnaenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Higher 4 band-gap configuration/calibration parameters"]
    #[inline(always)]
    pub fn refcfgh(&self) -> RefcfghR {
        RefcfghR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Scale value for up- or downscale of the DAC1 input data in steps by the power of 2 (=shift operation)"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> ScaleW<Dac1cfg1Spec> {
        ScaleW::new(self, 0)
    }
    #[doc = "Bit 3 - Switch between up- and downscale of the DAC1 input data values"]
    #[inline(always)]
    #[must_use]
    pub fn muldiv(&mut self) -> MuldivW<Dac1cfg1Spec> {
        MuldivW::new(self, 3)
    }
    #[doc = "Bits 4:11 - 8-bit offset value addition"]
    #[inline(always)]
    #[must_use]
    pub fn offs(&mut self) -> OffsW<Dac1cfg1Spec> {
        OffsW::new(self, 4)
    }
    #[doc = "Bits 12:14 - Selects one of the eight external trigger sources for DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TrigselW<Dac1cfg1Spec> {
        TrigselW::new(self, 12)
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SwtrigW<Dac1cfg1Spec> {
        SwtrigW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Select the trigger source for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn trigmod(&mut self) -> TrigmodW<Dac1cfg1Spec> {
        TrigmodW::new(self, 17)
    }
    #[doc = "Bits 19:23 - DAC1 analog configuration/calibration parameters"]
    #[inline(always)]
    #[must_use]
    pub fn anacfg(&mut self) -> AnacfgW<Dac1cfg1Spec> {
        AnacfgW::new(self, 19)
    }
    #[doc = "Bit 24 - Enable analog DAC for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn anaen(&mut self) -> AnaenW<Dac1cfg1Spec> {
        AnaenW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Higher 4 band-gap configuration/calibration parameters"]
    #[inline(always)]
    #[must_use]
    pub fn refcfgh(&mut self) -> RefcfghW<Dac1cfg1Spec> {
        RefcfghW::new(self, 28)
    }
}
#[doc = "DAC1 Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac1cfg1Spec;
impl crate::RegisterSpec for Dac1cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1cfg1::R`](R) reader structure"]
impl crate::Readable for Dac1cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dac1cfg1::W`](W) writer structure"]
impl crate::Writable for Dac1cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC1CFG1 to value 0"]
impl crate::Resettable for Dac1cfg1Spec {
    const RESET_VALUE: u32 = 0;
}
