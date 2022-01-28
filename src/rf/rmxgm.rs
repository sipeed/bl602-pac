#[doc = "Register `rmxgm` reader"]
pub struct R(crate::R<RMXGM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMXGM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMXGM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMXGM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rmxgm` writer"]
pub struct W(crate::W<RMXGM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMXGM_SPEC>;
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
impl From<crate::W<RMXGM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMXGM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rmxgm_10m_mode_en` reader - "]
pub struct RMXGM_10M_MODE_EN_R(crate::FieldReader<bool, bool>);
impl RMXGM_10M_MODE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMXGM_10M_MODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMXGM_10M_MODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rmxgm_10m_mode_en` writer - "]
pub struct RMXGM_10M_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMXGM_10M_MODE_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `rmxgm_bm` reader - "]
pub struct RMXGM_BM_R(crate::FieldReader<u8, u8>);
impl RMXGM_BM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RMXGM_BM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMXGM_BM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rmxgm_bm` writer - "]
pub struct RMXGM_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMXGM_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `rmx_bm` reader - "]
pub struct RMX_BM_R(crate::FieldReader<u8, u8>);
impl RMX_BM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RMX_BM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMX_BM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rmx_bm` writer - "]
pub struct RMX_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMX_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rmxgm_10m_mode_en(&self) -> RMXGM_10M_MODE_EN_R {
        RMXGM_10M_MODE_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rmxgm_bm(&self) -> RMXGM_BM_R {
        RMXGM_BM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&self) -> RMX_BM_R {
        RMX_BM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rmxgm_10m_mode_en(&mut self) -> RMXGM_10M_MODE_EN_W {
        RMXGM_10M_MODE_EN_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rmxgm_bm(&mut self) -> RMXGM_BM_W {
        RMXGM_BM_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&mut self) -> RMX_BM_W {
        RMX_BM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rmxgm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmxgm](index.html) module"]
pub struct RMXGM_SPEC;
impl crate::RegisterSpec for RMXGM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmxgm::R](R) reader structure"]
impl crate::Readable for RMXGM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmxgm::W](W) writer structure"]
impl crate::Writable for RMXGM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rmxgm to value 0"]
impl crate::Resettable for RMXGM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
