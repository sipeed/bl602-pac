#[doc = "Register `HBN_IRQ_MODE` reader"]
pub struct R(crate::R<HBN_IRQ_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_IRQ_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HBN_IRQ_MODE_SPEC>> for R {
    fn from(reader: crate::R<HBN_IRQ_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_IRQ_MODE` writer"]
pub struct W(crate::W<HBN_IRQ_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_IRQ_MODE_SPEC>;
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
impl core::convert::From<crate::W<HBN_IRQ_MODE_SPEC>> for W {
    fn from(writer: crate::W<HBN_IRQ_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin_wakeup_en` reader - "]
pub struct PIN_WAKEUP_EN_R(crate::FieldReader<bool, bool>);
impl PIN_WAKEUP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_WAKEUP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_WAKEUP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin_wakeup_en` writer - "]
pub struct PIN_WAKEUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_WAKEUP_EN_W<'a> {
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
#[doc = "Field `pin_wakeup_sel` reader - "]
pub struct PIN_WAKEUP_SEL_R(crate::FieldReader<u8, u8>);
impl PIN_WAKEUP_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PIN_WAKEUP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_WAKEUP_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin_wakeup_sel` writer - "]
pub struct PIN_WAKEUP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_WAKEUP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `irq_acomp1_en` reader - "]
pub struct IRQ_ACOMP1_EN_R(crate::FieldReader<u8, u8>);
impl IRQ_ACOMP1_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        IRQ_ACOMP1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_ACOMP1_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irq_acomp1_en` writer - "]
pub struct IRQ_ACOMP1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_ACOMP1_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `irq_acomp0_en` reader - "]
pub struct IRQ_ACOMP0_EN_R(crate::FieldReader<u8, u8>);
impl IRQ_ACOMP0_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        IRQ_ACOMP0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_ACOMP0_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irq_acomp0_en` writer - "]
pub struct IRQ_ACOMP0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_ACOMP0_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `irq_bor_en` reader - "]
pub struct IRQ_BOR_EN_R(crate::FieldReader<bool, bool>);
impl IRQ_BOR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_BOR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_BOR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irq_bor_en` writer - "]
pub struct IRQ_BOR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_BOR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `reg_en_hw_pu_pd` reader - "]
pub struct REG_EN_HW_PU_PD_R(crate::FieldReader<bool, bool>);
impl REG_EN_HW_PU_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_EN_HW_PU_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_EN_HW_PU_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_en_hw_pu_pd` writer - "]
pub struct REG_EN_HW_PU_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_EN_HW_PU_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `reg_aon_pad_ie_smt` reader - "]
pub struct REG_AON_PAD_IE_SMT_R(crate::FieldReader<bool, bool>);
impl REG_AON_PAD_IE_SMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_AON_PAD_IE_SMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_AON_PAD_IE_SMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_aon_pad_ie_smt` writer - "]
pub struct REG_AON_PAD_IE_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_AON_PAD_IE_SMT_W<'a> {
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
#[doc = "Field `hbn_pin_wakeup_mask` reader - "]
pub struct HBN_PIN_WAKEUP_MASK_R(crate::FieldReader<u8, u8>);
impl HBN_PIN_WAKEUP_MASK_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBN_PIN_WAKEUP_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_PIN_WAKEUP_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_pin_wakeup_mask` writer - "]
pub struct HBN_PIN_WAKEUP_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_PIN_WAKEUP_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `hbn_pin_wakeup_mode` reader - "]
pub struct HBN_PIN_WAKEUP_MODE_R(crate::FieldReader<u8, u8>);
impl HBN_PIN_WAKEUP_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBN_PIN_WAKEUP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_PIN_WAKEUP_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_pin_wakeup_mode` writer - "]
pub struct HBN_PIN_WAKEUP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_PIN_WAKEUP_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin_wakeup_en(&self) -> PIN_WAKEUP_EN_R {
        PIN_WAKEUP_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pin_wakeup_sel(&self) -> PIN_WAKEUP_SEL_R {
        PIN_WAKEUP_SEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn irq_acomp1_en(&self) -> IRQ_ACOMP1_EN_R {
        IRQ_ACOMP1_EN_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn irq_acomp0_en(&self) -> IRQ_ACOMP0_EN_R {
        IRQ_ACOMP0_EN_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn irq_bor_en(&self) -> IRQ_BOR_EN_R {
        IRQ_BOR_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_en_hw_pu_pd(&self) -> REG_EN_HW_PU_PD_R {
        REG_EN_HW_PU_PD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_aon_pad_ie_smt(&self) -> REG_AON_PAD_IE_SMT_R {
        REG_AON_PAD_IE_SMT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mask(&self) -> HBN_PIN_WAKEUP_MASK_R {
        HBN_PIN_WAKEUP_MASK_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mode(&self) -> HBN_PIN_WAKEUP_MODE_R {
        HBN_PIN_WAKEUP_MODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin_wakeup_en(&mut self) -> PIN_WAKEUP_EN_W {
        PIN_WAKEUP_EN_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pin_wakeup_sel(&mut self) -> PIN_WAKEUP_SEL_W {
        PIN_WAKEUP_SEL_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn irq_acomp1_en(&mut self) -> IRQ_ACOMP1_EN_W {
        IRQ_ACOMP1_EN_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn irq_acomp0_en(&mut self) -> IRQ_ACOMP0_EN_W {
        IRQ_ACOMP0_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn irq_bor_en(&mut self) -> IRQ_BOR_EN_W {
        IRQ_BOR_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_en_hw_pu_pd(&mut self) -> REG_EN_HW_PU_PD_W {
        REG_EN_HW_PU_PD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_aon_pad_ie_smt(&mut self) -> REG_AON_PAD_IE_SMT_W {
        REG_AON_PAD_IE_SMT_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mask(&mut self) -> HBN_PIN_WAKEUP_MASK_W {
        HBN_PIN_WAKEUP_MASK_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mode(&mut self) -> HBN_PIN_WAKEUP_MODE_W {
        HBN_PIN_WAKEUP_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_IRQ_MODE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_irq_mode](index.html) module"]
pub struct HBN_IRQ_MODE_SPEC;
impl crate::RegisterSpec for HBN_IRQ_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_irq_mode::R](R) reader structure"]
impl crate::Readable for HBN_IRQ_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_irq_mode::W](W) writer structure"]
impl crate::Writable for HBN_IRQ_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_IRQ_MODE to value 0"]
impl crate::Resettable for HBN_IRQ_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
