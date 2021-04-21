#[doc = "Register `gpadc_reg_status` reader"]
pub struct R(crate::R<GPADC_REG_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPADC_REG_STATUS_SPEC>> for R {
    fn from(reader: crate::R<GPADC_REG_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_status` writer"]
pub struct W(crate::W<GPADC_REG_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_STATUS_SPEC>;
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
impl core::convert::From<crate::W<GPADC_REG_STATUS_SPEC>> for W {
    fn from(writer: crate::W<GPADC_REG_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_reserved` reader - "]
pub struct GPADC_RESERVED_R(crate::FieldReader<u16, u16>);
impl GPADC_RESERVED_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPADC_RESERVED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_RESERVED_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_reserved` writer - "]
pub struct GPADC_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RESERVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `gpadc_data_rdy` reader - "]
pub struct GPADC_DATA_RDY_R(crate::FieldReader<bool, bool>);
impl GPADC_DATA_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_DATA_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_DATA_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn gpadc_reserved(&self) -> GPADC_RESERVED_R {
        GPADC_RESERVED_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_data_rdy(&self) -> GPADC_DATA_RDY_R {
        GPADC_DATA_RDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn gpadc_reserved(&mut self) -> GPADC_RESERVED_W {
        GPADC_RESERVED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_status](index.html) module"]
pub struct GPADC_REG_STATUS_SPEC;
impl crate::RegisterSpec for GPADC_REG_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_status::R](R) reader structure"]
impl crate::Readable for GPADC_REG_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_status::W](W) writer structure"]
impl crate::Writable for GPADC_REG_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_reg_status to value 0"]
impl crate::Resettable for GPADC_REG_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
