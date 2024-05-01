#[doc = "Register `DTSCON` reader"]
pub type R = crate::R<DtsconSpec>;
#[doc = "Register `DTSCON` writer"]
pub type W = crate::W<DtsconSpec>;
#[doc = "Sensor Power Down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwd {
    #[doc = "0: The DTS is powered"]
    Const0 = 0,
    #[doc = "1: The DTS is not powered"]
    Const1 = 1,
}
impl From<Pwd> for bool {
    #[inline(always)]
    fn from(variant: Pwd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWD` reader - Sensor Power Down"]
pub type PwdR = crate::BitReader<Pwd>;
impl PwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwd {
        match self.bits {
            false => Pwd::Const0,
            true => Pwd::Const1,
        }
    }
    #[doc = "The DTS is powered"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pwd::Const0
    }
    #[doc = "The DTS is not powered"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pwd::Const1
    }
}
#[doc = "Field `PWD` writer - Sensor Power Down"]
pub type PwdW<'a, REG> = crate::BitWriter<'a, REG, Pwd>;
impl<'a, REG> PwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DTS is powered"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwd::Const0)
    }
    #[doc = "The DTS is not powered"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwd::Const1)
    }
}
#[doc = "Sensor Measurement Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: No DTS measurement is started"]
    Const0 = 0,
    #[doc = "1: DTS measurement is started"]
    Const1 = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - Sensor Measurement Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DTS measurement is started"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Const0)
    }
    #[doc = "DTS measurement is started"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Const1)
    }
}
#[doc = "Field `OFFSET` reader - Offset Calibration Value"]
pub type OffsetR = crate::FieldReader;
#[doc = "Field `OFFSET` writer - Offset Calibration Value"]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GAIN` reader - Gain Calibration Value"]
pub type GainR = crate::FieldReader;
#[doc = "Field `GAIN` writer - Gain Calibration Value"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REFTRIM` reader - Reference Trim Calibration Value"]
pub type ReftrimR = crate::FieldReader;
#[doc = "Field `REFTRIM` writer - Reference Trim Calibration Value"]
pub type ReftrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BGTRIM` reader - Bandgap Trim Calibration Value"]
pub type BgtrimR = crate::FieldReader;
#[doc = "Field `BGTRIM` writer - Bandgap Trim Calibration Value"]
pub type BgtrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline(always)]
    pub fn pwd(&self) -> PwdR {
        PwdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline(always)]
    pub fn reftrim(&self) -> ReftrimR {
        ReftrimR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline(always)]
    pub fn bgtrim(&self) -> BgtrimR {
        BgtrimR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn pwd(&mut self) -> PwdW<DtsconSpec> {
        PwdW::new(self, 0)
    }
    #[doc = "Bit 1 - Sensor Measurement Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<DtsconSpec> {
        StartW::new(self, 1)
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OffsetW<DtsconSpec> {
        OffsetW::new(self, 4)
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GainW<DtsconSpec> {
        GainW::new(self, 11)
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn reftrim(&mut self) -> ReftrimW<DtsconSpec> {
        ReftrimW::new(self, 17)
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn bgtrim(&mut self) -> BgtrimW<DtsconSpec> {
        BgtrimW::new(self, 20)
    }
}
#[doc = "Die Temperature Sensor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtscon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtscon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtsconSpec;
impl crate::RegisterSpec for DtsconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtscon::R`](R) reader structure"]
impl crate::Readable for DtsconSpec {}
#[doc = "`write(|w| ..)` method takes [`dtscon::W`](W) writer structure"]
impl crate::Writable for DtsconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTSCON to value 0x01"]
impl crate::Resettable for DtsconSpec {
    const RESET_VALUE: u32 = 0x01;
}
