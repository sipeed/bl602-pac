#[doc = "Register `gpdac_data` reader"]
pub struct R(crate::R<GPDAC_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_data` writer"]
pub struct W(crate::W<GPDAC_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_DATA_SPEC>;
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
impl From<crate::W<GPDAC_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDAC_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpdac_a_data` reader - "]
pub struct GPDAC_A_DATA_R(crate::FieldReader<u16, u16>);
impl GPDAC_A_DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPDAC_A_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_A_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_a_data` writer - "]
pub struct GPDAC_A_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_A_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `gpdac_b_data` reader - "]
pub struct GPDAC_B_DATA_R(crate::FieldReader<u16, u16>);
impl GPDAC_B_DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPDAC_B_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_B_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_b_data` writer - "]
pub struct GPDAC_B_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_B_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn gpdac_a_data(&self) -> GPDAC_A_DATA_R {
        GPDAC_A_DATA_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn gpdac_b_data(&self) -> GPDAC_B_DATA_R {
        GPDAC_B_DATA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn gpdac_a_data(&mut self) -> GPDAC_A_DATA_W {
        GPDAC_A_DATA_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn gpdac_b_data(&mut self) -> GPDAC_B_DATA_W {
        GPDAC_B_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpdac_data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_data](index.html) module"]
pub struct GPDAC_DATA_SPEC;
impl crate::RegisterSpec for GPDAC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_data::R](R) reader structure"]
impl crate::Readable for GPDAC_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_data::W](W) writer structure"]
impl crate::Writable for GPDAC_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpdac_data to value 0"]
impl crate::Resettable for GPDAC_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
