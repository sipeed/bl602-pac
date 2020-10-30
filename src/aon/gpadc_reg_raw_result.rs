#[doc = "Register `gpadc_reg_raw_result` reader"]
pub struct R(crate::R<GPADC_REG_RAW_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_RAW_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPADC_REG_RAW_RESULT_SPEC>> for R {
    fn from(reader: crate::R<GPADC_REG_RAW_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_raw_result` writer"]
pub struct W(crate::W<GPADC_REG_RAW_RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_RAW_RESULT_SPEC>;
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
impl core::convert::From<crate::W<GPADC_REG_RAW_RESULT_SPEC>> for W {
    fn from(writer: crate::W<GPADC_REG_RAW_RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_raw_data` reader - "]
pub struct GPADC_RAW_DATA_R(crate::FieldReader<u16, u16>);
impl GPADC_RAW_DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPADC_RAW_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_RAW_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_raw_data` writer - "]
pub struct GPADC_RAW_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RAW_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpadc_raw_data(&self) -> GPADC_RAW_DATA_R {
        GPADC_RAW_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpadc_raw_data(&mut self) -> GPADC_RAW_DATA_W {
        GPADC_RAW_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_raw_result.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_raw_result](index.html) module"]
pub struct GPADC_REG_RAW_RESULT_SPEC;
impl crate::RegisterSpec for GPADC_REG_RAW_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_raw_result::R](R) reader structure"]
impl crate::Readable for GPADC_REG_RAW_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_raw_result::W](W) writer structure"]
impl crate::Writable for GPADC_REG_RAW_RESULT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_reg_raw_result to value 0"]
impl crate::Resettable for GPADC_REG_RAW_RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
