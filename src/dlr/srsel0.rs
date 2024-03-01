#[doc = "Register `SRSEL0` reader"]
pub type R = crate::R<Srsel0Spec>;
#[doc = "Register `SRSEL0` writer"]
pub type W = crate::W<Srsel0Spec>;
#[doc = "Field `RS0` reader - Request Source for Line 0"]
pub type Rs0R = crate::FieldReader;
#[doc = "Field `RS0` writer - Request Source for Line 0"]
pub type Rs0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS1` reader - Request Source for Line 1"]
pub type Rs1R = crate::FieldReader;
#[doc = "Field `RS1` writer - Request Source for Line 1"]
pub type Rs1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS2` reader - Request Source for Line 2"]
pub type Rs2R = crate::FieldReader;
#[doc = "Field `RS2` writer - Request Source for Line 2"]
pub type Rs2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS3` reader - Request Source for Line 3"]
pub type Rs3R = crate::FieldReader;
#[doc = "Field `RS3` writer - Request Source for Line 3"]
pub type Rs3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS4` reader - Request Source for Line 4"]
pub type Rs4R = crate::FieldReader;
#[doc = "Field `RS4` writer - Request Source for Line 4"]
pub type Rs4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS5` reader - Request Source for Line 5"]
pub type Rs5R = crate::FieldReader;
#[doc = "Field `RS5` writer - Request Source for Line 5"]
pub type Rs5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS6` reader - Request Source for Line 6"]
pub type Rs6R = crate::FieldReader;
#[doc = "Field `RS6` writer - Request Source for Line 6"]
pub type Rs6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RS7` reader - Request Source for Line 7"]
pub type Rs7R = crate::FieldReader;
#[doc = "Field `RS7` writer - Request Source for Line 7"]
pub type Rs7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Request Source for Line 0"]
    #[inline(always)]
    pub fn rs0(&self) -> Rs0R {
        Rs0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Request Source for Line 1"]
    #[inline(always)]
    pub fn rs1(&self) -> Rs1R {
        Rs1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Request Source for Line 2"]
    #[inline(always)]
    pub fn rs2(&self) -> Rs2R {
        Rs2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Request Source for Line 3"]
    #[inline(always)]
    pub fn rs3(&self) -> Rs3R {
        Rs3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Request Source for Line 4"]
    #[inline(always)]
    pub fn rs4(&self) -> Rs4R {
        Rs4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Request Source for Line 5"]
    #[inline(always)]
    pub fn rs5(&self) -> Rs5R {
        Rs5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Request Source for Line 6"]
    #[inline(always)]
    pub fn rs6(&self) -> Rs6R {
        Rs6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Request Source for Line 7"]
    #[inline(always)]
    pub fn rs7(&self) -> Rs7R {
        Rs7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Request Source for Line 0"]
    #[inline(always)]
    #[must_use]
    pub fn rs0(&mut self) -> Rs0W<Srsel0Spec> {
        Rs0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Request Source for Line 1"]
    #[inline(always)]
    #[must_use]
    pub fn rs1(&mut self) -> Rs1W<Srsel0Spec> {
        Rs1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Request Source for Line 2"]
    #[inline(always)]
    #[must_use]
    pub fn rs2(&mut self) -> Rs2W<Srsel0Spec> {
        Rs2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Request Source for Line 3"]
    #[inline(always)]
    #[must_use]
    pub fn rs3(&mut self) -> Rs3W<Srsel0Spec> {
        Rs3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Request Source for Line 4"]
    #[inline(always)]
    #[must_use]
    pub fn rs4(&mut self) -> Rs4W<Srsel0Spec> {
        Rs4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Request Source for Line 5"]
    #[inline(always)]
    #[must_use]
    pub fn rs5(&mut self) -> Rs5W<Srsel0Spec> {
        Rs5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Request Source for Line 6"]
    #[inline(always)]
    #[must_use]
    pub fn rs6(&mut self) -> Rs6W<Srsel0Spec> {
        Rs6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Request Source for Line 7"]
    #[inline(always)]
    #[must_use]
    pub fn rs7(&mut self) -> Rs7W<Srsel0Spec> {
        Rs7W::new(self, 28)
    }
}
#[doc = "Service Request Selection 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srsel0Spec;
impl crate::RegisterSpec for Srsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsel0::R`](R) reader structure"]
impl crate::Readable for Srsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`srsel0::W`](W) writer structure"]
impl crate::Writable for Srsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSEL0 to value 0"]
impl crate::Resettable for Srsel0Spec {
    const RESET_VALUE: u32 = 0;
}
