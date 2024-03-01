#[doc = "Register `CTR` reader"]
pub type R = crate::R<CtrSpec>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CtrSpec>;
#[doc = "Field `ENB` reader - RTC Module Enable"]
pub type EnbR = crate::BitReader;
#[doc = "Field `ENB` writer - RTC Module Enable"]
pub type EnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAE` reader - Timer Alarm Enable for Hibernation Wake-up"]
pub type TaeR = crate::BitReader;
#[doc = "Field `TAE` writer - Timer Alarm Enable for Hibernation Wake-up"]
pub type TaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESEC` reader - Enable Seconds Comparison for Hibernation Wake-up"]
pub type EsecR = crate::BitReader;
#[doc = "Field `ESEC` writer - Enable Seconds Comparison for Hibernation Wake-up"]
pub type EsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMIC` reader - Enable Minutes Comparison for Hibernation Wake-up"]
pub type EmicR = crate::BitReader;
#[doc = "Field `EMIC` writer - Enable Minutes Comparison for Hibernation Wake-up"]
pub type EmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EHOC` reader - Enable Hours Comparison for Hibernation Wake-up"]
pub type EhocR = crate::BitReader;
#[doc = "Field `EHOC` writer - Enable Hours Comparison for Hibernation Wake-up"]
pub type EhocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDAC` reader - Enable Days Comparison for Hibernation Wake-up"]
pub type EdacR = crate::BitReader;
#[doc = "Field `EDAC` writer - Enable Days Comparison for Hibernation Wake-up"]
pub type EdacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMOC` reader - Enable Months Comparison for Hibernation Wake-up"]
pub type EmocR = crate::BitReader;
#[doc = "Field `EMOC` writer - Enable Months Comparison for Hibernation Wake-up"]
pub type EmocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EYEC` reader - Enable Years Comparison for Hibernation Wake-up"]
pub type EyecR = crate::BitReader;
#[doc = "Field `EYEC` writer - Enable Years Comparison for Hibernation Wake-up"]
pub type EyecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - RTC Clock Divider Value"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - RTC Clock Divider Value"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - RTC Module Enable"]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Timer Alarm Enable for Hibernation Wake-up"]
    #[inline(always)]
    pub fn tae(&self) -> TaeR {
        TaeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Seconds Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn esec(&self) -> EsecR {
        EsecR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Minutes Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emic(&self) -> EmicR {
        EmicR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Hours Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn ehoc(&self) -> EhocR {
        EhocR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Days Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn edac(&self) -> EdacR {
        EdacR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Months Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emoc(&self) -> EmocR {
        EmocR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Years Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn eyec(&self) -> EyecR {
        EyecR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - RTC Clock Divider Value"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> EnbW<CtrSpec> {
        EnbW::new(self, 0)
    }
    #[doc = "Bit 2 - Timer Alarm Enable for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn tae(&mut self) -> TaeW<CtrSpec> {
        TaeW::new(self, 2)
    }
    #[doc = "Bit 8 - Enable Seconds Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn esec(&mut self) -> EsecW<CtrSpec> {
        EsecW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Minutes Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn emic(&mut self) -> EmicW<CtrSpec> {
        EmicW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Hours Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn ehoc(&mut self) -> EhocW<CtrSpec> {
        EhocW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Days Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn edac(&mut self) -> EdacW<CtrSpec> {
        EdacW::new(self, 11)
    }
    #[doc = "Bit 13 - Enable Months Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn emoc(&mut self) -> EmocW<CtrSpec> {
        EmocW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Years Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn eyec(&mut self) -> EyecW<CtrSpec> {
        EyecW::new(self, 14)
    }
    #[doc = "Bits 16:31 - RTC Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<CtrSpec> {
        DivW::new(self, 16)
    }
}
#[doc = "RTC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrSpec;
impl crate::RegisterSpec for CtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTR to value 0x7fff_0000"]
impl crate::Resettable for CtrSpec {
    const RESET_VALUE: u32 = 0x7fff_0000;
}
