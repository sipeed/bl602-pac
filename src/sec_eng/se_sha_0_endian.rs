#[doc = "Register `se_sha_0_endian` reader"]
pub struct R(crate::R<SE_SHA_0_ENDIAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_SHA_0_ENDIAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_SHA_0_ENDIAN_SPEC>> for R {
    fn from(reader: crate::R<SE_SHA_0_ENDIAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_sha_0_endian` writer"]
pub struct W(crate::W<SE_SHA_0_ENDIAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_SHA_0_ENDIAN_SPEC>;
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
impl core::convert::From<crate::W<SE_SHA_0_ENDIAN_SPEC>> for W {
    fn from(writer: crate::W<SE_SHA_0_ENDIAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_sha_0_dout_endian` reader - "]
pub struct SE_SHA_0_DOUT_ENDIAN_R(crate::FieldReader<bool, bool>);
impl SE_SHA_0_DOUT_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_0_DOUT_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_DOUT_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_dout_endian` writer - "]
pub struct SE_SHA_0_DOUT_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_DOUT_ENDIAN_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_0_dout_endian(&self) -> SE_SHA_0_DOUT_ENDIAN_R {
        SE_SHA_0_DOUT_ENDIAN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_0_dout_endian(&mut self) -> SE_SHA_0_DOUT_ENDIAN_W {
        SE_SHA_0_DOUT_ENDIAN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_sha_0_endian.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_endian](index.html) module"]
pub struct SE_SHA_0_ENDIAN_SPEC;
impl crate::RegisterSpec for SE_SHA_0_ENDIAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_sha_0_endian::R](R) reader structure"]
impl crate::Readable for SE_SHA_0_ENDIAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_sha_0_endian::W](W) writer structure"]
impl crate::Writable for SE_SHA_0_ENDIAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_sha_0_endian to value 0x01"]
impl crate::Resettable for SE_SHA_0_ENDIAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
