#[doc = "Register `DMA_RawIntErrorStatus` reader"]
pub struct R(crate::R<DMA_RAWINTERRORSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RAWINTERRORSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA_RAWINTERRORSTATUS_SPEC>> for R {
    fn from(reader: crate::R<DMA_RAWINTERRORSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RawIntErrorStatus` writer"]
pub struct W(crate::W<DMA_RAWINTERRORSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RAWINTERRORSTATUS_SPEC>;
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
impl core::convert::From<crate::W<DMA_RAWINTERRORSTATUS_SPEC>> for W {
    fn from(writer: crate::W<DMA_RAWINTERRORSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RawIntErrorStatus` reader - "]
pub struct RAWINTERRORSTATUS_R(crate::FieldReader<u8, u8>);
impl RAWINTERRORSTATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RAWINTERRORSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTERRORSTATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RawIntErrorStatus` writer - "]
pub struct RAWINTERRORSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAWINTERRORSTATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn raw_int_error_status(&self) -> RAWINTERRORSTATUS_R {
        RAWINTERRORSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn raw_int_error_status(&mut self) -> RAWINTERRORSTATUS_W {
        RAWINTERRORSTATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_RawIntErrorStatus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_raw_int_error_status](index.html) module"]
pub struct DMA_RAWINTERRORSTATUS_SPEC;
impl crate::RegisterSpec for DMA_RAWINTERRORSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_raw_int_error_status::R](R) reader structure"]
impl crate::Readable for DMA_RAWINTERRORSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_raw_int_error_status::W](W) writer structure"]
impl crate::Writable for DMA_RAWINTERRORSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_RawIntErrorStatus to value 0"]
impl crate::Resettable for DMA_RAWINTERRORSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
