#[doc = "Register `irrx_data_word1` reader"]
pub struct R(crate::R<IRRX_DATA_WORD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_DATA_WORD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IRRX_DATA_WORD1_SPEC>> for R {
    fn from(reader: crate::R<IRRX_DATA_WORD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irrx_data_word1` writer"]
pub struct W(crate::W<IRRX_DATA_WORD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRRX_DATA_WORD1_SPEC>;
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
impl core::convert::From<crate::W<IRRX_DATA_WORD1_SPEC>> for W {
    fn from(writer: crate::W<IRRX_DATA_WORD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_irrx_data_word1` reader - "]
pub struct STS_IRRX_DATA_WORD1_R(crate::FieldReader<u32, u32>);
impl STS_IRRX_DATA_WORD1_R {
    pub(crate) fn new(bits: u32) -> Self {
        STS_IRRX_DATA_WORD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_IRRX_DATA_WORD1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_irrx_data_word1` writer - "]
pub struct STS_IRRX_DATA_WORD1_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_IRRX_DATA_WORD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_irrx_data_word1(&self) -> STS_IRRX_DATA_WORD1_R {
        STS_IRRX_DATA_WORD1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_irrx_data_word1(&mut self) -> STS_IRRX_DATA_WORD1_W {
        STS_IRRX_DATA_WORD1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irrx_data_word1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_data_word1](index.html) module"]
pub struct IRRX_DATA_WORD1_SPEC;
impl crate::RegisterSpec for IRRX_DATA_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_data_word1::R](R) reader structure"]
impl crate::Readable for IRRX_DATA_WORD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irrx_data_word1::W](W) writer structure"]
impl crate::Writable for IRRX_DATA_WORD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irrx_data_word1 to value 0"]
impl crate::Resettable for IRRX_DATA_WORD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
