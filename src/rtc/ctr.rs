#[doc = "Register `CTR` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CTR_SPEC>;
#[doc = "Field `ENB` reader - RTC Module Enable"]
pub type ENB_R = crate::BitReader;
#[doc = "Field `ENB` writer - RTC Module Enable"]
pub type ENB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAE` reader - Timer Alarm Enable for Hibernation Wake-up"]
pub type TAE_R = crate::BitReader;
#[doc = "Field `TAE` writer - Timer Alarm Enable for Hibernation Wake-up"]
pub type TAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESEC` reader - Enable Seconds Comparison for Hibernation Wake-up"]
pub type ESEC_R = crate::BitReader;
#[doc = "Field `ESEC` writer - Enable Seconds Comparison for Hibernation Wake-up"]
pub type ESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMIC` reader - Enable Minutes Comparison for Hibernation Wake-up"]
pub type EMIC_R = crate::BitReader;
#[doc = "Field `EMIC` writer - Enable Minutes Comparison for Hibernation Wake-up"]
pub type EMIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EHOC` reader - Enable Hours Comparison for Hibernation Wake-up"]
pub type EHOC_R = crate::BitReader;
#[doc = "Field `EHOC` writer - Enable Hours Comparison for Hibernation Wake-up"]
pub type EHOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDAC` reader - Enable Days Comparison for Hibernation Wake-up"]
pub type EDAC_R = crate::BitReader;
#[doc = "Field `EDAC` writer - Enable Days Comparison for Hibernation Wake-up"]
pub type EDAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMOC` reader - Enable Months Comparison for Hibernation Wake-up"]
pub type EMOC_R = crate::BitReader;
#[doc = "Field `EMOC` writer - Enable Months Comparison for Hibernation Wake-up"]
pub type EMOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EYEC` reader - Enable Years Comparison for Hibernation Wake-up"]
pub type EYEC_R = crate::BitReader;
#[doc = "Field `EYEC` writer - Enable Years Comparison for Hibernation Wake-up"]
pub type EYEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - RTC Clock Divider Value"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - RTC Clock Divider Value"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - RTC Module Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Timer Alarm Enable for Hibernation Wake-up"]
    #[inline(always)]
    pub fn tae(&self) -> TAE_R {
        TAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Seconds Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn esec(&self) -> ESEC_R {
        ESEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Minutes Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emic(&self) -> EMIC_R {
        EMIC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Hours Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn ehoc(&self) -> EHOC_R {
        EHOC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Days Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn edac(&self) -> EDAC_R {
        EDAC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Months Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emoc(&self) -> EMOC_R {
        EMOC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Years Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn eyec(&self) -> EYEC_R {
        EYEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - RTC Clock Divider Value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<CTR_SPEC> {
        ENB_W::new(self, 0)
    }
    #[doc = "Bit 2 - Timer Alarm Enable for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn tae(&mut self) -> TAE_W<CTR_SPEC> {
        TAE_W::new(self, 2)
    }
    #[doc = "Bit 8 - Enable Seconds Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn esec(&mut self) -> ESEC_W<CTR_SPEC> {
        ESEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Minutes Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn emic(&mut self) -> EMIC_W<CTR_SPEC> {
        EMIC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Hours Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn ehoc(&mut self) -> EHOC_W<CTR_SPEC> {
        EHOC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Days Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn edac(&mut self) -> EDAC_W<CTR_SPEC> {
        EDAC_W::new(self, 11)
    }
    #[doc = "Bit 13 - Enable Months Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn emoc(&mut self) -> EMOC_W<CTR_SPEC> {
        EMOC_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Years Comparison for Hibernation Wake-up"]
    #[inline(always)]
    #[must_use]
    pub fn eyec(&mut self) -> EYEC_W<CTR_SPEC> {
        EYEC_W::new(self, 14)
    }
    #[doc = "Bits 16:31 - RTC Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<CTR_SPEC> {
        DIV_W::new(self, 16)
    }
}
#[doc = "RTC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTR to value 0x7fff_0000"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: u32 = 0x7fff_0000;
}
