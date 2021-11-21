#[doc = "Register `NSR` reader"]
pub struct R(crate::R<NSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSR` writer"]
pub struct W(crate::W<NSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSR_SPEC>;
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
impl From<crate::W<NSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEC` reader - Last Error Code"]
pub struct LEC_R(crate::FieldReader<u8, u8>);
impl LEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEC` writer - Last Error Code"]
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Message Transmitted Successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOK_A {
    #[doc = "0: No successful transmission since last (most recent) flag reset."]
    VALUE1 = 0,
    #[doc = "1: A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    VALUE2 = 1,
}
impl From<TXOK_A> for bool {
    #[inline(always)]
    fn from(variant: TXOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXOK` reader - Message Transmitted Successfully"]
pub struct TXOK_R(crate::FieldReader<bool, TXOK_A>);
impl TXOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOK_A {
        match self.bits {
            false => TXOK_A::VALUE1,
            true => TXOK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TXOK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TXOK_A::VALUE2
    }
}
impl core::ops::Deref for TXOK_R {
    type Target = crate::FieldReader<bool, TXOK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOK` writer - Message Transmitted Successfully"]
pub struct TXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No successful transmission since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXOK_A::VALUE1)
    }
    #[doc = "A message has been transmitted successfully (error-free and acknowledged by at least another node)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXOK_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Message Received Successfully\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOK_A {
    #[doc = "0: No successful reception since last (most recent) flag reset."]
    VALUE1 = 0,
    #[doc = "1: A message has been received successfully."]
    VALUE2 = 1,
}
impl From<RXOK_A> for bool {
    #[inline(always)]
    fn from(variant: RXOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOK` reader - Message Received Successfully"]
pub struct RXOK_R(crate::FieldReader<bool, RXOK_A>);
impl RXOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOK_A {
        match self.bits {
            false => RXOK_A::VALUE1,
            true => RXOK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RXOK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RXOK_A::VALUE2
    }
}
impl core::ops::Deref for RXOK_R {
    type Target = crate::FieldReader<bool, RXOK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOK` writer - Message Received Successfully"]
pub struct RXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No successful reception since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXOK_A::VALUE1)
    }
    #[doc = "A message has been received successfully."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXOK_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ALERT` reader - Alert Warning"]
pub struct ALERT_R(crate::FieldReader<bool, bool>);
impl ALERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALERT` writer - Alert Warning"]
pub struct ALERT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Error Warning Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWRN_A {
    #[doc = "0: No warning limit exceeded."]
    VALUE1 = 0,
    #[doc = "1: One of the error counters REC or TEC reached the warning limit EWRNLVL."]
    VALUE2 = 1,
}
impl From<EWRN_A> for bool {
    #[inline(always)]
    fn from(variant: EWRN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWRN` reader - Error Warning Status"]
pub struct EWRN_R(crate::FieldReader<bool, EWRN_A>);
impl EWRN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWRN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWRN_A {
        match self.bits {
            false => EWRN_A::VALUE1,
            true => EWRN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EWRN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == EWRN_A::VALUE2
    }
}
impl core::ops::Deref for EWRN_R {
    type Target = crate::FieldReader<bool, EWRN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bus-off Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFF_A {
    #[doc = "0: CAN controller is not in the bus-off state."]
    VALUE1 = 0,
    #[doc = "1: CAN controller is in the bus-off state."]
    VALUE2 = 1,
}
impl From<BOFF_A> for bool {
    #[inline(always)]
    fn from(variant: BOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFF` reader - Bus-off Status"]
pub struct BOFF_R(crate::FieldReader<bool, BOFF_A>);
impl BOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFF_A {
        match self.bits {
            false => BOFF_A::VALUE1,
            true => BOFF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BOFF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BOFF_A::VALUE2
    }
}
impl core::ops::Deref for BOFF_R {
    type Target = crate::FieldReader<bool, BOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "List Length Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLE_A {
    #[doc = "0: No List Length Error since last (most recent) flag reset."]
    VALUE1 = 0,
    #[doc = "1: A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    VALUE2 = 1,
}
impl From<LLE_A> for bool {
    #[inline(always)]
    fn from(variant: LLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LLE` reader - List Length Error"]
pub struct LLE_R(crate::FieldReader<bool, LLE_A>);
impl LLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLE_A {
        match self.bits {
            false => LLE_A::VALUE1,
            true => LLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LLE_A::VALUE2
    }
}
impl core::ops::Deref for LLE_R {
    type Target = crate::FieldReader<bool, LLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLE` writer - List Length Error"]
pub struct LLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No List Length Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LLE_A::VALUE1)
    }
    #[doc = "A List Length Error has been detected during message acceptance filtering. The number of elements in the list that belongs to this CAN node differs from the list SIZE given in the list termination pointer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LLE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "List Object Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOE_A {
    #[doc = "0: No List Object Error since last (most recent) flag reset."]
    VALUE1 = 0,
    #[doc = "1: A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    VALUE2 = 1,
}
impl From<LOE_A> for bool {
    #[inline(always)]
    fn from(variant: LOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOE` reader - List Object Error"]
pub struct LOE_R(crate::FieldReader<bool, LOE_A>);
impl LOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOE_A {
        match self.bits {
            false => LOE_A::VALUE1,
            true => LOE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LOE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LOE_A::VALUE2
    }
}
impl core::ops::Deref for LOE_R {
    type Target = crate::FieldReader<bool, LOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOE` writer - List Object Error"]
pub struct LOE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No List Object Error since last (most recent) flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOE_A::VALUE1)
    }
    #[doc = "A List Object Error has been detected during message acceptance filtering. A message object with wrong LIST index entry in the Message Object Status Register has been detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline(always)]
    pub fn txok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline(always)]
    pub fn rxok(&self) -> RXOK_R {
        RXOK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error Warning Status"]
    #[inline(always)]
    pub fn ewrn(&self) -> EWRN_R {
        EWRN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus-off Status"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline(always)]
    pub fn lle(&self) -> LLE_R {
        LLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline(always)]
    pub fn loe(&self) -> LOE_R {
        LOE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    #[doc = "Bit 3 - Message Transmitted Successfully"]
    #[inline(always)]
    pub fn txok(&mut self) -> TXOK_W {
        TXOK_W { w: self }
    }
    #[doc = "Bit 4 - Message Received Successfully"]
    #[inline(always)]
    pub fn rxok(&mut self) -> RXOK_W {
        RXOK_W { w: self }
    }
    #[doc = "Bit 5 - Alert Warning"]
    #[inline(always)]
    pub fn alert(&mut self) -> ALERT_W {
        ALERT_W { w: self }
    }
    #[doc = "Bit 8 - List Length Error"]
    #[inline(always)]
    pub fn lle(&mut self) -> LLE_W {
        LLE_W { w: self }
    }
    #[doc = "Bit 9 - List Object Error"]
    #[inline(always)]
    pub fn loe(&mut self) -> LOE_W {
        LOE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsr](index.html) module"]
pub struct NSR_SPEC;
impl crate::RegisterSpec for NSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nsr::R](R) reader structure"]
impl crate::Readable for NSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nsr::W](W) writer structure"]
impl crate::Writable for NSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NSR to value 0"]
impl crate::Resettable for NSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
