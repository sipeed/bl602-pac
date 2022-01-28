#[doc = "Register `DMA_Top_Config` reader"]
pub struct R(crate::R<DMA_TOP_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TOP_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TOP_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TOP_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_Top_Config` writer"]
pub struct W(crate::W<DMA_TOP_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_TOP_CONFIG_SPEC>;
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
impl From<crate::W<DMA_TOP_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_TOP_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M` reader - "]
pub struct M_R(crate::FieldReader<bool, bool>);
impl M_R {
    pub(crate) fn new(bits: bool) -> Self {
        M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M` writer - "]
pub struct M_W<'a> {
    w: &'a mut W,
}
impl<'a> M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `E` reader - "]
pub struct E_R(crate::FieldReader<bool, bool>);
impl E_R {
    pub(crate) fn new(bits: bool) -> Self {
        E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E` writer - "]
pub struct E_W<'a> {
    w: &'a mut W,
}
impl<'a> E_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&mut self) -> E_W {
        E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_Top_Config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_top_config](index.html) module"]
pub struct DMA_TOP_CONFIG_SPEC;
impl crate::RegisterSpec for DMA_TOP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_top_config::R](R) reader structure"]
impl crate::Readable for DMA_TOP_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_top_config::W](W) writer structure"]
impl crate::Writable for DMA_TOP_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_Top_Config to value 0"]
impl crate::Resettable for DMA_TOP_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
