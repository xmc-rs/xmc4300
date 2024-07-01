#[doc = "Register `GLOBCFG` reader"]
pub type R = crate::R<GLOBCFG_SPEC>;
#[doc = "Register `GLOBCFG` writer"]
pub type W = crate::W<GLOBCFG_SPEC>;
#[doc = "Divider Factor for the Analog Internal Clock\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: fADCI = fADC / 2"]
    VALUE1 = 0,
    #[doc = "1: fADCI = fADC / 2"]
    VALUE2 = 1,
    #[doc = "2: fADCI = fADC / 3"]
    VALUE3 = 2,
    #[doc = "31: fADCI = fADC / 32"]
    VALUE4 = 31,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVA_A {
    type Ux = u8;
}
impl crate::IsEnum for DIVA_A {}
#[doc = "Field `DIVA` reader - Divider Factor for the Analog Internal Clock"]
pub type DIVA_R = crate::FieldReader<DIVA_A>;
impl DIVA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIVA_A> {
        match self.bits {
            0 => Some(DIVA_A::VALUE1),
            1 => Some(DIVA_A::VALUE2),
            2 => Some(DIVA_A::VALUE3),
            31 => Some(DIVA_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVA_A::VALUE1
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVA_A::VALUE2
    }
    #[doc = "fADCI = fADC / 3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVA_A::VALUE3
    }
    #[doc = "fADCI = fADC / 32"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVA_A::VALUE4
    }
}
#[doc = "Field `DIVA` writer - Divider Factor for the Analog Internal Clock"]
pub type DIVA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DIVA_A>;
impl<'a, REG> DIVA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::VALUE1)
    }
    #[doc = "fADCI = fADC / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::VALUE2)
    }
    #[doc = "fADCI = fADC / 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::VALUE3)
    }
    #[doc = "fADCI = fADC / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DIVA_A::VALUE4)
    }
}
#[doc = "Double Clock for the MSB Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMSB_A {
    #[doc = "0: 1 clock cycles for the MSB (standard)"]
    VALUE1 = 0,
    #[doc = "1: 2 clock cycles for the MSB (fADCI > 20 MHz)"]
    VALUE2 = 1,
}
impl From<DCMSB_A> for bool {
    #[inline(always)]
    fn from(variant: DCMSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMSB` reader - Double Clock for the MSB Conversion"]
pub type DCMSB_R = crate::BitReader<DCMSB_A>;
impl DCMSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCMSB_A {
        match self.bits {
            false => DCMSB_A::VALUE1,
            true => DCMSB_A::VALUE2,
        }
    }
    #[doc = "1 clock cycles for the MSB (standard)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DCMSB_A::VALUE1
    }
    #[doc = "2 clock cycles for the MSB (fADCI > 20 MHz)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DCMSB_A::VALUE2
    }
}
#[doc = "Field `DCMSB` writer - Double Clock for the MSB Conversion"]
pub type DCMSB_W<'a, REG> = crate::BitWriter<'a, REG, DCMSB_A>;
impl<'a, REG> DCMSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 clock cycles for the MSB (standard)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DCMSB_A::VALUE1)
    }
    #[doc = "2 clock cycles for the MSB (fADCI > 20 MHz)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DCMSB_A::VALUE2)
    }
}
#[doc = "Divider Factor for the Arbiter Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVD_A {
    #[doc = "0: fADCD = fADC"]
    VALUE1 = 0,
    #[doc = "1: fADCD = fADC / 2"]
    VALUE2 = 1,
    #[doc = "2: fADCD = fADC / 3"]
    VALUE3 = 2,
    #[doc = "3: fADCD = fADC / 4"]
    VALUE4 = 3,
}
impl From<DIVD_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVD_A {
    type Ux = u8;
}
impl crate::IsEnum for DIVD_A {}
#[doc = "Field `DIVD` reader - Divider Factor for the Arbiter Clock"]
pub type DIVD_R = crate::FieldReader<DIVD_A>;
impl DIVD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIVD_A {
        match self.bits {
            0 => DIVD_A::VALUE1,
            1 => DIVD_A::VALUE2,
            2 => DIVD_A::VALUE3,
            3 => DIVD_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "fADCD = fADC"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVD_A::VALUE1
    }
    #[doc = "fADCD = fADC / 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVD_A::VALUE2
    }
    #[doc = "fADCD = fADC / 3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVD_A::VALUE3
    }
    #[doc = "fADCD = fADC / 4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVD_A::VALUE4
    }
}
#[doc = "Field `DIVD` writer - Divider Factor for the Arbiter Clock"]
pub type DIVD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DIVD_A, crate::Safe>;
impl<'a, REG> DIVD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fADCD = fADC"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVD_A::VALUE1)
    }
    #[doc = "fADCD = fADC / 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVD_A::VALUE2)
    }
    #[doc = "fADCD = fADC / 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DIVD_A::VALUE3)
    }
    #[doc = "fADCD = fADC / 4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DIVD_A::VALUE4)
    }
}
#[doc = "Write Control for Divider Parameters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIVWC_A {
    #[doc = "0: No write access to divider parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfields DIVA, DCMSB, DIVD can be written"]
    VALUE2 = 1,
}
impl From<DIVWC_A> for bool {
    #[inline(always)]
    fn from(variant: DIVWC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIVWC` writer - Write Control for Divider Parameters"]
pub type DIVWC_W<'a, REG> = crate::BitWriter<'a, REG, DIVWC_A>;
impl<'a, REG> DIVWC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to divider parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVWC_A::VALUE1)
    }
    #[doc = "Bitfields DIVA, DCMSB, DIVD can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVWC_A::VALUE2)
    }
}
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPCAL0_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL0_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPCAL0` reader - Disable Post-Calibration"]
pub type DPCAL0_R = crate::BitReader<DPCAL0_A>;
impl DPCAL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DPCAL0_A {
        match self.bits {
            false => DPCAL0_A::VALUE1,
            true => DPCAL0_A::VALUE2,
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL0_A::VALUE1
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL0_A::VALUE2
    }
}
#[doc = "Field `DPCAL0` writer - Disable Post-Calibration"]
pub type DPCAL0_W<'a, REG> = crate::BitWriter<'a, REG, DPCAL0_A>;
impl<'a, REG> DPCAL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DPCAL0_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DPCAL0_A::VALUE2)
    }
}
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPCAL1_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL1_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPCAL1` reader - Disable Post-Calibration"]
pub type DPCAL1_R = crate::BitReader<DPCAL1_A>;
impl DPCAL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DPCAL1_A {
        match self.bits {
            false => DPCAL1_A::VALUE1,
            true => DPCAL1_A::VALUE2,
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL1_A::VALUE1
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL1_A::VALUE2
    }
}
#[doc = "Field `DPCAL1` writer - Disable Post-Calibration"]
pub type DPCAL1_W<'a, REG> = crate::BitWriter<'a, REG, DPCAL1_A>;
impl<'a, REG> DPCAL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DPCAL1_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DPCAL1_A::VALUE2)
    }
}
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPCAL2_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL2_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPCAL2` reader - Disable Post-Calibration"]
pub type DPCAL2_R = crate::BitReader<DPCAL2_A>;
impl DPCAL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DPCAL2_A {
        match self.bits {
            false => DPCAL2_A::VALUE1,
            true => DPCAL2_A::VALUE2,
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL2_A::VALUE1
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL2_A::VALUE2
    }
}
#[doc = "Field `DPCAL2` writer - Disable Post-Calibration"]
pub type DPCAL2_W<'a, REG> = crate::BitWriter<'a, REG, DPCAL2_A>;
impl<'a, REG> DPCAL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DPCAL2_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DPCAL2_A::VALUE2)
    }
}
#[doc = "Disable Post-Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPCAL3_A {
    #[doc = "0: Automatic post-calibration after each conversion of group x"]
    VALUE1 = 0,
    #[doc = "1: No post-calibration"]
    VALUE2 = 1,
}
impl From<DPCAL3_A> for bool {
    #[inline(always)]
    fn from(variant: DPCAL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPCAL3` reader - Disable Post-Calibration"]
