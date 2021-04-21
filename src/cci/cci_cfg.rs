#[doc = "Register `cci_cfg` reader"]
pub struct R(crate::R<CCI_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCI_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CCI_CFG_SPEC>> for R {
    fn from(reader: crate::R<CCI_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cci_cfg` writer"]
pub struct W(crate::W<CCI_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCI_CFG_SPEC>;
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
impl core::convert::From<crate::W<CCI_CFG_SPEC>> for W {
    fn from(writer: crate::W<CCI_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mcci_clk_inv` reader - "]
pub struct REG_MCCI_CLK_INV_R(crate::FieldReader<bool, bool>);
impl REG_MCCI_CLK_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_MCCI_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MCCI_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mcci_clk_inv` writer - "]
pub struct REG_MCCI_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MCCI_CLK_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `reg_scci_clk_inv` reader - "]
pub struct REG_SCCI_CLK_INV_R(crate::FieldReader<bool, bool>);
impl REG_SCCI_CLK_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_SCCI_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SCCI_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_scci_clk_inv` writer - "]
pub struct REG_SCCI_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SCCI_CLK_INV_W<'a> {
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
#[doc = "Field `cfg_cci1_pre_read` reader - "]
pub struct CFG_CCI1_PRE_READ_R(crate::FieldReader<bool, bool>);
impl CFG_CCI1_PRE_READ_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_CCI1_PRE_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_CCI1_PRE_READ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cfg_cci1_pre_read` writer - "]
pub struct CFG_CCI1_PRE_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_CCI1_PRE_READ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `reg_div_m_cci_sclk` reader - "]
pub struct REG_DIV_M_CCI_SCLK_R(crate::FieldReader<u8, u8>);
impl REG_DIV_M_CCI_SCLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_DIV_M_CCI_SCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_DIV_M_CCI_SCLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_div_m_cci_sclk` writer - "]
pub struct REG_DIV_M_CCI_SCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_DIV_M_CCI_SCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `reg_m_cci_sclk_en` reader - "]
pub struct REG_M_CCI_SCLK_EN_R(crate::FieldReader<bool, bool>);
impl REG_M_CCI_SCLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_M_CCI_SCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_M_CCI_SCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_m_cci_sclk_en` writer - "]
pub struct REG_M_CCI_SCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_M_CCI_SCLK_EN_W<'a> {
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
#[doc = "Field `cci_mas_hw_mode` reader - "]
pub struct CCI_MAS_HW_MODE_R(crate::FieldReader<bool, bool>);
impl CCI_MAS_HW_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCI_MAS_HW_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCI_MAS_HW_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cci_mas_hw_mode` writer - "]
pub struct CCI_MAS_HW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_MAS_HW_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `cci_mas_sel_cci2` reader - "]
pub struct CCI_MAS_SEL_CCI2_R(crate::FieldReader<bool, bool>);
impl CCI_MAS_SEL_CCI2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCI_MAS_SEL_CCI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCI_MAS_SEL_CCI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cci_mas_sel_cci2` writer - "]
pub struct CCI_MAS_SEL_CCI2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_MAS_SEL_CCI2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `cci_slv_sel_cci2` reader - "]
pub struct CCI_SLV_SEL_CCI2_R(crate::FieldReader<bool, bool>);
impl CCI_SLV_SEL_CCI2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCI_SLV_SEL_CCI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCI_SLV_SEL_CCI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cci_slv_sel_cci2` writer - "]
pub struct CCI_SLV_SEL_CCI2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_SLV_SEL_CCI2_W<'a> {
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
#[doc = "Field `cci_en` reader - "]
pub struct CCI_EN_R(crate::FieldReader<bool, bool>);
impl CCI_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cci_en` writer - "]
pub struct CCI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_EN_W<'a> {
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
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mcci_clk_inv(&self) -> REG_MCCI_CLK_INV_R {
        REG_MCCI_CLK_INV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_scci_clk_inv(&self) -> REG_SCCI_CLK_INV_R {
        REG_SCCI_CLK_INV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_cci1_pre_read(&self) -> CFG_CCI1_PRE_READ_R {
        CFG_CCI1_PRE_READ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn reg_div_m_cci_sclk(&self) -> REG_DIV_M_CCI_SCLK_R {
        REG_DIV_M_CCI_SCLK_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_m_cci_sclk_en(&self) -> REG_M_CCI_SCLK_EN_R {
        REG_M_CCI_SCLK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cci_mas_hw_mode(&self) -> CCI_MAS_HW_MODE_R {
        CCI_MAS_HW_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cci_mas_sel_cci2(&self) -> CCI_MAS_SEL_CCI2_R {
        CCI_MAS_SEL_CCI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_slv_sel_cci2(&self) -> CCI_SLV_SEL_CCI2_R {
        CCI_SLV_SEL_CCI2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_en(&self) -> CCI_EN_R {
        CCI_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mcci_clk_inv(&mut self) -> REG_MCCI_CLK_INV_W {
        REG_MCCI_CLK_INV_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_scci_clk_inv(&mut self) -> REG_SCCI_CLK_INV_W {
        REG_SCCI_CLK_INV_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_cci1_pre_read(&mut self) -> CFG_CCI1_PRE_READ_W {
        CFG_CCI1_PRE_READ_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn reg_div_m_cci_sclk(&mut self) -> REG_DIV_M_CCI_SCLK_W {
        REG_DIV_M_CCI_SCLK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_m_cci_sclk_en(&mut self) -> REG_M_CCI_SCLK_EN_W {
        REG_M_CCI_SCLK_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cci_mas_hw_mode(&mut self) -> CCI_MAS_HW_MODE_W {
        CCI_MAS_HW_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cci_mas_sel_cci2(&mut self) -> CCI_MAS_SEL_CCI2_W {
        CCI_MAS_SEL_CCI2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_slv_sel_cci2(&mut self) -> CCI_SLV_SEL_CCI2_W {
        CCI_SLV_SEL_CCI2_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_en(&mut self) -> CCI_EN_W {
        CCI_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cci_cfg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_cfg](index.html) module"]
pub struct CCI_CFG_SPEC;
impl crate::RegisterSpec for CCI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cci_cfg::R](R) reader structure"]
impl crate::Readable for CCI_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cci_cfg::W](W) writer structure"]
impl crate::Writable for CCI_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cci_cfg to value 0x0221"]
impl crate::Resettable for CCI_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0221
    }
}
