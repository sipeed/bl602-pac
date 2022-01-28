#[doc = "Register `temp_comp` reader"]
pub struct R(crate::R<TEMP_COMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMP_COMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMP_COMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMP_COMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `temp_comp` writer"]
pub struct W(crate::W<TEMP_COMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMP_COMP_SPEC>;
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
impl From<crate::W<TEMP_COMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMP_COMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `temp_comp_en` reader - "]
pub struct TEMP_COMP_EN_R(crate::FieldReader<bool, bool>);
impl TEMP_COMP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEMP_COMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEMP_COMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `temp_comp_en` writer - "]
pub struct TEMP_COMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_COMP_EN_W<'a> {
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
#[doc = "Field `const_fcal` reader - "]
pub struct CONST_FCAL_R(crate::FieldReader<u8, u8>);
impl CONST_FCAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONST_FCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONST_FCAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `const_fcal` writer - "]
pub struct CONST_FCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONST_FCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `const_acal` reader - "]
pub struct CONST_ACAL_R(crate::FieldReader<u8, u8>);
impl CONST_ACAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONST_ACAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONST_ACAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `const_acal` writer - "]
pub struct CONST_ACAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONST_ACAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn temp_comp_en(&self) -> TEMP_COMP_EN_R {
        TEMP_COMP_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn const_fcal(&self) -> CONST_FCAL_R {
        CONST_FCAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn const_acal(&self) -> CONST_ACAL_R {
        CONST_ACAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn temp_comp_en(&mut self) -> TEMP_COMP_EN_W {
        TEMP_COMP_EN_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn const_fcal(&mut self) -> CONST_FCAL_W {
        CONST_FCAL_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn const_acal(&mut self) -> CONST_ACAL_W {
        CONST_ACAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "temp_comp.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_comp](index.html) module"]
pub struct TEMP_COMP_SPEC;
impl crate::RegisterSpec for TEMP_COMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [temp_comp::R](R) reader structure"]
impl crate::Readable for TEMP_COMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [temp_comp::W](W) writer structure"]
impl crate::Writable for TEMP_COMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets temp_comp to value 0"]
impl crate::Resettable for TEMP_COMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
