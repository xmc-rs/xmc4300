#[doc = "Register `SW_RESET` reader"]
pub type R = crate::R<SW_RESET_SPEC>;
#[doc = "Register `SW_RESET` writer"]
pub type W = crate::W<SW_RESET_SPEC>;
#[doc = "Field `SW_RST_ALL` reader - Software Reset for All"]
pub type SW_RST_ALL_R = crate::BitReader;
#[doc = "Field `SW_RST_ALL` writer - Software Reset for All"]
pub type SW_RST_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Software Reset for CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW_RST_CMD_LINE_A {
    #[doc = "0: Work"]
    VALUE1 = 0,
    #[doc = "1: Reset"]
    VALUE2 = 1,
}
impl From<SW_RST_CMD_LINE_A> for bool {
    #[inline(always)]
    fn from(variant: SW_RST_CMD_LINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RST_CMD_LINE` reader - Software Reset for CMD Line"]
pub type SW_RST_CMD_LINE_R = crate::BitReader<SW_RST_CMD_LINE_A>;
impl SW_RST_CMD_LINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SW_RST_CMD_LINE_A {
        match self.bits {
            false => SW_RST_CMD_LINE_A::VALUE1,
            true => SW_RST_CMD_LINE_A::VALUE2,
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SW_RST_CMD_LINE_A::VALUE1
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SW_RST_CMD_LINE_A::VALUE2
    }
}
#[doc = "Field `SW_RST_CMD_LINE` writer - Software Reset for CMD Line"]
pub type SW_RST_CMD_LINE_W<'a, REG> = crate::BitWriter<'a, REG, SW_RST_CMD_LINE_A>;
impl<'a, REG> SW_RST_CMD_LINE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SW_RST_CMD_LINE_A::VALUE1)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SW_RST_CMD_LINE_A::VALUE2)
    }
}
#[doc = "Software Reset for DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW_RST_DAT_LINE_A {
    #[doc = "0: Work"]
    VALUE1 = 0,
    #[doc = "1: Reset"]
    VALUE2 = 1,
}
impl From<SW_RST_DAT_LINE_A> for bool {
    #[inline(always)]
    fn from(variant: SW_RST_DAT_LINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RST_DAT_LINE` reader - Software Reset for DAT Line"]
pub type SW_RST_DAT_LINE_R = crate::BitReader<SW_RST_DAT_LINE_A>;
impl SW_RST_DAT_LINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SW_RST_DAT_LINE_A {
        match self.bits {
            false => SW_RST_DAT_LINE_A::VALUE1,
            true => SW_RST_DAT_LINE_A::VALUE2,
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SW_RST_DAT_LINE_A::VALUE1
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SW_RST_DAT_LINE_A::VALUE2
    }
}
#[doc = "Field `SW_RST_DAT_LINE` writer - Software Reset for DAT Line"]
pub type SW_RST_DAT_LINE_W<'a, REG> = crate::BitWriter<'a, REG, SW_RST_DAT_LINE_A>;
impl<'a, REG> SW_RST_DAT_LINE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SW_RST_DAT_LINE_A::VALUE1)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SW_RST_DAT_LINE_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SW_RST_ALL_R {
        SW_RST_ALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sw_rst_cmd_line(&self) -> SW_RST_CMD_LINE_R {
        SW_RST_CMD_LINE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sw_rst_dat_line(&self) -> SW_RST_DAT_LINE_R {
        SW_RST_DAT_LINE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_all(&mut self) -> SW_RST_ALL_W<SW_RESET_SPEC> {
        SW_RST_ALL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset for CMD Line"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_cmd_line(&mut self) -> SW_RST_CMD_LINE_W<SW_RESET_SPEC> {
        SW_RST_CMD_LINE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset for DAT Line"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_dat_line(&mut self) -> SW_RST_DAT_LINE_W<SW_RESET_SPEC> {
        SW_RST_DAT_LINE_W::new(self, 2)
    }
}
#[doc = "Software Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_RESET_SPEC;
impl crate::RegisterSpec for SW_RESET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sw_reset::R`](R) reader structure"]
impl crate::Readable for SW_RESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_reset::W`](W) writer structure"]
impl crate::Writable for SW_RESET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SW_RESET to value 0"]
impl crate::Resettable for SW_RESET_SPEC {
    const RESET_VALUE: u8 = 0;
}
