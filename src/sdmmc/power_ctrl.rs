#[doc = "Register `POWER_CTRL` reader"]
pub type R = crate::R<PowerCtrlSpec>;
#[doc = "Register `POWER_CTRL` writer"]
pub type W = crate::W<PowerCtrlSpec>;
#[doc = "SD Bus Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdBusPower {
    #[doc = "0: Power off"]
    Value1 = 0,
    #[doc = "1: Power on"]
    Value2 = 1,
}
impl From<SdBusPower> for bool {
    #[inline(always)]
    fn from(variant: SdBusPower) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SD_BUS_POWER` reader - SD Bus Power"]
pub type SdBusPowerR = crate::BitReader<SdBusPower>;
impl SdBusPowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdBusPower {
        match self.bits {
            false => SdBusPower::Value1,
            true => SdBusPower::Value2,
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SdBusPower::Value1
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SdBusPower::Value2
    }
}
#[doc = "Field `SD_BUS_POWER` writer - SD Bus Power"]
pub type SdBusPowerW<'a, REG> = crate::BitWriter<'a, REG, SdBusPower>;
impl<'a, REG> SdBusPowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SdBusPower::Value1)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SdBusPower::Value2)
    }
}
#[doc = "SD Bus Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SdBusVoltageSel {
    #[doc = "7: 3.3V (Flattop.)"]
    Value1 = 7,
}
impl From<SdBusVoltageSel> for u8 {
    #[inline(always)]
    fn from(variant: SdBusVoltageSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SdBusVoltageSel {
    type Ux = u8;
}
impl crate::IsEnum for SdBusVoltageSel {}
#[doc = "Field `SD_BUS_VOLTAGE_SEL` reader - SD Bus Voltage Select"]
pub type SdBusVoltageSelR = crate::FieldReader<SdBusVoltageSel>;
impl SdBusVoltageSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SdBusVoltageSel> {
        match self.bits {
            7 => Some(SdBusVoltageSel::Value1),
            _ => None,
        }
    }
    #[doc = "3.3V (Flattop.)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SdBusVoltageSel::Value1
    }
}
#[doc = "Field `SD_BUS_VOLTAGE_SEL` writer - SD Bus Voltage Select"]
pub type SdBusVoltageSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, SdBusVoltageSel>;
impl<'a, REG> SdBusVoltageSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3.3V (Flattop.)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SdBusVoltageSel::Value1)
    }
}
#[doc = "Field `HARDWARE_RESET` reader - Hardware reset"]
pub type HardwareResetR = crate::BitReader;
#[doc = "Field `HARDWARE_RESET` writer - Hardware reset"]
pub type HardwareResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sd_bus_power(&self) -> SdBusPowerR {
        SdBusPowerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sd_bus_voltage_sel(&self) -> SdBusVoltageSelR {
        SdBusVoltageSelR::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 4 - Hardware reset"]
    #[inline(always)]
    pub fn hardware_reset(&self) -> HardwareResetR {
        HardwareResetR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_power(&mut self) -> SdBusPowerW<PowerCtrlSpec> {
        SdBusPowerW::new(self, 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_voltage_sel(&mut self) -> SdBusVoltageSelW<PowerCtrlSpec> {
        SdBusVoltageSelW::new(self, 1)
    }
    #[doc = "Bit 4 - Hardware reset"]
    #[inline(always)]
    #[must_use]
    pub fn hardware_reset(&mut self) -> HardwareResetW<PowerCtrlSpec> {
        HardwareResetW::new(self, 4)
    }
}
#[doc = "Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerCtrlSpec;
impl crate::RegisterSpec for PowerCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`power_ctrl::R`](R) reader structure"]
impl crate::Readable for PowerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`power_ctrl::W`](W) writer structure"]
impl crate::Writable for PowerCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets POWER_CTRL to value 0"]
impl crate::Resettable for PowerCtrlSpec {
    const RESET_VALUE: u8 = 0;
}
