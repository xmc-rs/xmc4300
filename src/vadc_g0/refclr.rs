#[doc = "Register `REFCLR` writer"]
pub type W = crate::W<REFCLR_SPEC>;
#[doc = "Clear Result Event for Result Register 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV0_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV0_A> for bool {
    #[inline(always)]
    fn from(variant: REV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV0` writer - Clear Result Event for Result Register 0"]
pub type REV0_W<'a, REG> = crate::BitWriter<'a, REG, REV0_A>;
impl<'a, REG> REV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV0_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV0_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV1_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV1_A> for bool {
    #[inline(always)]
    fn from(variant: REV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV1` writer - Clear Result Event for Result Register 1"]
pub type REV1_W<'a, REG> = crate::BitWriter<'a, REG, REV1_A>;
impl<'a, REG> REV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV1_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV1_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV2_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV2_A> for bool {
    #[inline(always)]
    fn from(variant: REV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV2` writer - Clear Result Event for Result Register 2"]
pub type REV2_W<'a, REG> = crate::BitWriter<'a, REG, REV2_A>;
impl<'a, REG> REV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV2_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV2_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV3_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV3_A> for bool {
    #[inline(always)]
    fn from(variant: REV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV3` writer - Clear Result Event for Result Register 3"]
pub type REV3_W<'a, REG> = crate::BitWriter<'a, REG, REV3_A>;
impl<'a, REG> REV3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV3_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV3_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV4_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV4_A> for bool {
    #[inline(always)]
    fn from(variant: REV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV4` writer - Clear Result Event for Result Register 4"]
pub type REV4_W<'a, REG> = crate::BitWriter<'a, REG, REV4_A>;
impl<'a, REG> REV4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV4_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV4_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV5_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV5_A> for bool {
    #[inline(always)]
    fn from(variant: REV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV5` writer - Clear Result Event for Result Register 5"]
pub type REV5_W<'a, REG> = crate::BitWriter<'a, REG, REV5_A>;
impl<'a, REG> REV5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV5_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV5_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV6_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV6_A> for bool {
    #[inline(always)]
    fn from(variant: REV6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV6` writer - Clear Result Event for Result Register 6"]
pub type REV6_W<'a, REG> = crate::BitWriter<'a, REG, REV6_A>;
impl<'a, REG> REV6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV6_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV6_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV7_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV7_A> for bool {
    #[inline(always)]
    fn from(variant: REV7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV7` writer - Clear Result Event for Result Register 7"]
pub type REV7_W<'a, REG> = crate::BitWriter<'a, REG, REV7_A>;
impl<'a, REG> REV7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV7_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV7_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV8_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV8_A> for bool {
    #[inline(always)]
    fn from(variant: REV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV8` writer - Clear Result Event for Result Register 8"]
pub type REV8_W<'a, REG> = crate::BitWriter<'a, REG, REV8_A>;
impl<'a, REG> REV8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV8_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV8_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV9_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV9_A> for bool {
    #[inline(always)]
    fn from(variant: REV9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV9` writer - Clear Result Event for Result Register 9"]
pub type REV9_W<'a, REG> = crate::BitWriter<'a, REG, REV9_A>;
impl<'a, REG> REV9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV9_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV9_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV10_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV10_A> for bool {
    #[inline(always)]
    fn from(variant: REV10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV10` writer - Clear Result Event for Result Register 10"]
pub type REV10_W<'a, REG> = crate::BitWriter<'a, REG, REV10_A>;
impl<'a, REG> REV10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV10_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV10_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV11_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV11_A> for bool {
    #[inline(always)]
    fn from(variant: REV11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV11` writer - Clear Result Event for Result Register 11"]
pub type REV11_W<'a, REG> = crate::BitWriter<'a, REG, REV11_A>;
impl<'a, REG> REV11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV11_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV11_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV12_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV12_A> for bool {
    #[inline(always)]
    fn from(variant: REV12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV12` writer - Clear Result Event for Result Register 12"]
pub type REV12_W<'a, REG> = crate::BitWriter<'a, REG, REV12_A>;
impl<'a, REG> REV12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV12_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV12_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV13_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV13_A> for bool {
    #[inline(always)]
    fn from(variant: REV13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV13` writer - Clear Result Event for Result Register 13"]
pub type REV13_W<'a, REG> = crate::BitWriter<'a, REG, REV13_A>;
impl<'a, REG> REV13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV13_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV13_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV14_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV14_A> for bool {
    #[inline(always)]
    fn from(variant: REV14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV14` writer - Clear Result Event for Result Register 14"]
pub type REV14_W<'a, REG> = crate::BitWriter<'a, REG, REV14_A>;
impl<'a, REG> REV14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV14_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV14_A::VALUE2)
    }
}
#[doc = "Clear Result Event for Result Register 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV15_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Clear the result event flag in GxREFLAG"]
    VALUE2 = 1,
}
impl From<REV15_A> for bool {
    #[inline(always)]
    fn from(variant: REV15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REV15` writer - Clear Result Event for Result Register 15"]
pub type REV15_W<'a, REG> = crate::BitWriter<'a, REG, REV15_A>;
impl<'a, REG> REV15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(REV15_A::VALUE1)
    }
    #[doc = "Clear the result event flag in GxREFLAG"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(REV15_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Result Event for Result Register 0"]
    #[inline(always)]
    pub fn rev0(&mut self) -> REV0_W<REFCLR_SPEC> {
        REV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Result Event for Result Register 1"]
    #[inline(always)]
    pub fn rev1(&mut self) -> REV1_W<REFCLR_SPEC> {
        REV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Result Event for Result Register 2"]
    #[inline(always)]
    pub fn rev2(&mut self) -> REV2_W<REFCLR_SPEC> {
        REV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Result Event for Result Register 3"]
    #[inline(always)]
    pub fn rev3(&mut self) -> REV3_W<REFCLR_SPEC> {
        REV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Result Event for Result Register 4"]
    #[inline(always)]
    pub fn rev4(&mut self) -> REV4_W<REFCLR_SPEC> {
        REV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Result Event for Result Register 5"]
    #[inline(always)]
    pub fn rev5(&mut self) -> REV5_W<REFCLR_SPEC> {
        REV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Result Event for Result Register 6"]
    #[inline(always)]
    pub fn rev6(&mut self) -> REV6_W<REFCLR_SPEC> {
        REV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Result Event for Result Register 7"]
    #[inline(always)]
    pub fn rev7(&mut self) -> REV7_W<REFCLR_SPEC> {
        REV7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Result Event for Result Register 8"]
    #[inline(always)]
    pub fn rev8(&mut self) -> REV8_W<REFCLR_SPEC> {
        REV8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Result Event for Result Register 9"]
    #[inline(always)]
    pub fn rev9(&mut self) -> REV9_W<REFCLR_SPEC> {
        REV9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Result Event for Result Register 10"]
    #[inline(always)]
    pub fn rev10(&mut self) -> REV10_W<REFCLR_SPEC> {
        REV10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear Result Event for Result Register 11"]
    #[inline(always)]
    pub fn rev11(&mut self) -> REV11_W<REFCLR_SPEC> {
        REV11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear Result Event for Result Register 12"]
    #[inline(always)]
    pub fn rev12(&mut self) -> REV12_W<REFCLR_SPEC> {
        REV12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Result Event for Result Register 13"]
    #[inline(always)]
    pub fn rev13(&mut self) -> REV13_W<REFCLR_SPEC> {
        REV13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear Result Event for Result Register 14"]
    #[inline(always)]
    pub fn rev14(&mut self) -> REV14_W<REFCLR_SPEC> {
        REV14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear Result Event for Result Register 15"]
    #[inline(always)]
    pub fn rev15(&mut self) -> REV15_W<REFCLR_SPEC> {
        REV15_W::new(self, 15)
    }
}
#[doc = "Result Event Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REFCLR_SPEC;
impl crate::RegisterSpec for REFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`refclr::W`](W) writer structure"]
impl crate::Writable for REFCLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REFCLR to value 0"]
impl crate::Resettable for REFCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
