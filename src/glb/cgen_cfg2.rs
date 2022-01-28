#[doc = "Register `cgen_cfg2` reader"]
pub struct R(crate::R<CGEN_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGEN_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGEN_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg2` writer"]
pub struct W(crate::W<CGEN_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG2_SPEC>;
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
impl From<crate::W<CGEN_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGEN_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cgen_s3` reader - "]
pub struct CGEN_S3_R(crate::FieldReader<bool, bool>);
impl CGEN_S3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CGEN_S3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGEN_S3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cgen_s3` writer - "]
pub struct CGEN_S3_W<'a> {
    w: &'a mut W,
}
impl<'a> CGEN_S3_W<'a> {
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
#[doc = "Field `cgen_s2` reader - "]
pub struct CGEN_S2_R(crate::FieldReader<bool, bool>);
impl CGEN_S2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CGEN_S2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGEN_S2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cgen_s2` writer - "]
pub struct CGEN_S2_W<'a> {
    w: &'a mut W,
}
impl<'a> CGEN_S2_W<'a> {
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
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s3(&self) -> CGEN_S3_R {
        CGEN_S3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s2(&self) -> CGEN_S2_R {
        CGEN_S2_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_s3(&mut self) -> CGEN_S3_W {
        CGEN_S3_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_s2(&mut self) -> CGEN_S2_W {
        CGEN_S2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_cfg2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg2](index.html) module"]
pub struct CGEN_CFG2_SPEC;
impl crate::RegisterSpec for CGEN_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg2::R](R) reader structure"]
impl crate::Readable for CGEN_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg2::W](W) writer structure"]
impl crate::Writable for CGEN_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cgen_cfg2 to value 0x11"]
impl crate::Resettable for CGEN_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x11
    }
}
