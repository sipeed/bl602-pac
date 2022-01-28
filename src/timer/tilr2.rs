#[doc = "Register `TILR2` reader"]
pub struct R(crate::R<TILR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TILR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TILR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TILR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TILR2` writer"]
pub struct W(crate::W<TILR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TILR2_SPEC>;
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
impl From<crate::W<TILR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TILR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tilr_2` reader - "]
pub struct TILR_2_R(crate::FieldReader<bool, bool>);
impl TILR_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TILR_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TILR_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tilr_2` writer - "]
pub struct TILR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TILR_2_W<'a> {
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
#[doc = "Field `tilr_1` reader - "]
pub struct TILR_1_R(crate::FieldReader<bool, bool>);
impl TILR_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TILR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TILR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tilr_1` writer - "]
pub struct TILR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TILR_1_W<'a> {
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
#[doc = "Field `tilr_0` reader - "]
pub struct TILR_0_R(crate::FieldReader<bool, bool>);
impl TILR_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TILR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TILR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tilr_0` writer - "]
pub struct TILR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TILR_0_W<'a> {
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
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tilr_2(&self) -> TILR_2_R {
        TILR_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tilr_1(&self) -> TILR_1_R {
        TILR_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tilr_0(&self) -> TILR_0_R {
        TILR_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tilr_2(&mut self) -> TILR_2_W {
        TILR_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tilr_1(&mut self) -> TILR_1_W {
        TILR_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tilr_0(&mut self) -> TILR_0_W {
        TILR_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TILR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tilr2](index.html) module"]
pub struct TILR2_SPEC;
impl crate::RegisterSpec for TILR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tilr2::R](R) reader structure"]
impl crate::Readable for TILR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tilr2::W](W) writer structure"]
impl crate::Writable for TILR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TILR2 to value 0"]
impl crate::Resettable for TILR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
