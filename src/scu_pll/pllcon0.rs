#[doc = "Register `PLLCON0` reader"]
pub type R = crate::R<Pllcon0Spec>;
#[doc = "Register `PLLCON0` writer"]
pub type W = crate::W<Pllcon0Spec>;
#[doc = "VCO Bypass\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcobyp {
    #[doc = "0: Normal operation, VCO is not bypassed"]
    Const0 = 0,
    #[doc = "1: Prescaler Mode, VCO is bypassed"]
    Const1 = 1,
}
impl From<Vcobyp> for bool {
    #[inline(always)]
    fn from(variant: Vcobyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOBYP` reader - VCO Bypass"]
pub type VcobypR = crate::BitReader<Vcobyp>;
impl VcobypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcobyp {
        match self.bits {
            false => Vcobyp::Const0,
            true => Vcobyp::Const1,
        }
    }
    #[doc = "Normal operation, VCO is not bypassed"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Vcobyp::Const0
    }
    #[doc = "Prescaler Mode, VCO is bypassed"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Vcobyp::Const1
    }
}
#[doc = "Field `VCOBYP` writer - VCO Bypass"]
pub type VcobypW<'a, REG> = crate::BitWriter<'a, REG, Vcobyp>;
impl<'a, REG> VcobypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, VCO is not bypassed"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vcobyp::Const0)
    }
    #[doc = "Prescaler Mode, VCO is bypassed"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vcobyp::Const1)
    }
}
#[doc = "VCO Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcopwd {
    #[doc = "0: Normal behavior"]
    Const0 = 0,
    #[doc = "1: The VCO is put into a Power Saving Mode and can no longer be used. Only the Bypass and Prescaler Mode are active if previously selected."]
    Const1 = 1,
}
impl From<Vcopwd> for bool {
    #[inline(always)]
    fn from(variant: Vcopwd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOPWD` reader - VCO Power Saving Mode"]
pub type VcopwdR = crate::BitReader<Vcopwd>;
impl VcopwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcopwd {
        match self.bits {
            false => Vcopwd::Const0,
            true => Vcopwd::Const1,
        }
    }
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Vcopwd::Const0
    }
    #[doc = "The VCO is put into a Power Saving Mode and can no longer be used. Only the Bypass and Prescaler Mode are active if previously selected."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Vcopwd::Const1
    }
}
#[doc = "Field `VCOPWD` writer - VCO Power Saving Mode"]
pub type VcopwdW<'a, REG> = crate::BitWriter<'a, REG, Vcopwd>;
impl<'a, REG> VcopwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vcopwd::Const0)
    }
    #[doc = "The VCO is put into a Power Saving Mode and can no longer be used. Only the Bypass and Prescaler Mode are active if previously selected."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vcopwd::Const1)
    }
}
#[doc = "VCO Trim Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcotr {
    #[doc = "0: VCO bandwidth is operation in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    Const0 = 0,
    #[doc = "1: VCO bandwidth is operation in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    Const1 = 1,
}
impl From<Vcotr> for bool {
    #[inline(always)]
    fn from(variant: Vcotr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOTR` reader - VCO Trim Control"]
pub type VcotrR = crate::BitReader<Vcotr>;
impl VcotrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcotr {
        match self.bits {
            false => Vcotr::Const0,
            true => Vcotr::Const1,
        }
    }
    #[doc = "VCO bandwidth is operation in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Vcotr::Const0
    }
    #[doc = "VCO bandwidth is operation in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Vcotr::Const1
    }
}
#[doc = "Field `VCOTR` writer - VCO Trim Control"]
pub type VcotrW<'a, REG> = crate::BitWriter<'a, REG, Vcotr>;
impl<'a, REG> VcotrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VCO bandwidth is operation in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vcotr::Const0)
    }
    #[doc = "VCO bandwidth is operation in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vcotr::Const1)
    }
}
#[doc = "Disconnect Oscillator from VCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Findis {
    #[doc = "0: connect oscillator to the VCO part"]
    Const0 = 0,
    #[doc = "1: disconnect oscillator from the VCO part."]
    Const1 = 1,
}
impl From<Findis> for bool {
    #[inline(always)]
    fn from(variant: Findis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FINDIS` reader - Disconnect Oscillator from VCO"]
pub type FindisR = crate::BitReader<Findis>;
impl FindisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Findis {
        match self.bits {
            false => Findis::Const0,
            true => Findis::Const1,
        }
    }
    #[doc = "connect oscillator to the VCO part"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Findis::Const0
    }
    #[doc = "disconnect oscillator from the VCO part."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Findis::Const1
    }
}
#[doc = "Field `FINDIS` writer - Disconnect Oscillator from VCO"]
pub type FindisW<'a, REG> = crate::BitWriter<'a, REG, Findis>;
impl<'a, REG> FindisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "connect oscillator to the VCO part"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Findis::Const0)
    }
    #[doc = "disconnect oscillator from the VCO part."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Findis::Const1)
    }
}
#[doc = "Oscillator Disconnect Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscdiscdis {
    #[doc = "0: In case of a PLL loss-of-lock bit FINDIS is set"]
    Const0 = 0,
    #[doc = "1: In case of a PLL loss-of-lock bit FINDIS is cleared"]
    Const1 = 1,
}
impl From<Oscdiscdis> for bool {
    #[inline(always)]
    fn from(variant: Oscdiscdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCDISCDIS` reader - Oscillator Disconnect Disable"]
pub type OscdiscdisR = crate::BitReader<Oscdiscdis>;
impl OscdiscdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscdiscdis {
        match self.bits {
            false => Oscdiscdis::Const0,
            true => Oscdiscdis::Const1,
        }
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is set"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Oscdiscdis::Const0
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is cleared"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Oscdiscdis::Const1
    }
}
#[doc = "Field `OSCDISCDIS` writer - Oscillator Disconnect Disable"]
pub type OscdiscdisW<'a, REG> = crate::BitWriter<'a, REG, Oscdiscdis>;
impl<'a, REG> OscdiscdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is set"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Oscdiscdis::Const0)
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is cleared"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Oscdiscdis::Const1)
    }
}
#[doc = "PLL Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllpwd {
    #[doc = "0: Normal behavior"]
    Const0 = 0,
    #[doc = "1: The complete PLL block is put into a Power Saving Mode and can no longer be used. Only the Bypass Mode is active if previously selected."]
    Const1 = 1,
}
impl From<Pllpwd> for bool {
    #[inline(always)]
    fn from(variant: Pllpwd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPWD` reader - PLL Power Saving Mode"]
pub type PllpwdR = crate::BitReader<Pllpwd>;
impl PllpwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllpwd {
        match self.bits {
            false => Pllpwd::Const0,
            true => Pllpwd::Const1,
        }
    }
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Pllpwd::Const0
    }
    #[doc = "The complete PLL block is put into a Power Saving Mode and can no longer be used. Only the Bypass Mode is active if previously selected."]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Pllpwd::Const1
    }
}
#[doc = "Field `PLLPWD` writer - PLL Power Saving Mode"]
pub type PllpwdW<'a, REG> = crate::BitWriter<'a, REG, Pllpwd>;
impl<'a, REG> PllpwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpwd::Const0)
    }
    #[doc = "The complete PLL block is put into a Power Saving Mode and can no longer be used. Only the Bypass Mode is active if previously selected."]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpwd::Const1)
    }
}
#[doc = "Oscillator Watchdog Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscres {
    #[doc = "0: The Oscillator Watchdog of the PLL is not reset and remains active"]
    Const0 = 0,
    #[doc = "1: The Oscillator Watchdog of the PLL is reset"]
    Const1 = 1,
}
impl From<Oscres> for bool {
    #[inline(always)]
    fn from(variant: Oscres) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCRES` reader - Oscillator Watchdog Reset"]
pub type OscresR = crate::BitReader<Oscres>;
impl OscresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscres {
        match self.bits {
            false => Oscres::Const0,
            true => Oscres::Const1,
        }
    }
    #[doc = "The Oscillator Watchdog of the PLL is not reset and remains active"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Oscres::Const0
    }
    #[doc = "The Oscillator Watchdog of the PLL is reset"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Oscres::Const1
    }
}
#[doc = "Field `OSCRES` writer - Oscillator Watchdog Reset"]
pub type OscresW<'a, REG> = crate::BitWriter<'a, REG, Oscres>;
impl<'a, REG> OscresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Oscillator Watchdog of the PLL is not reset and remains active"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Oscres::Const0)
    }
    #[doc = "The Oscillator Watchdog of the PLL is reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Oscres::Const1)
    }
}
#[doc = "Field `RESLD` writer - Restart VCO Lock Detection"]
pub type ResldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Automatic Oscillator Calibration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aotren {
    #[doc = "0: Disable"]
    Const0 = 0,
    #[doc = "1: Enable"]
    Const1 = 1,
}
impl From<Aotren> for bool {
    #[inline(always)]
    fn from(variant: Aotren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOTREN` reader - Automatic Oscillator Calibration Enable"]
pub type AotrenR = crate::BitReader<Aotren>;
impl AotrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aotren {
        match self.bits {
            false => Aotren::Const0,
            true => Aotren::Const1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Aotren::Const0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Aotren::Const1
    }
}
#[doc = "Field `AOTREN` writer - Automatic Oscillator Calibration Enable"]
pub type AotrenW<'a, REG> = crate::BitWriter<'a, REG, Aotren>;
impl<'a, REG> AotrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aotren::Const0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aotren::Const1)
    }
}
#[doc = "Factory Oscillator Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fotr {
    #[doc = "0: No effect"]
    Const0 = 0,
    #[doc = "1: Force fixed-value trimming"]
    Const1 = 1,
}
impl From<Fotr> for bool {
    #[inline(always)]
    fn from(variant: Fotr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOTR` reader - Factory Oscillator Calibration"]
pub type FotrR = crate::BitReader<Fotr>;
impl FotrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fotr {
        match self.bits {
            false => Fotr::Const0,
            true => Fotr::Const1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Fotr::Const0
    }
    #[doc = "Force fixed-value trimming"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Fotr::Const1
    }
}
#[doc = "Field `FOTR` writer - Factory Oscillator Calibration"]
pub type FotrW<'a, REG> = crate::BitWriter<'a, REG, Fotr>;
impl<'a, REG> FotrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fotr::Const0)
    }
    #[doc = "Force fixed-value trimming"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fotr::Const1)
    }
}
impl R {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    pub fn vcobyp(&self) -> VcobypR {
        VcobypR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    pub fn vcopwd(&self) -> VcopwdR {
        VcopwdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    pub fn vcotr(&self) -> VcotrR {
        VcotrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    pub fn findis(&self) -> FindisR {
        FindisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    pub fn oscdiscdis(&self) -> OscdiscdisR {
        OscdiscdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    pub fn pllpwd(&self) -> PllpwdR {
        PllpwdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Oscillator Watchdog Reset"]
    #[inline(always)]
    pub fn oscres(&self) -> OscresR {
        OscresR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Automatic Oscillator Calibration Enable"]
    #[inline(always)]
    pub fn aotren(&self) -> AotrenR {
        AotrenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Factory Oscillator Calibration"]
    #[inline(always)]
    pub fn fotr(&self) -> FotrR {
        FotrR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn vcobyp(&mut self) -> VcobypW<Pllcon0Spec> {
        VcobypW::new(self, 0)
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    #[must_use]
    pub fn vcopwd(&mut self) -> VcopwdW<Pllcon0Spec> {
        VcopwdW::new(self, 1)
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcotr(&mut self) -> VcotrW<Pllcon0Spec> {
        VcotrW::new(self, 2)
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    #[must_use]
    pub fn findis(&mut self) -> FindisW<Pllcon0Spec> {
        FindisW::new(self, 4)
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    #[must_use]
    pub fn oscdiscdis(&mut self) -> OscdiscdisW<Pllcon0Spec> {
        OscdiscdisW::new(self, 6)
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pllpwd(&mut self) -> PllpwdW<Pllcon0Spec> {
        PllpwdW::new(self, 16)
    }
    #[doc = "Bit 17 - Oscillator Watchdog Reset"]
    #[inline(always)]
    #[must_use]
    pub fn oscres(&mut self) -> OscresW<Pllcon0Spec> {
        OscresW::new(self, 17)
    }
    #[doc = "Bit 18 - Restart VCO Lock Detection"]
    #[inline(always)]
    #[must_use]
    pub fn resld(&mut self) -> ResldW<Pllcon0Spec> {
        ResldW::new(self, 18)
    }
    #[doc = "Bit 19 - Automatic Oscillator Calibration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aotren(&mut self) -> AotrenW<Pllcon0Spec> {
        AotrenW::new(self, 19)
    }
    #[doc = "Bit 20 - Factory Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn fotr(&mut self) -> FotrW<Pllcon0Spec> {
        FotrW::new(self, 20)
    }
}
#[doc = "PLL Configuration 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcon0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcon0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pllcon0Spec;
impl crate::RegisterSpec for Pllcon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcon0::R`](R) reader structure"]
impl crate::Readable for Pllcon0Spec {}
#[doc = "`write(|w| ..)` method takes [`pllcon0::W`](W) writer structure"]
impl crate::Writable for Pllcon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLCON0 to value 0x0003_0003"]
impl crate::Resettable for Pllcon0Spec {
    const RESET_VALUE: u32 = 0x0003_0003;
}
