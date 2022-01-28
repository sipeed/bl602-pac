#[doc = "Register `gpdac_dma_config` reader"]
pub struct R(crate::R<GPDAC_DMA_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_DMA_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_DMA_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_DMA_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_dma_config` writer"]
pub struct W(crate::W<GPDAC_DMA_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_DMA_CONFIG_SPEC>;
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
impl From<crate::W<GPDAC_DMA_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDAC_DMA_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpdac_dma_format` reader - "]
pub struct GPDAC_DMA_FORMAT_R(crate::FieldReader<u8, u8>);
impl GPDAC_DMA_FORMAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPDAC_DMA_FORMAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_DMA_FORMAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_dma_format` writer - "]
pub struct GPDAC_DMA_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_DMA_FORMAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `gpdac_dma_tx_en` reader - "]
pub struct GPDAC_DMA_TX_EN_R(crate::FieldReader<bool, bool>);
impl GPDAC_DMA_TX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDAC_DMA_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_DMA_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_dma_tx_en` writer - "]
pub struct GPDAC_DMA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_DMA_TX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gpdac_dma_format(&self) -> GPDAC_DMA_FORMAT_R {
        GPDAC_DMA_FORMAT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_dma_tx_en(&self) -> GPDAC_DMA_TX_EN_R {
        GPDAC_DMA_TX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn gpdac_dma_format(&mut self) -> GPDAC_DMA_FORMAT_W {
        GPDAC_DMA_FORMAT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdac_dma_tx_en(&mut self) -> GPDAC_DMA_TX_EN_W {
        GPDAC_DMA_TX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpdac_dma_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_dma_config](index.html) module"]
pub struct GPDAC_DMA_CONFIG_SPEC;
impl crate::RegisterSpec for GPDAC_DMA_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_dma_config::R](R) reader structure"]
impl crate::Readable for GPDAC_DMA_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_dma_config::W](W) writer structure"]
impl crate::Writable for GPDAC_DMA_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpdac_dma_config to value 0"]
impl crate::Resettable for GPDAC_DMA_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
