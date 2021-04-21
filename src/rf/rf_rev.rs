#[doc = "Register `rf_rev` reader"]
pub struct R(crate::R<RF_REV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_REV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_REV_SPEC>> for R {
    fn from(reader: crate::R<RF_REV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_rev` writer"]
pub struct W(crate::W<RF_REV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_REV_SPEC>;
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
impl core::convert::From<crate::W<RF_REV_SPEC>> for W {
    fn from(writer: crate::W<RF_REV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hw_rev` reader - "]
pub struct HW_REV_R(crate::FieldReader<u8, u8>);
impl HW_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        HW_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hw_rev` writer - "]
pub struct HW_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `fw_rev` reader - "]
pub struct FW_REV_R(crate::FieldReader<u8, u8>);
impl FW_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        FW_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FW_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fw_rev` writer - "]
pub struct FW_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `rf_id` reader - "]
pub struct RF_ID_R(crate::FieldReader<u8, u8>);
impl RF_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_id` writer - "]
pub struct RF_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hw_rev(&self) -> HW_REV_R {
        HW_REV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn fw_rev(&self) -> FW_REV_R {
        FW_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rf_id(&self) -> RF_ID_R {
        RF_ID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hw_rev(&mut self) -> HW_REV_W {
        HW_REV_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn fw_rev(&mut self) -> FW_REV_W {
        FW_REV_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rf_id(&mut self) -> RF_ID_W {
        RF_ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Silicon revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_rev](index.html) module"]
pub struct RF_REV_SPEC;
impl crate::RegisterSpec for RF_REV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_rev::R](R) reader structure"]
impl crate::Readable for RF_REV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_rev::W](W) writer structure"]
impl crate::Writable for RF_REV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_rev to value 0"]
impl crate::Resettable for RF_REV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
