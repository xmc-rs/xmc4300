#[doc = "Register `DTSCON` reader"]
pub type R = crate::R<DTSCON_SPEC>;
#[doc = "Register `DTSCON` writer"]
pub type W = crate::W<DTSCON_SPEC>;
#[doc = "Sensor Power Down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_A {
    #[doc = "0: The DTS is powered"]
    CONST_0 = 0,
    #[doc = "1: The DTS is not powered"]
    CONST_1 = 1,
}
impl From<PWD_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWD` reader - Sensor Power Down"]
pub type PWD_R = crate::BitReader<PWD_A>;
impl PWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWD_A {
        match self.bits {
            false => PWD_A::CONST_0,
            true => PWD_A::CONST_1,
        }
    }
    #[doc = "The DTS is powered"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PWD_A::CONST_0
    }
    #[doc = "The DTS is not powered"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PWD_A::CONST_1
    }
}
#[doc = "Field `PWD` writer - Sensor Power Down"]
pub type PWD_W<'a, REG> = crate::BitWriter<'a, REG, PWD_A>;
impl<'a, REG> PWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DTS is powered"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PWD_A::CONST_0)
    }
    #[doc = "The DTS is not powered"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PWD_A::CONST_1)
    }
}
#[doc = "Sensor Measurement Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: No DTS measurement is started"]
    CONST_0 = 0,
    #[doc = "1: DTS measurement is started"]
    CONST_1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - Sensor Measurement Start"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, START_A>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DTS measurement is started"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::CONST_0)
    }
    #[doc = "DTS measurement is started"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::CONST_1)
    }
}
#[doc = "Field `OFFSET` reader - Offset Calibration Value"]
pub type OFFSET_R = crate::FieldReader;
#[doc = "Field `OFFSET` writer - Offset Calibration Value"]
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GAIN` reader - Gain Calibration Value"]
pub type GAIN_R = crate::FieldReader;
#[doc = "Field `GAIN` writer - Gain Calibration Value"]
pub type GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REFTRIM` reader - Reference Trim Calibration Value"]
pub type REFTRIM_R = crate::FieldReader;
#[doc = "Field `REFTRIM` writer - Reference Trim Calibration Value"]
pub type REFTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BGTRIM` reader - Bandgap Trim Calibration Value"]
pub type BGTRIM_R = crate::FieldReader;
#[doc = "Field `BGTRIM` writer - Bandgap Trim Calibration Value"]
pub type BGTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline(always)]
    pub fn pwd(&self) -> PWD_R {
        PWD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline(always)]
    pub fn reftrim(&self) -> REFTRIM_R {
        REFTRIM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline(always)]
    pub fn bgtrim(&self) -> BGTRIM_R {
        BGTRIM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sensor Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn pwd(&mut self) -> PWD_W<DTSCON_SPEC> {
        PWD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sensor Measurement Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<DTSCON_SPEC> {
        START_W::new(self, 1)
    }
    #[doc = "Bits 4:10 - Offset Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<DTSCON_SPEC> {
        OFFSET_W::new(self, 4)
    }
    #[doc = "Bits 11:16 - Gain Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<DTSCON_SPEC> {
        GAIN_W::new(self, 11)
    }
    #[doc = "Bits 17:19 - Reference Trim Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn reftrim(&mut self) -> REFTRIM_W<DTSCON_SPEC> {
        REFTRIM_W::new(self, 17)
    }
    #[doc = "Bits 20:23 - Bandgap Trim Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn bgtrim(&mut self) -> BGTRIM_W<DTSCON_SPEC> {
        BGTRIM_W::new(self, 20)
    }
}
#[doc = "Die Temperature Sensor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtscon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtscon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTSCON_SPEC;
impl crate::RegisterSpec for DTSCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtscon::R`](R) reader structure"]
impl crate::Readable for DTSCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtscon::W`](W) writer structure"]
impl crate::Writable for DTSCON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTSCON to value 0x01"]
impl crate::Resettable for DTSCON_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
