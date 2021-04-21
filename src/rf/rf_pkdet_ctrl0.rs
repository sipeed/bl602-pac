#[doc = "Register `rf_pkdet_ctrl0` reader"]
pub struct R(crate::R<RF_PKDET_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_PKDET_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_PKDET_CTRL0_SPEC>> for R {
    fn from(reader: crate::R<RF_PKDET_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_pkdet_ctrl0` writer"]
pub struct W(crate::W<RF_PKDET_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_PKDET_CTRL0_SPEC>;
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
impl core::convert::From<crate::W<RF_PKDET_CTRL0_SPEC>> for W {
    fn from(writer: crate::W<RF_PKDET_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pkdet_out_mode` reader - "]
pub struct PKDET_OUT_MODE_R(crate::FieldReader<bool, bool>);
impl PKDET_OUT_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKDET_OUT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKDET_OUT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pkdet_out_mode` writer - "]
pub struct PKDET_OUT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `pkdet_out_cnt_en` reader - "]
pub struct PKDET_OUT_CNT_EN_R(crate::FieldReader<bool, bool>);
impl PKDET_OUT_CNT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKDET_OUT_CNT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKDET_OUT_CNT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pkdet_out_cnt_en` writer - "]
pub struct PKDET_OUT_CNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_CNT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `pkdet_out_cnt_sts` reader - "]
pub struct PKDET_OUT_CNT_STS_R(crate::FieldReader<u8, u8>);
impl PKDET_OUT_CNT_STS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKDET_OUT_CNT_STS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKDET_OUT_CNT_STS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pkdet_out_cnt_sts` writer - "]
pub struct PKDET_OUT_CNT_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_CNT_STS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pkdet_out_mode(&self) -> PKDET_OUT_MODE_R {
        PKDET_OUT_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pkdet_out_cnt_en(&self) -> PKDET_OUT_CNT_EN_R {
        PKDET_OUT_CNT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pkdet_out_cnt_sts(&self) -> PKDET_OUT_CNT_STS_R {
        PKDET_OUT_CNT_STS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pkdet_out_mode(&mut self) -> PKDET_OUT_MODE_W {
        PKDET_OUT_MODE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pkdet_out_cnt_en(&mut self) -> PKDET_OUT_CNT_EN_W {
        PKDET_OUT_CNT_EN_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pkdet_out_cnt_sts(&mut self) -> PKDET_OUT_CNT_STS_W {
        PKDET_OUT_CNT_STS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_pkdet_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_pkdet_ctrl0](index.html) module"]
pub struct RF_PKDET_CTRL0_SPEC;
impl crate::RegisterSpec for RF_PKDET_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_pkdet_ctrl0::R](R) reader structure"]
impl crate::Readable for RF_PKDET_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_pkdet_ctrl0::W](W) writer structure"]
impl crate::Writable for RF_PKDET_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_pkdet_ctrl0 to value 0"]
impl crate::Resettable for RF_PKDET_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
