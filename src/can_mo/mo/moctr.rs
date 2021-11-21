#[doc = "Register `MOCTR` writer"]
pub struct W(crate::W<MOCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOCTR_SPEC>;
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
impl From<crate::W<MOCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESRXPND` writer - Reset/Set Receive Pending"]
pub struct RESRXPND_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRXPND_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SETRXPND` writer - Reset/Set Receive Pending"]
pub struct SETRXPND_W<'a> {
    w: &'a mut W,
}
impl<'a> SETRXPND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `RESTXPND` writer - Reset/Set Transmit Pending"]
pub struct RESTXPND_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTXPND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SETTXPND` writer - Reset/Set Transmit Pending"]
pub struct SETTXPND_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTXPND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RESRXUPD` writer - Reset/Set Receive Updating"]
pub struct RESRXUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRXUPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SETRXUPD` writer - Reset/Set Receive Updating"]
pub struct SETRXUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SETRXUPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RESNEWDAT` writer - Reset/Set New Data"]
pub struct RESNEWDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESNEWDAT_W<'a> {
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
#[doc = "Field `SETNEWDAT` writer - Reset/Set New Data"]
pub struct SETNEWDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SETNEWDAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RESMSGLST` writer - Reset/Set Message Lost"]
pub struct RESMSGLST_W<'a> {
    w: &'a mut W,
}
impl<'a> RESMSGLST_W<'a> {
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
#[doc = "Field `SETMSGLST` writer - Reset/Set Message Lost"]
pub struct SETMSGLST_W<'a> {
    w: &'a mut W,
}
impl<'a> SETMSGLST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `RESMSGVAL` writer - Reset/Set Message Valid"]
pub struct RESMSGVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESMSGVAL_W<'a> {
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
#[doc = "Field `SETMSGVAL` writer - Reset/Set Message Valid"]
pub struct SETMSGVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SETMSGVAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `RESRTSEL` writer - Reset/Set Receive/Transmit Selected"]
pub struct RESRTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRTSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `SETRTSEL` writer - Reset/Set Receive/Transmit Selected"]
pub struct SETRTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SETRTSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RESRXEN` writer - Reset/Set Receive Enable"]
pub struct RESRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESRXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SETRXEN` writer - Reset/Set Receive Enable"]
pub struct SETRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SETRXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `RESTXRQ` writer - Reset/Set Transmit Request"]
pub struct RESTXRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTXRQ_W<'a> {
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
#[doc = "Field `SETTXRQ` writer - Reset/Set Transmit Request"]
pub struct SETTXRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTXRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RESTXEN0` writer - Reset/Set Transmit Enable 0"]
pub struct RESTXEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTXEN0_W<'a> {
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
#[doc = "Field `SETTXEN0` writer - Reset/Set Transmit Enable 0"]
pub struct SETTXEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTXEN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `RESTXEN1` writer - Reset/Set Transmit Enable 1"]
pub struct RESTXEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTXEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SETTXEN1` writer - Reset/Set Transmit Enable 1"]
pub struct SETTXEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTXEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `RESDIR` writer - Reset/Set Message Direction"]
pub struct RESDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SETDIR` writer - Reset/Set Message Direction"]
pub struct SETDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> SETDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Reset/Set Receive Pending"]
    #[inline(always)]
    pub fn resrxpnd(&mut self) -> RESRXPND_W {
        RESRXPND_W { w: self }
    }
    #[doc = "Bit 16 - Reset/Set Receive Pending"]
    #[inline(always)]
    pub fn setrxpnd(&mut self) -> SETRXPND_W {
        SETRXPND_W { w: self }
    }
    #[doc = "Bit 1 - Reset/Set Transmit Pending"]
    #[inline(always)]
    pub fn restxpnd(&mut self) -> RESTXPND_W {
        RESTXPND_W { w: self }
    }
    #[doc = "Bit 17 - Reset/Set Transmit Pending"]
    #[inline(always)]
    pub fn settxpnd(&mut self) -> SETTXPND_W {
        SETTXPND_W { w: self }
    }
    #[doc = "Bit 2 - Reset/Set Receive Updating"]
    #[inline(always)]
    pub fn resrxupd(&mut self) -> RESRXUPD_W {
        RESRXUPD_W { w: self }
    }
    #[doc = "Bit 18 - Reset/Set Receive Updating"]
    #[inline(always)]
    pub fn setrxupd(&mut self) -> SETRXUPD_W {
        SETRXUPD_W { w: self }
    }
    #[doc = "Bit 3 - Reset/Set New Data"]
    #[inline(always)]
    pub fn resnewdat(&mut self) -> RESNEWDAT_W {
        RESNEWDAT_W { w: self }
    }
    #[doc = "Bit 19 - Reset/Set New Data"]
    #[inline(always)]
    pub fn setnewdat(&mut self) -> SETNEWDAT_W {
        SETNEWDAT_W { w: self }
    }
    #[doc = "Bit 4 - Reset/Set Message Lost"]
    #[inline(always)]
    pub fn resmsglst(&mut self) -> RESMSGLST_W {
        RESMSGLST_W { w: self }
    }
    #[doc = "Bit 20 - Reset/Set Message Lost"]
    #[inline(always)]
    pub fn setmsglst(&mut self) -> SETMSGLST_W {
        SETMSGLST_W { w: self }
    }
    #[doc = "Bit 5 - Reset/Set Message Valid"]
    #[inline(always)]
    pub fn resmsgval(&mut self) -> RESMSGVAL_W {
        RESMSGVAL_W { w: self }
    }
    #[doc = "Bit 21 - Reset/Set Message Valid"]
    #[inline(always)]
    pub fn setmsgval(&mut self) -> SETMSGVAL_W {
        SETMSGVAL_W { w: self }
    }
    #[doc = "Bit 6 - Reset/Set Receive/Transmit Selected"]
    #[inline(always)]
    pub fn resrtsel(&mut self) -> RESRTSEL_W {
        RESRTSEL_W { w: self }
    }
    #[doc = "Bit 22 - Reset/Set Receive/Transmit Selected"]
    #[inline(always)]
    pub fn setrtsel(&mut self) -> SETRTSEL_W {
        SETRTSEL_W { w: self }
    }
    #[doc = "Bit 7 - Reset/Set Receive Enable"]
    #[inline(always)]
    pub fn resrxen(&mut self) -> RESRXEN_W {
        RESRXEN_W { w: self }
    }
    #[doc = "Bit 23 - Reset/Set Receive Enable"]
    #[inline(always)]
    pub fn setrxen(&mut self) -> SETRXEN_W {
        SETRXEN_W { w: self }
    }
    #[doc = "Bit 8 - Reset/Set Transmit Request"]
    #[inline(always)]
    pub fn restxrq(&mut self) -> RESTXRQ_W {
        RESTXRQ_W { w: self }
    }
    #[doc = "Bit 24 - Reset/Set Transmit Request"]
    #[inline(always)]
    pub fn settxrq(&mut self) -> SETTXRQ_W {
        SETTXRQ_W { w: self }
    }
    #[doc = "Bit 9 - Reset/Set Transmit Enable 0"]
    #[inline(always)]
    pub fn restxen0(&mut self) -> RESTXEN0_W {
        RESTXEN0_W { w: self }
    }
    #[doc = "Bit 25 - Reset/Set Transmit Enable 0"]
    #[inline(always)]
    pub fn settxen0(&mut self) -> SETTXEN0_W {
        SETTXEN0_W { w: self }
    }
    #[doc = "Bit 10 - Reset/Set Transmit Enable 1"]
    #[inline(always)]
    pub fn restxen1(&mut self) -> RESTXEN1_W {
        RESTXEN1_W { w: self }
    }
    #[doc = "Bit 26 - Reset/Set Transmit Enable 1"]
    #[inline(always)]
    pub fn settxen1(&mut self) -> SETTXEN1_W {
        SETTXEN1_W { w: self }
    }
    #[doc = "Bit 11 - Reset/Set Message Direction"]
    #[inline(always)]
    pub fn resdir(&mut self) -> RESDIR_W {
        RESDIR_W { w: self }
    }
    #[doc = "Bit 27 - Reset/Set Message Direction"]
    #[inline(always)]
    pub fn setdir(&mut self) -> SETDIR_W {
        SETDIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moctr](index.html) module"]
pub struct MOCTR_SPEC;
impl crate::RegisterSpec for MOCTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [moctr::W](W) writer structure"]
impl crate::Writable for MOCTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOCTR to value 0"]
impl crate::Resettable for MOCTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
