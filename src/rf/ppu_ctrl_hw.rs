#[doc = "Register `ppu_ctrl_hw` reader"]
pub struct R(crate::R<PPU_CTRL_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPU_CTRL_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PPU_CTRL_HW_SPEC>> for R {
    fn from(reader: crate::R<PPU_CTRL_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ppu_ctrl_hw` writer"]
pub struct W(crate::W<PPU_CTRL_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPU_CTRL_HW_SPEC>;
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
impl core::convert::From<crate::W<PPU_CTRL_HW_SPEC>> for W {
    fn from(writer: crate::W<PPU_CTRL_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ppu_txbuf_hw` reader - "]
pub struct PPU_TXBUF_HW_R(crate::FieldReader<bool, bool>);
impl PPU_TXBUF_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_TXBUF_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_TXBUF_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_txbuf_hw` writer - "]
pub struct PPU_TXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_TXBUF_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `ppu_rxbuf_hw` reader - "]
pub struct PPU_RXBUF_HW_R(crate::FieldReader<bool, bool>);
impl PPU_RXBUF_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_RXBUF_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_RXBUF_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_rxbuf_hw` writer - "]
pub struct PPU_RXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_RXBUF_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `ppu_osmx_hw` reader - "]
pub struct PPU_OSMX_HW_R(crate::FieldReader<bool, bool>);
impl PPU_OSMX_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_OSMX_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_OSMX_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_osmx_hw` writer - "]
pub struct PPU_OSMX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_OSMX_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ppu_pfd_hw` reader - "]
pub struct PPU_PFD_HW_R(crate::FieldReader<bool, bool>);
impl PPU_PFD_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_PFD_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_PFD_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_pfd_hw` writer - "]
pub struct PPU_PFD_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_PFD_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `ppu_fbdv_hw` reader - "]
pub struct PPU_FBDV_HW_R(crate::FieldReader<bool, bool>);
impl PPU_FBDV_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_FBDV_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_FBDV_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_fbdv_hw` writer - "]
pub struct PPU_FBDV_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_FBDV_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `ppu_vco_hw` reader - "]
pub struct PPU_VCO_HW_R(crate::FieldReader<bool, bool>);
impl PPU_VCO_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_VCO_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_VCO_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_vco_hw` writer - "]
pub struct PPU_VCO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_VCO_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `ppu_rbb_hw` reader - "]
pub struct PPU_RBB_HW_R(crate::FieldReader<bool, bool>);
impl PPU_RBB_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_RBB_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_RBB_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_rbb_hw` writer - "]
pub struct PPU_RBB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_RBB_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `ppu_rmxgm_hw` reader - "]
pub struct PPU_RMXGM_HW_R(crate::FieldReader<bool, bool>);
impl PPU_RMXGM_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_RMXGM_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_RMXGM_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_rmxgm_hw` writer - "]
pub struct PPU_RMXGM_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_RMXGM_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ppu_lna_hw` reader - "]
pub struct PPU_LNA_HW_R(crate::FieldReader<bool, bool>);
impl PPU_LNA_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_LNA_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_LNA_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_lna_hw` writer - "]
pub struct PPU_LNA_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_LNA_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ppu_txbuf_hw(&self) -> PPU_TXBUF_HW_R {
        PPU_TXBUF_HW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ppu_rxbuf_hw(&self) -> PPU_RXBUF_HW_R {
        PPU_RXBUF_HW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ppu_osmx_hw(&self) -> PPU_OSMX_HW_R {
        PPU_OSMX_HW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ppu_pfd_hw(&self) -> PPU_PFD_HW_R {
        PPU_PFD_HW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ppu_fbdv_hw(&self) -> PPU_FBDV_HW_R {
        PPU_FBDV_HW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ppu_vco_hw(&self) -> PPU_VCO_HW_R {
        PPU_VCO_HW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ppu_rbb_hw(&self) -> PPU_RBB_HW_R {
        PPU_RBB_HW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ppu_rmxgm_hw(&self) -> PPU_RMXGM_HW_R {
        PPU_RMXGM_HW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ppu_lna_hw(&self) -> PPU_LNA_HW_R {
        PPU_LNA_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ppu_txbuf_hw(&mut self) -> PPU_TXBUF_HW_W {
        PPU_TXBUF_HW_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ppu_rxbuf_hw(&mut self) -> PPU_RXBUF_HW_W {
        PPU_RXBUF_HW_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ppu_osmx_hw(&mut self) -> PPU_OSMX_HW_W {
        PPU_OSMX_HW_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ppu_pfd_hw(&mut self) -> PPU_PFD_HW_W {
        PPU_PFD_HW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ppu_fbdv_hw(&mut self) -> PPU_FBDV_HW_W {
        PPU_FBDV_HW_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ppu_vco_hw(&mut self) -> PPU_VCO_HW_W {
        PPU_VCO_HW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ppu_rbb_hw(&mut self) -> PPU_RBB_HW_W {
        PPU_RBB_HW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ppu_rmxgm_hw(&mut self) -> PPU_RMXGM_HW_W {
        PPU_RMXGM_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ppu_lna_hw(&mut self) -> PPU_LNA_HW_W {
        PPU_LNA_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ppu_ctrl_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppu_ctrl_hw](index.html) module"]
pub struct PPU_CTRL_HW_SPEC;
impl crate::RegisterSpec for PPU_CTRL_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppu_ctrl_hw::R](R) reader structure"]
impl crate::Readable for PPU_CTRL_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppu_ctrl_hw::W](W) writer structure"]
impl crate::Writable for PPU_CTRL_HW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ppu_ctrl_hw to value 0"]
impl crate::Resettable for PPU_CTRL_HW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
