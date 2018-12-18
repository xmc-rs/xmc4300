#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPACR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CP10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP10R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    VALUE1,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    VALUE2,
    #[doc = "Full access."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP10R::VALUE1 => 0,
            CP10R::VALUE2 => 1,
            CP10R::VALUE4 => 3,
            CP10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP10R {
        match value {
            0 => CP10R::VALUE1,
            1 => CP10R::VALUE2,
            3 => CP10R::VALUE4,
            i => CP10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CP10R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CP10R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CP10R::VALUE4
    }
}
#[doc = "Possible values of the field `CP11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP11R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    VALUE1,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    VALUE2,
    #[doc = "Full access."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CP11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CP11R::VALUE1 => 0,
            CP11R::VALUE2 => 1,
            CP11R::VALUE4 => 3,
            CP11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CP11R {
        match value {
            0 => CP11R::VALUE1,
            1 => CP11R::VALUE2,
            3 => CP11R::VALUE4,
            i => CP11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CP11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CP11R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CP11R::VALUE4
    }
}
#[doc = "Values that can be written to the field `CP10`"]
pub enum CP10W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    VALUE1,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    VALUE2,
    #[doc = "Full access."]
    VALUE4,
}
impl CP10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP10W::VALUE1 => 0,
            CP10W::VALUE2 => 1,
            CP10W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP10W<'a> {
    w: &'a mut W,
}
impl<'a> _CP10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CP10W::VALUE1)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CP10W::VALUE2)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CP10W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CP11`"]
pub enum CP11W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    VALUE1,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    VALUE2,
    #[doc = "Full access."]
    VALUE4,
}
impl CP11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP11W::VALUE1 => 0,
            CP11W::VALUE2 => 1,
            CP11W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CP11W<'a> {
    w: &'a mut W,
}
impl<'a> _CP11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CP11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CP11W::VALUE1)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CP11W::VALUE2)
    }
    #[doc = "Full access."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CP11W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline]
    pub fn cp10(&self) -> CP10R {
        CP10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline]
    pub fn cp11(&self) -> CP11R {
        CP11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10"]
    #[inline]
    pub fn cp10(&mut self) -> _CP10W {
        _CP10W { w: self }
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11"]
    #[inline]
    pub fn cp11(&mut self) -> _CP11W {
        _CP11W { w: self }
    }
}
