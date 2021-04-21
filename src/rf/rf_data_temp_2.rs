#[doc = "Register `rf_data_temp_2` reader"]
pub struct R(crate::R<RF_DATA_TEMP_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_DATA_TEMP_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_DATA_TEMP_2_SPEC>> for R {
    fn from(reader: crate::R<RF_DATA_TEMP_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_data_temp_2` writer"]
pub struct W(crate::W<RF_DATA_TEMP_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_DATA_TEMP_2_SPEC>;
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
impl core::convert::From<crate::W<RF_DATA_TEMP_2_SPEC>> for W {
    fn from(writer: crate::W<RF_DATA_TEMP_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_data_temp_2` reader - "]
pub struct RF_DATA_TEMP_2_R(crate::FieldReader<u32, u32>);
impl RF_DATA_TEMP_2_R {
    pub(crate) fn new(bits: u32) -> Self {
        RF_DATA_TEMP_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_DATA_TEMP_2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_data_temp_2` writer - "]
pub struct RF_DATA_TEMP_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_DATA_TEMP_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_2(&self) -> RF_DATA_TEMP_2_R {
        RF_DATA_TEMP_2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_data_temp_2(&mut self) -> RF_DATA_TEMP_2_W {
        RF_DATA_TEMP_2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_data_temp_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_data_temp_2](index.html) module"]
pub struct RF_DATA_TEMP_2_SPEC;
impl crate::RegisterSpec for RF_DATA_TEMP_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_data_temp_2::R](R) reader structure"]
impl crate::Readable for RF_DATA_TEMP_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_data_temp_2::W](W) writer structure"]
impl crate::Writable for RF_DATA_TEMP_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_data_temp_2 to value 0"]
impl crate::Resettable for RF_DATA_TEMP_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
