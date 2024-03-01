#[doc = "Register `IR` reader"]
pub type R = crate::R<IrSpec>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IrSpec>;
#[doc = "Field `IR` reader - Input Register"]
pub type IrR = crate::FieldReader<u32>;
#[doc = "Field `IR` writer - Input Register"]
pub type IrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Input Register"]
    #[inline(always)]
    pub fn ir(&self) -> IrR {
        IrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Register"]
    #[inline(always)]
    #[must_use]
    pub fn ir(&mut self) -> IrW<IrSpec> {
        IrW::new(self, 0)
    }
}
#[doc = "Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrSpec;
impl crate::RegisterSpec for IrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IrSpec {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IrSpec {
    const RESET_VALUE: u32 = 0;
}
