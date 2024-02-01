#[doc = "Register `POWER_CTRL` reader"]
pub type R = crate::R<POWER_CTRL_SPEC>;
#[doc = "Register `POWER_CTRL` writer"]
pub type W = crate::W<POWER_CTRL_SPEC>;
#[doc = "Field `SD_BUS_POWER` reader - SD Bus Power"]
pub type SD_BUS_POWER_R = crate::BitReader<SD_BUS_POWER_A>;
#[doc = "SD Bus Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SD_BUS_POWER_A {
    #[doc = "0: Power off"]
    VALUE1 = 0,
    #[doc = "1: Power on"]
    VALUE2 = 1,
}
impl From<SD_BUS_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: SD_BUS_POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl SD_BUS_POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SD_BUS_POWER_A {
        match self.bits {
            false => SD_BUS_POWER_A::VALUE1,
            true => SD_BUS_POWER_A::VALUE2,
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SD_BUS_POWER_A::VALUE1
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SD_BUS_POWER_A::VALUE2
    }
}
#[doc = "Field `SD_BUS_POWER` writer - SD Bus Power"]
pub type SD_BUS_POWER_W<'a, REG> = crate::BitWriter<'a, REG, SD_BUS_POWER_A>;
impl<'a, REG> SD_BUS_POWER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SD_BUS_POWER_A::VALUE1)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SD_BUS_POWER_A::VALUE2)
    }
}
#[doc = "Field `SD_BUS_VOLTAGE_SEL` reader - SD Bus Voltage Select"]
pub type SD_BUS_VOLTAGE_SEL_R = crate::FieldReader<SD_BUS_VOLTAGE_SEL_A>;
#[doc = "SD Bus Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SD_BUS_VOLTAGE_SEL_A {
    #[doc = "7: 3.3V (Flattop.)"]
    VALUE1 = 7,
}
impl From<SD_BUS_VOLTAGE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SD_BUS_VOLTAGE_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SD_BUS_VOLTAGE_SEL_A {
    type Ux = u8;
}
impl SD_BUS_VOLTAGE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SD_BUS_VOLTAGE_SEL_A> {
        match self.bits {
            7 => Some(SD_BUS_VOLTAGE_SEL_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "3.3V (Flattop.)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SD_BUS_VOLTAGE_SEL_A::VALUE1
    }
}
#[doc = "Field `SD_BUS_VOLTAGE_SEL` writer - SD Bus Voltage Select"]
pub type SD_BUS_VOLTAGE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SD_BUS_VOLTAGE_SEL_A>;
impl<'a, REG> SD_BUS_VOLTAGE_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3.3V (Flattop.)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SD_BUS_VOLTAGE_SEL_A::VALUE1)
    }
}
#[doc = "Field `HARDWARE_RESET` reader - Hardware reset"]
pub type HARDWARE_RESET_R = crate::BitReader;
#[doc = "Field `HARDWARE_RESET` writer - Hardware reset"]
pub type HARDWARE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sd_bus_power(&self) -> SD_BUS_POWER_R {
        SD_BUS_POWER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sd_bus_voltage_sel(&self) -> SD_BUS_VOLTAGE_SEL_R {
        SD_BUS_VOLTAGE_SEL_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 4 - Hardware reset"]
    #[inline(always)]
    pub fn hardware_reset(&self) -> HARDWARE_RESET_R {
        HARDWARE_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_power(&mut self) -> SD_BUS_POWER_W<POWER_CTRL_SPEC> {
        SD_BUS_POWER_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_voltage_sel(&mut self) -> SD_BUS_VOLTAGE_SEL_W<POWER_CTRL_SPEC> {
        SD_BUS_VOLTAGE_SEL_W::new(self, 1)
    }
    #[doc = "Bit 4 - Hardware reset"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_reset(&mut self) -> HARDWARE_RESET_W<POWER_CTRL_SPEC> {
        HARDWARE_RESET_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_CTRL_SPEC;
impl crate::RegisterSpec for POWER_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`power_ctrl::R`](R) reader structure"]
impl crate::Readable for POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_ctrl::W`](W) writer structure"]
impl crate::Writable for POWER_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets POWER_CTRL to value 0"]
impl crate::Resettable for POWER_CTRL_SPEC {
    const RESET_VALUE: u8 = 0;
}
