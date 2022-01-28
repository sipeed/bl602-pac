#[doc = "Register `HBN_TIME_H` reader"]
pub struct R(crate::R<HBN_TIME_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_TIME_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_TIME_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_TIME_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_TIME_H` writer"]
pub struct W(crate::W<HBN_TIME_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_TIME_H_SPEC>;
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
impl From<crate::W<HBN_TIME_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_TIME_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hbn_time_h` reader - "]
pub struct HBN_TIME_H_R(crate::FieldReader<u8, u8>);
impl HBN_TIME_H_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBN_TIME_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_TIME_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_time_h` writer - "]
pub struct HBN_TIME_H_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_TIME_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hbn_time_h(&self) -> HBN_TIME_H_R {
        HBN_TIME_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hbn_time_h(&mut self) -> HBN_TIME_H_W {
        HBN_TIME_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_TIME_H.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_time_h](index.html) module"]
pub struct HBN_TIME_H_SPEC;
impl crate::RegisterSpec for HBN_TIME_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_time_h::R](R) reader structure"]
impl crate::Readable for HBN_TIME_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_time_h::W](W) writer structure"]
impl crate::Writable for HBN_TIME_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_TIME_H to value 0"]
impl crate::Resettable for HBN_TIME_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
