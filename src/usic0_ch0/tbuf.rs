#[doc = "Register `TBUF[%s]` reader"]
pub type R = crate::R<TBUF_SPEC>;
#[doc = "Register `TBUF[%s]` writer"]
pub type W = crate::W<TBUF_SPEC>;
#[doc = "Field `TDATA` reader - Transmit Data"]
pub type TDATA_R = crate::FieldReader<u16>;
#[doc = "Field `TDATA` writer - Transmit Data"]
pub type TDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn tdata(&self) -> TDATA_R {
        TDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn tdata(&mut self) -> TDATA_W<TBUF_SPEC> {
        TDATA_W::new(self, 0)
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
#[doc = "Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBUF_SPEC;
impl crate::RegisterSpec for TBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbuf::R`](R) reader structure"]
impl crate::Readable for TBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbuf::W`](W) writer structure"]
impl crate::Writable for TBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBUF[%s]
to value 0"]
impl crate::Resettable for TBUF_SPEC {
    const RESET_VALUE: u32 = 0;
}
