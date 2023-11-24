#[doc = "Register `DAC0DATA` reader"]
pub type R = crate::R<DAC0DATA_SPEC>;
#[doc = "Register `DAC0DATA` writer"]
pub type W = crate::W<DAC0DATA_SPEC>;
#[doc = "Field `DATA0` reader - DAC0 Data Bits"]
pub type DATA0_R = crate::FieldReader<u16>;
#[doc = "Field `DATA0` writer - DAC0 Data Bits"]
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 Data Bits"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<DAC0DATA_SPEC> {
        DATA0_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC0DATA_SPEC;
impl crate::RegisterSpec for DAC0DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0data::R`](R) reader structure"]
impl crate::Readable for DAC0DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac0data::W`](W) writer structure"]
impl crate::Writable for DAC0DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC0DATA to value 0"]
impl crate::Resettable for DAC0DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
