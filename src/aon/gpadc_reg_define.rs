#[doc = "Register `gpadc_reg_define` reader"]
pub struct R(crate::R<GPADC_REG_DEFINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_DEFINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_DEFINE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_DEFINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_define` writer"]
pub struct W(crate::W<GPADC_REG_DEFINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_DEFINE_SPEC>;
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
impl From<crate::W<GPADC_REG_DEFINE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_DEFINE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_os_cal_data` reader - "]
pub struct GPADC_OS_CAL_DATA_R(crate::FieldReader<u16, u16>);
impl GPADC_OS_CAL_DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPADC_OS_CAL_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_OS_CAL_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_os_cal_data` writer - "]
pub struct GPADC_OS_CAL_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_OS_CAL_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gpadc_os_cal_data(&self) -> GPADC_OS_CAL_DATA_R {
        GPADC_OS_CAL_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gpadc_os_cal_data(&mut self) -> GPADC_OS_CAL_DATA_W {
        GPADC_OS_CAL_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_define.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_define](index.html) module"]
pub struct GPADC_REG_DEFINE_SPEC;
impl crate::RegisterSpec for GPADC_REG_DEFINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_define::R](R) reader structure"]
impl crate::Readable for GPADC_REG_DEFINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_define::W](W) writer structure"]
impl crate::Writable for GPADC_REG_DEFINE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_reg_define to value 0"]
impl crate::Resettable for GPADC_REG_DEFINE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
