#[doc = "Register `PDS_INT` reader"]
pub struct R(crate::R<PDS_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDS_INT_SPEC>> for R {
    fn from(reader: crate::R<PDS_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_INT` writer"]
pub struct W(crate::W<PDS_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_INT_SPEC>;
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
impl core::convert::From<crate::W<PDS_INT_SPEC>> for W {
    fn from(writer: crate::W<PDS_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_pds_int_clr` reader - "]
pub struct CR_PDS_INT_CLR_R(crate::FieldReader<bool, bool>);
impl CR_PDS_INT_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_int_clr` writer - "]
pub struct CR_PDS_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `cr_pds_pll_done_int_mask` reader - "]
pub struct CR_PDS_PLL_DONE_INT_MASK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_PLL_DONE_INT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_PLL_DONE_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_PLL_DONE_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_pll_done_int_mask` writer - "]
pub struct CR_PDS_PLL_DONE_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_PLL_DONE_INT_MASK_W<'a> {
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
#[doc = "Field `cr_pds_rf_done_int_mask` reader - "]
pub struct CR_PDS_RF_DONE_INT_MASK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_RF_DONE_INT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_RF_DONE_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_RF_DONE_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_rf_done_int_mask` writer - "]
pub struct CR_PDS_RF_DONE_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_RF_DONE_INT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `cr_pds_irq_in_dis` reader - "]
pub struct CR_PDS_IRQ_IN_DIS_R(crate::FieldReader<bool, bool>);
impl CR_PDS_IRQ_IN_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_IRQ_IN_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_IRQ_IN_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_irq_in_dis` writer - "]
pub struct CR_PDS_IRQ_IN_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_IRQ_IN_DIS_W<'a> {
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
#[doc = "Field `cr_pds_wake_int_mask` reader - "]
pub struct CR_PDS_WAKE_INT_MASK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_WAKE_INT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_WAKE_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_WAKE_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_wake_int_mask` writer - "]
pub struct CR_PDS_WAKE_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WAKE_INT_MASK_W<'a> {
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
#[doc = "Field `ro_pds_pll_done_int` reader - "]
pub struct RO_PDS_PLL_DONE_INT_R(crate::FieldReader<bool, bool>);
impl RO_PDS_PLL_DONE_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RO_PDS_PLL_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_PLL_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_pds_rf_done_int` reader - "]
pub struct RO_PDS_RF_DONE_INT_R(crate::FieldReader<bool, bool>);
impl RO_PDS_RF_DONE_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RO_PDS_RF_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_RF_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_pds_irq_in` reader - "]
pub struct RO_PDS_IRQ_IN_R(crate::FieldReader<bool, bool>);
impl RO_PDS_IRQ_IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RO_PDS_IRQ_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_IRQ_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_pds_wake_int` reader - "]
pub struct RO_PDS_WAKE_INT_R(crate::FieldReader<bool, bool>);
impl RO_PDS_WAKE_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RO_PDS_WAKE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_WAKE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&self) -> CR_PDS_INT_CLR_R {
        CR_PDS_INT_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_pll_done_int_mask(&self) -> CR_PDS_PLL_DONE_INT_MASK_R {
        CR_PDS_PLL_DONE_INT_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&self) -> CR_PDS_RF_DONE_INT_MASK_R {
        CR_PDS_RF_DONE_INT_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_irq_in_dis(&self) -> CR_PDS_IRQ_IN_DIS_R {
        CR_PDS_IRQ_IN_DIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&self) -> CR_PDS_WAKE_INT_MASK_R {
        CR_PDS_WAKE_INT_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_pll_done_int(&self) -> RO_PDS_PLL_DONE_INT_R {
        RO_PDS_PLL_DONE_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&self) -> RO_PDS_RF_DONE_INT_R {
        RO_PDS_RF_DONE_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ro_pds_irq_in(&self) -> RO_PDS_IRQ_IN_R {
        RO_PDS_IRQ_IN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&self) -> RO_PDS_WAKE_INT_R {
        RO_PDS_WAKE_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&mut self) -> CR_PDS_INT_CLR_W {
        CR_PDS_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_pll_done_int_mask(&mut self) -> CR_PDS_PLL_DONE_INT_MASK_W {
        CR_PDS_PLL_DONE_INT_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&mut self) -> CR_PDS_RF_DONE_INT_MASK_W {
        CR_PDS_RF_DONE_INT_MASK_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_irq_in_dis(&mut self) -> CR_PDS_IRQ_IN_DIS_W {
        CR_PDS_IRQ_IN_DIS_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&mut self) -> CR_PDS_WAKE_INT_MASK_W {
        CR_PDS_WAKE_INT_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_INT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_int](index.html) module"]
pub struct PDS_INT_SPEC;
impl crate::RegisterSpec for PDS_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_int::R](R) reader structure"]
impl crate::Readable for PDS_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_int::W](W) writer structure"]
impl crate::Writable for PDS_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDS_INT to value 0"]
impl crate::Resettable for PDS_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
