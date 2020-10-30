#[doc = "Register `PDS_TIME1` reader"]
pub struct R(crate::R<PDS_TIME1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_TIME1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDS_TIME1_SPEC>> for R {
    fn from(reader: crate::R<PDS_TIME1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_TIME1` writer"]
pub struct W(crate::W<PDS_TIME1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_TIME1_SPEC>;
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
impl core::convert::From<crate::W<PDS_TIME1_SPEC>> for W {
    fn from(writer: crate::W<PDS_TIME1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_sleep_duration` reader - "]
pub struct CR_SLEEP_DURATION_R(crate::FieldReader<u32, u32>);
impl CR_SLEEP_DURATION_R {
    pub(crate) fn new(bits: u32) -> Self {
        CR_SLEEP_DURATION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SLEEP_DURATION_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_sleep_duration` writer - "]
pub struct CR_SLEEP_DURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SLEEP_DURATION_W<'a> {
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
    pub fn cr_sleep_duration(&self) -> CR_SLEEP_DURATION_R {
        CR_SLEEP_DURATION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_sleep_duration(&mut self) -> CR_SLEEP_DURATION_W {
        CR_SLEEP_DURATION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_TIME1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_time1](index.html) module"]
pub struct PDS_TIME1_SPEC;
impl crate::RegisterSpec for PDS_TIME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_time1::R](R) reader structure"]
impl crate::Readable for PDS_TIME1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_time1::W](W) writer structure"]
impl crate::Writable for PDS_TIME1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDS_TIME1 to value 0"]
impl crate::Resettable for PDS_TIME1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
