#[doc = "Register `rfif_dig_ctrl` reader"]
pub struct R(crate::R<RFIF_DIG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIF_DIG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIF_DIG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIF_DIG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfif_dig_ctrl` writer"]
pub struct W(crate::W<RFIF_DIG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIF_DIG_CTRL_SPEC>;
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
impl From<crate::W<RFIF_DIG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIF_DIG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rfif_ppud_manaual_en` reader - "]
pub struct RFIF_PPUD_MANAUAL_EN_R(crate::FieldReader<bool, bool>);
impl RFIF_PPUD_MANAUAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFIF_PPUD_MANAUAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIF_PPUD_MANAUAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfif_ppud_manaual_en` writer - "]
pub struct RFIF_PPUD_MANAUAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIF_PPUD_MANAUAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `rfif_ppud_cnt1` reader - "]
pub struct RFIF_PPUD_CNT1_R(crate::FieldReader<u8, u8>);
impl RFIF_PPUD_CNT1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFIF_PPUD_CNT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIF_PPUD_CNT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfif_ppud_cnt1` writer - "]
pub struct RFIF_PPUD_CNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIF_PPUD_CNT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
#[doc = "Field `rfif_ppud_cnt2` reader - "]
pub struct RFIF_PPUD_CNT2_R(crate::FieldReader<u16, u16>);
impl RFIF_PPUD_CNT2_R {
    pub(crate) fn new(bits: u16) -> Self {
        RFIF_PPUD_CNT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIF_PPUD_CNT2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfif_ppud_cnt2` writer - "]
pub struct RFIF_PPUD_CNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIF_PPUD_CNT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `rfif_int_lo_unlocked_mask` reader - "]
pub struct RFIF_INT_LO_UNLOCKED_MASK_R(crate::FieldReader<bool, bool>);
impl RFIF_INT_LO_UNLOCKED_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFIF_INT_LO_UNLOCKED_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIF_INT_LO_UNLOCKED_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfif_int_lo_unlocked_mask` writer - "]
pub struct RFIF_INT_LO_UNLOCKED_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIF_INT_LO_UNLOCKED_MASK_W<'a> {
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
#[doc = "Field `rfckg_rxclk_div2_mode` reader - "]
pub struct RFCKG_RXCLK_DIV2_MODE_R(crate::FieldReader<bool, bool>);
impl RFCKG_RXCLK_DIV2_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCKG_RXCLK_DIV2_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCKG_RXCLK_DIV2_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfckg_rxclk_div2_mode` writer - "]
pub struct RFCKG_RXCLK_DIV2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_RXCLK_DIV2_MODE_W<'a> {
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
#[doc = "Field `test_gc_from_pad_en` reader - "]
pub struct TEST_GC_FROM_PAD_EN_R(crate::FieldReader<bool, bool>);
impl TEST_GC_FROM_PAD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEST_GC_FROM_PAD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_GC_FROM_PAD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `test_gc_from_pad_en` writer - "]
pub struct TEST_GC_FROM_PAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_GC_FROM_PAD_EN_W<'a> {
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
#[doc = "Field `test_from_pad_en` reader - "]
pub struct TEST_FROM_PAD_EN_R(crate::FieldReader<bool, bool>);
impl TEST_FROM_PAD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEST_FROM_PAD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_FROM_PAD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `test_from_pad_en` writer - "]
pub struct TEST_FROM_PAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_FROM_PAD_EN_W<'a> {
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
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rfif_ppud_manaual_en(&self) -> RFIF_PPUD_MANAUAL_EN_R {
        RFIF_PPUD_MANAUAL_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn rfif_ppud_cnt1(&self) -> RFIF_PPUD_CNT1_R {
        RFIF_PPUD_CNT1_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rfif_ppud_cnt2(&self) -> RFIF_PPUD_CNT2_R {
        RFIF_PPUD_CNT2_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfif_int_lo_unlocked_mask(&self) -> RFIF_INT_LO_UNLOCKED_MASK_R {
        RFIF_INT_LO_UNLOCKED_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_rxclk_div2_mode(&self) -> RFCKG_RXCLK_DIV2_MODE_R {
        RFCKG_RXCLK_DIV2_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn test_gc_from_pad_en(&self) -> TEST_GC_FROM_PAD_EN_R {
        TEST_GC_FROM_PAD_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn test_from_pad_en(&self) -> TEST_FROM_PAD_EN_R {
        TEST_FROM_PAD_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rfif_ppud_manaual_en(&mut self) -> RFIF_PPUD_MANAUAL_EN_W {
        RFIF_PPUD_MANAUAL_EN_W { w: self }
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn rfif_ppud_cnt1(&mut self) -> RFIF_PPUD_CNT1_W {
        RFIF_PPUD_CNT1_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rfif_ppud_cnt2(&mut self) -> RFIF_PPUD_CNT2_W {
        RFIF_PPUD_CNT2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfif_int_lo_unlocked_mask(&mut self) -> RFIF_INT_LO_UNLOCKED_MASK_W {
        RFIF_INT_LO_UNLOCKED_MASK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_rxclk_div2_mode(&mut self) -> RFCKG_RXCLK_DIV2_MODE_W {
        RFCKG_RXCLK_DIV2_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn test_gc_from_pad_en(&mut self) -> TEST_GC_FROM_PAD_EN_W {
        TEST_GC_FROM_PAD_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn test_from_pad_en(&mut self) -> TEST_FROM_PAD_EN_W {
        TEST_FROM_PAD_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rfif_dig_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfif_dig_ctrl](index.html) module"]
pub struct RFIF_DIG_CTRL_SPEC;
impl crate::RegisterSpec for RFIF_DIG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfif_dig_ctrl::R](R) reader structure"]
impl crate::Readable for RFIF_DIG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfif_dig_ctrl::W](W) writer structure"]
impl crate::Writable for RFIF_DIG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rfif_dig_ctrl to value 0"]
impl crate::Resettable for RFIF_DIG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
