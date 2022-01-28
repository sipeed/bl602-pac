#[doc = "Register `clk_cfg2` reader"]
pub struct R(crate::R<CLK_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg2` writer"]
pub struct W(crate::W<CLK_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG2_SPEC>;
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
impl From<crate::W<CLK_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_clk_en` reader - "]
pub struct DMA_CLK_EN_R(crate::FieldReader<u8, u8>);
impl DMA_CLK_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_CLK_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_clk_en` writer - "]
pub struct DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CLK_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `ir_clk_en` reader - "]
pub struct IR_CLK_EN_R(crate::FieldReader<bool, bool>);
impl IR_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IR_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IR_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ir_clk_en` writer - "]
pub struct IR_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ir_clk_div` reader - "]
pub struct IR_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl IR_CLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        IR_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IR_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ir_clk_div` writer - "]
pub struct IR_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `sf_clk_sel2` reader - "]
pub struct SF_CLK_SEL2_R(crate::FieldReader<u8, u8>);
impl SF_CLK_SEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_CLK_SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CLK_SEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_clk_sel2` writer - "]
pub struct SF_CLK_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_SEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `sf_clk_sel` reader - "]
pub struct SF_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl SF_CLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_clk_sel` writer - "]
pub struct SF_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `sf_clk_en` reader - "]
pub struct SF_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SF_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_clk_en` writer - "]
pub struct SF_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `sf_clk_div` reader - "]
pub struct SF_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl SF_CLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_clk_div` writer - "]
pub struct SF_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `hbn_uart_clk_sel` reader - "]
pub struct HBN_UART_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl HBN_UART_CLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HBN_UART_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_UART_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_clk_en` reader - "]
pub struct UART_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UART_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_clk_en` writer - "]
pub struct UART_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLK_EN_W<'a> {
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
#[doc = "Field `uart_clk_div` reader - "]
pub struct UART_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl UART_CLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_clk_div` writer - "]
pub struct UART_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ir_clk_en(&self) -> IR_CLK_EN_R {
        IR_CLK_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ir_clk_div(&self) -> IR_CLK_DIV_R {
        IR_CLK_DIV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sf_clk_sel2(&self) -> SF_CLK_SEL2_R {
        SF_CLK_SEL2_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sf_clk_sel(&self) -> SF_CLK_SEL_R {
        SF_CLK_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_clk_en(&self) -> SF_CLK_EN_R {
        SF_CLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_clk_div(&self) -> SF_CLK_DIV_R {
        SF_CLK_DIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HBN_UART_CLK_SEL_R {
        HBN_UART_CLK_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uart_clk_div(&self) -> UART_CLK_DIV_R {
        UART_CLK_DIV_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W {
        DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ir_clk_en(&mut self) -> IR_CLK_EN_W {
        IR_CLK_EN_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ir_clk_div(&mut self) -> IR_CLK_DIV_W {
        IR_CLK_DIV_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sf_clk_sel2(&mut self) -> SF_CLK_SEL2_W {
        SF_CLK_SEL2_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sf_clk_sel(&mut self) -> SF_CLK_SEL_W {
        SF_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_clk_en(&mut self) -> SF_CLK_EN_W {
        SF_CLK_EN_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_clk_div(&mut self) -> SF_CLK_DIV_W {
        SF_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W {
        UART_CLK_EN_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uart_clk_div(&mut self) -> UART_CLK_DIV_W {
        UART_CLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration for UART and Flash\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg2](index.html) module"]
pub struct CLK_CFG2_SPEC;
impl crate::RegisterSpec for CLK_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg2::R](R) reader structure"]
impl crate::Readable for CLK_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg2::W](W) writer structure"]
impl crate::Writable for CLK_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clk_cfg2 to value 0xff8f_2b17"]
impl crate::Resettable for CLK_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff8f_2b17
    }
}
