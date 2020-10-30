#[doc = "Register `TMSR2` reader"]
pub struct R(crate::R<TMSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TMSR2_SPEC>> for R {
    fn from(reader: crate::R<TMSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMSR2` writer"]
pub struct W(crate::W<TMSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMSR2_SPEC>;
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
impl core::convert::From<crate::W<TMSR2_SPEC>> for W {
    fn from(writer: crate::W<TMSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmsr_2` reader - "]
pub struct TMSR_2_R(crate::FieldReader<bool, bool>);
impl TMSR_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMSR_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMSR_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmsr_2` writer - "]
pub struct TMSR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TMSR_2_W<'a> {
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
#[doc = "Field `tmsr_1` reader - "]
pub struct TMSR_1_R(crate::FieldReader<bool, bool>);
impl TMSR_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMSR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMSR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmsr_1` writer - "]
pub struct TMSR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMSR_1_W<'a> {
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
#[doc = "Field `tmsr_0` reader - "]
pub struct TMSR_0_R(crate::FieldReader<bool, bool>);
impl TMSR_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMSR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMSR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmsr_0` writer - "]
pub struct TMSR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMSR_0_W<'a> {
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
    pub fn tmsr_2(&self) -> TMSR_2_R {
        TMSR_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmsr_1(&self) -> TMSR_1_R {
        TMSR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmsr_0(&self) -> TMSR_0_R {
        TMSR_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tmsr_2(&mut self) -> TMSR_2_W {
        TMSR_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmsr_1(&mut self) -> TMSR_1_W {
        TMSR_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmsr_0(&mut self) -> TMSR_0_W {
        TMSR_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMSR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmsr2](index.html) module"]
pub struct TMSR2_SPEC;
impl crate::RegisterSpec for TMSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmsr2::R](R) reader structure"]
impl crate::Readable for TMSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmsr2::W](W) writer structure"]
impl crate::Writable for TMSR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMSR2 to value 0"]
impl crate::Resettable for TMSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
