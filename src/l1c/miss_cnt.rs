#[doc = "Register `miss_cnt` reader"]
pub struct R(crate::R<MISS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MISS_CNT_SPEC>> for R {
    fn from(reader: crate::R<MISS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `miss_cnt` writer"]
pub struct W(crate::W<MISS_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISS_CNT_SPEC>;
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
impl core::convert::From<crate::W<MISS_CNT_SPEC>> for W {
    fn from(writer: crate::W<MISS_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `miss_cnt` reader - "]
pub struct MISS_CNT_R(crate::FieldReader<u32, u32>);
impl MISS_CNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        MISS_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISS_CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `miss_cnt` writer - "]
pub struct MISS_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MISS_CNT_W<'a> {
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
    pub fn miss_cnt(&self) -> MISS_CNT_R {
        MISS_CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn miss_cnt(&mut self) -> MISS_CNT_W {
        MISS_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "miss_cnt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miss_cnt](index.html) module"]
pub struct MISS_CNT_SPEC;
impl crate::RegisterSpec for MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miss_cnt::R](R) reader structure"]
impl crate::Readable for MISS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miss_cnt::W](W) writer structure"]
impl crate::Writable for MISS_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets miss_cnt to value 0"]
impl crate::Resettable for MISS_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
