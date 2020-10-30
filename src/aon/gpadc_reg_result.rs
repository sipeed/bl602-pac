#[doc = "Register `gpadc_reg_result` reader"]
pub struct R(crate::R<GPADC_REG_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPADC_REG_RESULT_SPEC>> for R {
    fn from(reader: crate::R<GPADC_REG_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_result` writer"]
pub struct W(crate::W<GPADC_REG_RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_RESULT_SPEC>;
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
impl core::convert::From<crate::W<GPADC_REG_RESULT_SPEC>> for W {
    fn from(writer: crate::W<GPADC_REG_RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_data_out` reader - "]
pub struct GPADC_DATA_OUT_R(crate::FieldReader<u32, u32>);
impl GPADC_DATA_OUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        GPADC_DATA_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_DATA_OUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_data_out` writer - "]
pub struct GPADC_DATA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DATA_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_data_out(&self) -> GPADC_DATA_OUT_R {
        GPADC_DATA_OUT_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_data_out(&mut self) -> GPADC_DATA_OUT_W {
        GPADC_DATA_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_result.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_result](index.html) module"]
pub struct GPADC_REG_RESULT_SPEC;
impl crate::RegisterSpec for GPADC_REG_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_result::R](R) reader structure"]
impl crate::Readable for GPADC_REG_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_result::W](W) writer structure"]
impl crate::Writable for GPADC_REG_RESULT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_reg_result to value 0"]
impl crate::Resettable for GPADC_REG_RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
