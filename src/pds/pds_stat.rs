#[doc = "Register `pds_stat` reader"]
pub struct R(crate::R<PDS_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDS_STAT_SPEC>> for R {
    fn from(reader: crate::R<PDS_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_stat` writer"]
pub struct W(crate::W<PDS_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_STAT_SPEC>;
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
impl core::convert::From<crate::W<PDS_STAT_SPEC>> for W {
    fn from(writer: crate::W<PDS_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ro_pds_pll_state` reader - "]
pub struct RO_PDS_PLL_STATE_R(crate::FieldReader<u8, u8>);
impl RO_PDS_PLL_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RO_PDS_PLL_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_PLL_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_pds_pll_state` writer - "]
pub struct RO_PDS_PLL_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_PLL_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `ro_pds_rf_state` reader - "]
pub struct RO_PDS_RF_STATE_R(crate::FieldReader<u8, u8>);
impl RO_PDS_RF_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RO_PDS_RF_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_RF_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_pds_rf_state` writer - "]
pub struct RO_PDS_RF_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_RF_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `ro_pds_state` reader - "]
pub struct RO_PDS_STATE_R(crate::FieldReader<u8, u8>);
impl RO_PDS_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RO_PDS_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_pds_state` writer - "]
pub struct RO_PDS_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ro_pds_pll_state(&self) -> RO_PDS_PLL_STATE_R {
        RO_PDS_PLL_STATE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&self) -> RO_PDS_RF_STATE_R {
        RO_PDS_RF_STATE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ro_pds_state(&self) -> RO_PDS_STATE_R {
        RO_PDS_STATE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ro_pds_pll_state(&mut self) -> RO_PDS_PLL_STATE_W {
        RO_PDS_PLL_STATE_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ro_pds_rf_state(&mut self) -> RO_PDS_RF_STATE_W {
        RO_PDS_RF_STATE_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ro_pds_state(&mut self) -> RO_PDS_STATE_W {
        RO_PDS_STATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_stat.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_stat](index.html) module"]
pub struct PDS_STAT_SPEC;
impl crate::RegisterSpec for PDS_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_stat::R](R) reader structure"]
impl crate::Readable for PDS_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_stat::W](W) writer structure"]
impl crate::Writable for PDS_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pds_stat to value 0"]
impl crate::Resettable for PDS_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