pub type DPCAL3_R = crate::BitReader<DPCAL3_A>;
impl DPCAL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DPCAL3_A {
        match self.bits {
            false => DPCAL3_A::VALUE1,
            true => DPCAL3_A::VALUE2,
        }
    }
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPCAL3_A::VALUE1
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPCAL3_A::VALUE2
    }
}
#[doc = "Field `DPCAL3` writer - Disable Post-Calibration"]
pub type DPCAL3_W<'a, REG> = crate::BitWriter<'a, REG, DPCAL3_A>;
impl<'a, REG> DPCAL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic post-calibration after each conversion of group x"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DPCAL3_A::VALUE1)
    }
    #[doc = "No post-calibration"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DPCAL3_A::VALUE2)
    }
}
#[doc = "Start-Up Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUCAL_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Initiate the start-up calibration phase (indication in bit GxARBCFG.CAL)"]
    VALUE2 = 1,
}
impl From<SUCAL_A> for bool {
    #[inline(always)]
    fn from(variant: SUCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUCAL` writer - Start-Up Calibration"]
pub type SUCAL_W<'a, REG> = crate::BitWriter<'a, REG, SUCAL_A>;
impl<'a, REG> SUCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SUCAL_A::VALUE1)
    }
    #[doc = "Initiate the start-up calibration phase (indication in bit GxARBCFG.CAL)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SUCAL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:4 - Divider Factor for the Analog Internal Clock"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Double Clock for the MSB Conversion"]
    #[inline(always)]
    pub fn dcmsb(&self) -> DCMSB_R {
        DCMSB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Divider Factor for the Arbiter Clock"]
    #[inline(always)]
    pub fn divd(&self) -> DIVD_R {
        DIVD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal0(&self) -> DPCAL0_R {
        DPCAL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal1(&self) -> DPCAL1_R {
        DPCAL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal2(&self) -> DPCAL2_R {
        DPCAL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Disable Post-Calibration"]
    #[inline(always)]
    pub fn dpcal3(&self) -> DPCAL3_R {
        DPCAL3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Divider Factor for the Analog Internal Clock"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DIVA_W<GLOBCFG_SPEC> {
        DIVA_W::new(self, 0)
    }
    #[doc = "Bit 7 - Double Clock for the MSB Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn dcmsb(&mut self) -> DCMSB_W<GLOBCFG_SPEC> {
        DCMSB_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Divider Factor for the Arbiter Clock"]
    #[inline(always)]
    #[must_use]
    pub fn divd(&mut self) -> DIVD_W<GLOBCFG_SPEC> {
        DIVD_W::new(self, 8)
    }
    #[doc = "Bit 15 - Write Control for Divider Parameters"]
    #[inline(always)]
    #[must_use]
    pub fn divwc(&mut self) -> DIVWC_W<GLOBCFG_SPEC> {
        DIVWC_W::new(self, 15)
    }
    #[doc = "Bit 16 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal0(&mut self) -> DPCAL0_W<GLOBCFG_SPEC> {
        DPCAL0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal1(&mut self) -> DPCAL1_W<GLOBCFG_SPEC> {
        DPCAL1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal2(&mut self) -> DPCAL2_W<GLOBCFG_SPEC> {
        DPCAL2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Disable Post-Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dpcal3(&mut self) -> DPCAL3_W<GLOBCFG_SPEC> {
        DPCAL3_W::new(self, 19)
    }
    #[doc = "Bit 31 - Start-Up Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn sucal(&mut self) -> SUCAL_W<GLOBCFG_SPEC> {
        SUCAL_W::new(self, 31)
    }
}
#[doc = "Global Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`globcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`globcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBCFG_SPEC;
impl crate::RegisterSpec for GLOBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globcfg::R`](R) reader structure"]
impl crate::Readable for GLOBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`globcfg::W`](W) writer structure"]
impl crate::Writable for GLOBCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBCFG to value 0x0f"]
impl crate::Resettable for GLOBCFG_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
