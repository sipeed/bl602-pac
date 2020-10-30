#[doc = "Register `clk_cfg0` reader"]
pub struct R(crate::R<CLK_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CFG0_SPEC>> for R {
    fn from(reader: crate::R<CLK_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg0` writer"]
pub struct W(crate::W<CLK_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG0_SPEC>;
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
impl core::convert::From<crate::W<CLK_CFG0_SPEC>> for W {
    fn from(writer: crate::W<CLK_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `glb_id` reader - "]
pub struct GLB_ID_R(crate::FieldReader<u8, u8>);
impl GLB_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        GLB_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLB_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `glb_id` writer - "]
pub struct GLB_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> GLB_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `chip_rdy` reader - "]
pub struct CHIP_RDY_R(crate::FieldReader<bool, bool>);
impl CHIP_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHIP_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `chip_rdy` writer - "]
pub struct CHIP_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `fclk_sw_state` reader - "]
pub struct FCLK_SW_STATE_R(crate::FieldReader<u8, u8>);
impl FCLK_SW_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCLK_SW_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCLK_SW_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fclk_sw_state` writer - "]
pub struct FCLK_SW_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCLK_SW_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `reg_bclk_div` reader - "]
pub struct REG_BCLK_DIV_R(crate::FieldReader<u8, u8>);
impl REG_BCLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_BCLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_BCLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_bclk_div` writer - "]
pub struct REG_BCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_BCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `reg_hclk_div` reader - "]
pub struct REG_HCLK_DIV_R(crate::FieldReader<u8, u8>);
impl REG_HCLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_HCLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_HCLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_hclk_div` writer - "]
pub struct REG_HCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_HCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `hbn_root_clk_sel` reader - "]
pub struct HBN_ROOT_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl HBN_ROOT_CLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBN_ROOT_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_ROOT_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_root_clk_sel` writer - "]
pub struct HBN_ROOT_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_ROOT_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `reg_pll_sel` reader - "]
pub struct REG_PLL_SEL_R(crate::FieldReader<u8, u8>);
impl REG_PLL_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_PLL_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_PLL_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_pll_sel` writer - "]
pub struct REG_PLL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PLL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `reg_bclk_en` reader - "]
pub struct REG_BCLK_EN_R(crate::FieldReader<bool, bool>);
impl REG_BCLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_BCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_BCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_bclk_en` writer - "]
pub struct REG_BCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_BCLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `reg_hclk_en` reader - "]
pub struct REG_HCLK_EN_R(crate::FieldReader<bool, bool>);
impl REG_HCLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_HCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_HCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_hclk_en` writer - "]
pub struct REG_HCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_HCLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `reg_fclk_en` reader - "]
pub struct REG_FCLK_EN_R(crate::FieldReader<bool, bool>);
impl REG_FCLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_FCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_FCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_fclk_en` writer - "]
pub struct REG_FCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_FCLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `reg_pll_en` reader - "]
pub struct REG_PLL_EN_R(crate::FieldReader<bool, bool>);
impl REG_PLL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_PLL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_PLL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_pll_en` writer - "]
pub struct REG_PLL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PLL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn glb_id(&self) -> GLB_ID_R {
        GLB_ID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn chip_rdy(&self) -> CHIP_RDY_R {
        CHIP_RDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn fclk_sw_state(&self) -> FCLK_SW_STATE_R {
        FCLK_SW_STATE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_bclk_div(&self) -> REG_BCLK_DIV_R {
        REG_BCLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_hclk_div(&self) -> REG_HCLK_DIV_R {
        REG_HCLK_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&self) -> HBN_ROOT_CLK_SEL_R {
        HBN_ROOT_CLK_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_pll_sel(&self) -> REG_PLL_SEL_R {
        REG_PLL_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_bclk_en(&self) -> REG_BCLK_EN_R {
        REG_BCLK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_hclk_en(&self) -> REG_HCLK_EN_R {
        REG_HCLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_fclk_en(&self) -> REG_FCLK_EN_R {
        REG_FCLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pll_en(&self) -> REG_PLL_EN_R {
        REG_PLL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn glb_id(&mut self) -> GLB_ID_W {
        GLB_ID_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn chip_rdy(&mut self) -> CHIP_RDY_W {
        CHIP_RDY_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn fclk_sw_state(&mut self) -> FCLK_SW_STATE_W {
        FCLK_SW_STATE_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_bclk_div(&mut self) -> REG_BCLK_DIV_W {
        REG_BCLK_DIV_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_hclk_div(&mut self) -> REG_HCLK_DIV_W {
        REG_HCLK_DIV_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&mut self) -> HBN_ROOT_CLK_SEL_W {
        HBN_ROOT_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_pll_sel(&mut self) -> REG_PLL_SEL_W {
        REG_PLL_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_bclk_en(&mut self) -> REG_BCLK_EN_W {
        REG_BCLK_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_hclk_en(&mut self) -> REG_HCLK_EN_W {
        REG_HCLK_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_fclk_en(&mut self) -> REG_FCLK_EN_W {
        REG_FCLK_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pll_en(&mut self) -> REG_PLL_EN_W {
        REG_PLL_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clk_cfg0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg0](index.html) module"]
pub struct CLK_CFG0_SPEC;
impl crate::RegisterSpec for CLK_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg0::R](R) reader structure"]
impl crate::Readable for CLK_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg0::W](W) writer structure"]
impl crate::Writable for CLK_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clk_cfg0 to value 0"]
impl crate::Resettable for CLK_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
