#[doc = "Register `DC_SPEED_COUNT_FIL_DEPTH` reader"]
pub type R = crate::R<DcSpeedCountFilDepthSpec>;
#[doc = "Register `DC_SPEED_COUNT_FIL_DEPTH` writer"]
pub type W = crate::W<DcSpeedCountFilDepthSpec>;
#[doc = "Field `FILTER_DEPTH` reader - Filter depth for averaging the clock period deviation"]
pub type FilterDepthR = crate::FieldReader;
#[doc = "Field `FILTER_DEPTH` writer - Filter depth for averaging the clock period deviation"]
pub type FilterDepthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Filter depth for averaging the clock period deviation"]
    #[inline(always)]
    pub fn filter_depth(&self) -> FilterDepthR {
        FilterDepthR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter depth for averaging the clock period deviation"]
    #[inline(always)]
    #[must_use]
    pub fn filter_depth(&mut self) -> FilterDepthW<DcSpeedCountFilDepthSpec> {
        FilterDepthW::new(self, 0)
    }
}
#[doc = "Speed Counter Filter Depth\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_speed_count_fil_depth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc_speed_count_fil_depth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcSpeedCountFilDepthSpec;
impl crate::RegisterSpec for DcSpeedCountFilDepthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dc_speed_count_fil_depth::R`](R) reader structure"]
impl crate::Readable for DcSpeedCountFilDepthSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_speed_count_fil_depth::W`](W) writer structure"]
impl crate::Writable for DcSpeedCountFilDepthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DC_SPEED_COUNT_FIL_DEPTH to value 0x0c"]
impl crate::Resettable for DcSpeedCountFilDepthSpec {
    const RESET_VALUE: u8 = 0x0c;
}
