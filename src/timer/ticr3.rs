#[doc = "Register `TICR3` reader"]
pub struct R(crate::R<TICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TICR3_SPEC>> for R {
    fn from(reader: crate::R<TICR3_SPEC>) -> Self {
        R(reader)
    }
}
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
impl core::convert::From<crate::W<TICR3_SPEC>> for W {
    fn from(writer: crate::W<TICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tclr_2` reader - "]
pub struct TCLR_2_R(crate::FieldReader<bool, bool>);
impl TCLR_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCLR_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLR_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `tclr_1` reader - "]
pub struct TCLR_1_R(crate::FieldReader<bool, bool>);
impl TCLR_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCLR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `tclr_0` reader - "]
pub struct TCLR_0_R(crate::FieldReader<bool, bool>);
impl TCLR_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCLR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tclr_2(&self) -> TCLR_2_R {
        TCLR_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tclr_1(&self) -> TCLR_1_R {
        TCLR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tclr_0(&self) -> TCLR_0_R {
        TCLR_0_R::new((self.bits & 0x01) != 0)
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TICR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ticr3](index.html) module"]
pub struct TICR3_SPEC;
impl crate::RegisterSpec for TICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ticr3::R](R) reader structure"]
impl crate::Readable for TICR3_SPEC {
    type Reader = R;
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
