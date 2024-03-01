#[doc = "Register `SW_RESET` reader"]
pub type R = crate::R<SwResetSpec>;
#[doc = "Register `SW_RESET` writer"]
pub type W = crate::W<SwResetSpec>;
#[doc = "Field `SW_RST_ALL` reader - Software Reset for All"]
pub type SwRstAllR = crate::BitReader;
#[doc = "Field `SW_RST_ALL` writer - Software Reset for All"]
pub type SwRstAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Software Reset for CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRstCmdLine {
    #[doc = "0: Work"]
    Value1 = 0,
    #[doc = "1: Reset"]
    Value2 = 1,
}
impl From<SwRstCmdLine> for bool {
    #[inline(always)]
    fn from(variant: SwRstCmdLine) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RST_CMD_LINE` reader - Software Reset for CMD Line"]
pub type SwRstCmdLineR = crate::BitReader<SwRstCmdLine>;
impl SwRstCmdLineR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRstCmdLine {
        match self.bits {
            false => SwRstCmdLine::Value1,
            true => SwRstCmdLine::Value2,
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SwRstCmdLine::Value1
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SwRstCmdLine::Value2
    }
}
#[doc = "Field `SW_RST_CMD_LINE` writer - Software Reset for CMD Line"]
pub type SwRstCmdLineW<'a, REG> = crate::BitWriter<'a, REG, SwRstCmdLine>;
impl<'a, REG> SwRstCmdLineW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRstCmdLine::Value1)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SwRstCmdLine::Value2)
    }
}
#[doc = "Software Reset for DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRstDatLine {
    #[doc = "0: Work"]
    Value1 = 0,
    #[doc = "1: Reset"]
    Value2 = 1,
}
impl From<SwRstDatLine> for bool {
    #[inline(always)]
    fn from(variant: SwRstDatLine) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RST_DAT_LINE` reader - Software Reset for DAT Line"]
pub type SwRstDatLineR = crate::BitReader<SwRstDatLine>;
impl SwRstDatLineR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRstDatLine {
        match self.bits {
            false => SwRstDatLine::Value1,
            true => SwRstDatLine::Value2,
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SwRstDatLine::Value1
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SwRstDatLine::Value2
    }
}
#[doc = "Field `SW_RST_DAT_LINE` writer - Software Reset for DAT Line"]
pub type SwRstDatLineW<'a, REG> = crate::BitWriter<'a, REG, SwRstDatLine>;
impl<'a, REG> SwRstDatLineW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRstDatLine::Value1)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SwRstDatLine::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SwRstAllR {
        SwRstAllR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sw_rst_cmd_line(&self) -> SwRstCmdLineR {
        SwRstCmdLineR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sw_rst_dat_line(&self) -> SwRstDatLineR {
        SwRstDatLineR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_all(&mut self) -> SwRstAllW<SwResetSpec> {
        SwRstAllW::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset for CMD Line"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_cmd_line(&mut self) -> SwRstCmdLineW<SwResetSpec> {
        SwRstCmdLineW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset for DAT Line"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_dat_line(&mut self) -> SwRstDatLineW<SwResetSpec> {
        SwRstDatLineW::new(self, 2)
    }
}
#[doc = "Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwResetSpec;
impl crate::RegisterSpec for SwResetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sw_reset::R`](R) reader structure"]
impl crate::Readable for SwResetSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_reset::W`](W) writer structure"]
impl crate::Writable for SwResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SW_RESET to value 0"]
impl crate::Resettable for SwResetSpec {
    const RESET_VALUE: u8 = 0;
}
