#[doc = "Register `TICR3` writer"]
pub struct W(crate::W<TICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TICR3_SPEC>;
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
impl From<crate::W<TICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tclr_2` writer - "]
pub struct TCLR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_2_W<'a> {
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
#[doc = "Field `tclr_1` writer - "]
pub struct TCLR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_1_W<'a> {
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
#[doc = "Field `tclr_0` writer - "]
pub struct TCLR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_0_W<'a> {
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
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tclr_2(&mut self) -> TCLR_2_W {
        TCLR_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tclr_1(&mut self) -> TCLR_1_W {
        TCLR_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tclr_0(&mut self) -> TCLR_0_W {
        TCLR_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TICR3.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ticr3](index.html) module"]
pub struct TICR3_SPEC;
impl crate::RegisterSpec for TICR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ticr3::W](W) writer structure"]
impl crate::Writable for TICR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TICR3 to value 0"]
impl crate::Resettable for TICR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
