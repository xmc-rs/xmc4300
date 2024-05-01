#[doc = "Register `DAC0DATA` reader"]
pub type R = crate::R<Dac0dataSpec>;
#[doc = "Register `DAC0DATA` writer"]
pub type W = crate::W<Dac0dataSpec>;
#[doc = "Field `DATA0` reader - DAC0 Data Bits"]
pub type Data0R = crate::FieldReader<u16>;
#[doc = "Field `DATA0` writer - DAC0 Data Bits"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<Dac0dataSpec> {
        Data0W::new(self, 0)
    }
}
#[doc = "DAC0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac0dataSpec;
impl crate::RegisterSpec for Dac0dataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0data::R`](R) reader structure"]
impl crate::Readable for Dac0dataSpec {}
#[doc = "`write(|w| ..)` method takes [`dac0data::W`](W) writer structure"]
impl crate::Writable for Dac0dataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC0DATA to value 0"]
impl crate::Resettable for Dac0dataSpec {
    const RESET_VALUE: u32 = 0;
}
