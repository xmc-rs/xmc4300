#[doc = "Register `PCR` reader"]
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR` writer"]
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTR0` reader - Protocol Control Bit 0"]
pub type CTR0_R = crate::BitReader<bool>;
#[doc = "Field `CTR0` writer - Protocol Control Bit 0"]
pub type CTR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR1` reader - Protocol Control Bit 1"]
pub type CTR1_R = crate::BitReader<bool>;
#[doc = "Field `CTR1` writer - Protocol Control Bit 1"]
pub type CTR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR2` reader - Protocol Control Bit 2"]
pub type CTR2_R = crate::BitReader<bool>;
#[doc = "Field `CTR2` writer - Protocol Control Bit 2"]
pub type CTR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR3` reader - Protocol Control Bit 3"]
pub type CTR3_R = crate::BitReader<bool>;
#[doc = "Field `CTR3` writer - Protocol Control Bit 3"]
pub type CTR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR4` reader - Protocol Control Bit 4"]
pub type CTR4_R = crate::BitReader<bool>;
#[doc = "Field `CTR4` writer - Protocol Control Bit 4"]
pub type CTR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR5` reader - Protocol Control Bit 5"]
pub type CTR5_R = crate::BitReader<bool>;
#[doc = "Field `CTR5` writer - Protocol Control Bit 5"]
pub type CTR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR6` reader - Protocol Control Bit 6"]
pub type CTR6_R = crate::BitReader<bool>;
#[doc = "Field `CTR6` writer - Protocol Control Bit 6"]
pub type CTR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR7` reader - Protocol Control Bit 7"]
pub type CTR7_R = crate::BitReader<bool>;
#[doc = "Field `CTR7` writer - Protocol Control Bit 7"]
pub type CTR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR8` reader - Protocol Control Bit 8"]
pub type CTR8_R = crate::BitReader<bool>;
#[doc = "Field `CTR8` writer - Protocol Control Bit 8"]
pub type CTR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR9` reader - Protocol Control Bit 9"]
pub type CTR9_R = crate::BitReader<bool>;
#[doc = "Field `CTR9` writer - Protocol Control Bit 9"]
pub type CTR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR10` reader - Protocol Control Bit 10"]
pub type CTR10_R = crate::BitReader<bool>;
#[doc = "Field `CTR10` writer - Protocol Control Bit 10"]
pub type CTR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR11` reader - Protocol Control Bit 11"]
pub type CTR11_R = crate::BitReader<bool>;
#[doc = "Field `CTR11` writer - Protocol Control Bit 11"]
pub type CTR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR12` reader - Protocol Control Bit 12"]
pub type CTR12_R = crate::BitReader<bool>;
#[doc = "Field `CTR12` writer - Protocol Control Bit 12"]
pub type CTR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR13` reader - Protocol Control Bit 13"]
pub type CTR13_R = crate::BitReader<bool>;
#[doc = "Field `CTR13` writer - Protocol Control Bit 13"]
pub type CTR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR14` reader - Protocol Control Bit 14"]
pub type CTR14_R = crate::BitReader<bool>;
#[doc = "Field `CTR14` writer - Protocol Control Bit 14"]
pub type CTR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR15` reader - Protocol Control Bit 15"]
pub type CTR15_R = crate::BitReader<bool>;
#[doc = "Field `CTR15` writer - Protocol Control Bit 15"]
pub type CTR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR16` reader - Protocol Control Bit 16"]
pub type CTR16_R = crate::BitReader<bool>;
#[doc = "Field `CTR16` writer - Protocol Control Bit 16"]
pub type CTR16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR17` reader - Protocol Control Bit 17"]
pub type CTR17_R = crate::BitReader<bool>;
#[doc = "Field `CTR17` writer - Protocol Control Bit 17"]
pub type CTR17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR18` reader - Protocol Control Bit 18"]
pub type CTR18_R = crate::BitReader<bool>;
#[doc = "Field `CTR18` writer - Protocol Control Bit 18"]
pub type CTR18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR19` reader - Protocol Control Bit 19"]
pub type CTR19_R = crate::BitReader<bool>;
#[doc = "Field `CTR19` writer - Protocol Control Bit 19"]
pub type CTR19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR20` reader - Protocol Control Bit 20"]
pub type CTR20_R = crate::BitReader<bool>;
#[doc = "Field `CTR20` writer - Protocol Control Bit 20"]
pub type CTR20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR21` reader - Protocol Control Bit 21"]
pub type CTR21_R = crate::BitReader<bool>;
#[doc = "Field `CTR21` writer - Protocol Control Bit 21"]
pub type CTR21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR22` reader - Protocol Control Bit 22"]
pub type CTR22_R = crate::BitReader<bool>;
#[doc = "Field `CTR22` writer - Protocol Control Bit 22"]
pub type CTR22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR23` reader - Protocol Control Bit 23"]
pub type CTR23_R = crate::BitReader<bool>;
#[doc = "Field `CTR23` writer - Protocol Control Bit 23"]
pub type CTR23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR24` reader - Protocol Control Bit 24"]
pub type CTR24_R = crate::BitReader<bool>;
#[doc = "Field `CTR24` writer - Protocol Control Bit 24"]
pub type CTR24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR25` reader - Protocol Control Bit 25"]
pub type CTR25_R = crate::BitReader<bool>;
#[doc = "Field `CTR25` writer - Protocol Control Bit 25"]
pub type CTR25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR26` reader - Protocol Control Bit 26"]
pub type CTR26_R = crate::BitReader<bool>;
#[doc = "Field `CTR26` writer - Protocol Control Bit 26"]
pub type CTR26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR27` reader - Protocol Control Bit 27"]
pub type CTR27_R = crate::BitReader<bool>;
#[doc = "Field `CTR27` writer - Protocol Control Bit 27"]
pub type CTR27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR28` reader - Protocol Control Bit 28"]
pub type CTR28_R = crate::BitReader<bool>;
#[doc = "Field `CTR28` writer - Protocol Control Bit 28"]
pub type CTR28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR29` reader - Protocol Control Bit 29"]
pub type CTR29_R = crate::BitReader<bool>;
#[doc = "Field `CTR29` writer - Protocol Control Bit 29"]
pub type CTR29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR30` reader - Protocol Control Bit 30"]
pub type CTR30_R = crate::BitReader<bool>;
#[doc = "Field `CTR30` writer - Protocol Control Bit 30"]
pub type CTR30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CTR31` reader - Protocol Control Bit 31"]
pub type CTR31_R = crate::BitReader<bool>;
#[doc = "Field `CTR31` writer - Protocol Control Bit 31"]
pub type CTR31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protocol Control Bit 0"]
    #[inline(always)]
    pub fn ctr0(&self) -> CTR0_R {
        CTR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protocol Control Bit 1"]
    #[inline(always)]
    pub fn ctr1(&self) -> CTR1_R {
        CTR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protocol Control Bit 2"]
    #[inline(always)]
    pub fn ctr2(&self) -> CTR2_R {
        CTR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protocol Control Bit 3"]
    #[inline(always)]
    pub fn ctr3(&self) -> CTR3_R {
        CTR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protocol Control Bit 4"]
    #[inline(always)]
    pub fn ctr4(&self) -> CTR4_R {
        CTR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protocol Control Bit 5"]
    #[inline(always)]
    pub fn ctr5(&self) -> CTR5_R {
        CTR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protocol Control Bit 6"]
    #[inline(always)]
    pub fn ctr6(&self) -> CTR6_R {
        CTR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protocol Control Bit 7"]
    #[inline(always)]
    pub fn ctr7(&self) -> CTR7_R {
        CTR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protocol Control Bit 8"]
    #[inline(always)]
    pub fn ctr8(&self) -> CTR8_R {
        CTR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protocol Control Bit 9"]
    #[inline(always)]
    pub fn ctr9(&self) -> CTR9_R {
        CTR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protocol Control Bit 10"]
    #[inline(always)]
    pub fn ctr10(&self) -> CTR10_R {
        CTR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protocol Control Bit 11"]
    #[inline(always)]
    pub fn ctr11(&self) -> CTR11_R {
        CTR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol Control Bit 12"]
    #[inline(always)]
    pub fn ctr12(&self) -> CTR12_R {
        CTR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protocol Control Bit 13"]
    #[inline(always)]
    pub fn ctr13(&self) -> CTR13_R {
        CTR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Control Bit 14"]
    #[inline(always)]
    pub fn ctr14(&self) -> CTR14_R {
        CTR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Protocol Control Bit 15"]
    #[inline(always)]
    pub fn ctr15(&self) -> CTR15_R {
        CTR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Protocol Control Bit 16"]
    #[inline(always)]
    pub fn ctr16(&self) -> CTR16_R {
        CTR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protocol Control Bit 17"]
    #[inline(always)]
    pub fn ctr17(&self) -> CTR17_R {
        CTR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protocol Control Bit 18"]
    #[inline(always)]
    pub fn ctr18(&self) -> CTR18_R {
        CTR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protocol Control Bit 19"]
    #[inline(always)]
    pub fn ctr19(&self) -> CTR19_R {
        CTR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protocol Control Bit 20"]
    #[inline(always)]
    pub fn ctr20(&self) -> CTR20_R {
        CTR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protocol Control Bit 21"]
    #[inline(always)]
    pub fn ctr21(&self) -> CTR21_R {
        CTR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protocol Control Bit 22"]
    #[inline(always)]
    pub fn ctr22(&self) -> CTR22_R {
        CTR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Protocol Control Bit 23"]
    #[inline(always)]
    pub fn ctr23(&self) -> CTR23_R {
        CTR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Protocol Control Bit 24"]
    #[inline(always)]
    pub fn ctr24(&self) -> CTR24_R {
        CTR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protocol Control Bit 25"]
    #[inline(always)]
    pub fn ctr25(&self) -> CTR25_R {
        CTR25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protocol Control Bit 26"]
    #[inline(always)]
    pub fn ctr26(&self) -> CTR26_R {
        CTR26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Control Bit 27"]
    #[inline(always)]
    pub fn ctr27(&self) -> CTR27_R {
        CTR27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Control Bit 28"]
    #[inline(always)]
    pub fn ctr28(&self) -> CTR28_R {
        CTR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Protocol Control Bit 29"]
    #[inline(always)]
    pub fn ctr29(&self) -> CTR29_R {
        CTR29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Protocol Control Bit 30"]
    #[inline(always)]
    pub fn ctr30(&self) -> CTR30_R {
        CTR30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Protocol Control Bit 31"]
    #[inline(always)]
    pub fn ctr31(&self) -> CTR31_R {
        CTR31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protocol Control Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctr0(&mut self) -> CTR0_W<0> {
        CTR0_W::new(self)
    }
    #[doc = "Bit 1 - Protocol Control Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctr1(&mut self) -> CTR1_W<1> {
        CTR1_W::new(self)
    }
    #[doc = "Bit 2 - Protocol Control Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctr2(&mut self) -> CTR2_W<2> {
        CTR2_W::new(self)
    }
    #[doc = "Bit 3 - Protocol Control Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctr3(&mut self) -> CTR3_W<3> {
        CTR3_W::new(self)
    }
    #[doc = "Bit 4 - Protocol Control Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ctr4(&mut self) -> CTR4_W<4> {
        CTR4_W::new(self)
    }
    #[doc = "Bit 5 - Protocol Control Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ctr5(&mut self) -> CTR5_W<5> {
        CTR5_W::new(self)
    }
    #[doc = "Bit 6 - Protocol Control Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ctr6(&mut self) -> CTR6_W<6> {
        CTR6_W::new(self)
    }
    #[doc = "Bit 7 - Protocol Control Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ctr7(&mut self) -> CTR7_W<7> {
        CTR7_W::new(self)
    }
    #[doc = "Bit 8 - Protocol Control Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ctr8(&mut self) -> CTR8_W<8> {
        CTR8_W::new(self)
    }
    #[doc = "Bit 9 - Protocol Control Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ctr9(&mut self) -> CTR9_W<9> {
        CTR9_W::new(self)
    }
    #[doc = "Bit 10 - Protocol Control Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ctr10(&mut self) -> CTR10_W<10> {
        CTR10_W::new(self)
    }
    #[doc = "Bit 11 - Protocol Control Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ctr11(&mut self) -> CTR11_W<11> {
        CTR11_W::new(self)
    }
    #[doc = "Bit 12 - Protocol Control Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ctr12(&mut self) -> CTR12_W<12> {
        CTR12_W::new(self)
    }
    #[doc = "Bit 13 - Protocol Control Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ctr13(&mut self) -> CTR13_W<13> {
        CTR13_W::new(self)
    }
    #[doc = "Bit 14 - Protocol Control Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ctr14(&mut self) -> CTR14_W<14> {
        CTR14_W::new(self)
    }
    #[doc = "Bit 15 - Protocol Control Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ctr15(&mut self) -> CTR15_W<15> {
        CTR15_W::new(self)
    }
    #[doc = "Bit 16 - Protocol Control Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ctr16(&mut self) -> CTR16_W<16> {
        CTR16_W::new(self)
    }
    #[doc = "Bit 17 - Protocol Control Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ctr17(&mut self) -> CTR17_W<17> {
        CTR17_W::new(self)
    }
    #[doc = "Bit 18 - Protocol Control Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ctr18(&mut self) -> CTR18_W<18> {
        CTR18_W::new(self)
    }
    #[doc = "Bit 19 - Protocol Control Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ctr19(&mut self) -> CTR19_W<19> {
        CTR19_W::new(self)
    }
    #[doc = "Bit 20 - Protocol Control Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ctr20(&mut self) -> CTR20_W<20> {
        CTR20_W::new(self)
    }
    #[doc = "Bit 21 - Protocol Control Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ctr21(&mut self) -> CTR21_W<21> {
        CTR21_W::new(self)
    }
    #[doc = "Bit 22 - Protocol Control Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ctr22(&mut self) -> CTR22_W<22> {
        CTR22_W::new(self)
    }
    #[doc = "Bit 23 - Protocol Control Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ctr23(&mut self) -> CTR23_W<23> {
        CTR23_W::new(self)
    }
    #[doc = "Bit 24 - Protocol Control Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn ctr24(&mut self) -> CTR24_W<24> {
        CTR24_W::new(self)
    }
    #[doc = "Bit 25 - Protocol Control Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn ctr25(&mut self) -> CTR25_W<25> {
        CTR25_W::new(self)
    }
    #[doc = "Bit 26 - Protocol Control Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn ctr26(&mut self) -> CTR26_W<26> {
        CTR26_W::new(self)
    }
    #[doc = "Bit 27 - Protocol Control Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn ctr27(&mut self) -> CTR27_W<27> {
        CTR27_W::new(self)
    }
    #[doc = "Bit 28 - Protocol Control Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn ctr28(&mut self) -> CTR28_W<28> {
        CTR28_W::new(self)
    }
    #[doc = "Bit 29 - Protocol Control Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn ctr29(&mut self) -> CTR29_W<29> {
        CTR29_W::new(self)
    }
    #[doc = "Bit 30 - Protocol Control Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn ctr30(&mut self) -> CTR30_W<30> {
        CTR30_W::new(self)
    }
    #[doc = "Bit 31 - Protocol Control Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ctr31(&mut self) -> CTR31_W<31> {
        CTR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protocol Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](index.html) module"]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr::R](R) reader structure"]
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr::W](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
