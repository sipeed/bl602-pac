#[doc = "Register `dcdc18_top_1` reader"]
pub struct R(crate::R<DCDC18_TOP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC18_TOP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC18_TOP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC18_TOP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dcdc18_top_1` writer"]
pub struct W(crate::W<DCDC18_TOP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC18_TOP_1_SPEC>;
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
impl From<crate::W<DCDC18_TOP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC18_TOP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dcdc18_pulldown_aon` reader - "]
pub struct DCDC18_PULLDOWN_AON_R(crate::FieldReader<bool, bool>);
impl DCDC18_PULLDOWN_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC18_PULLDOWN_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_PULLDOWN_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_pulldown_aon` writer - "]
pub struct DCDC18_PULLDOWN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_PULLDOWN_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `dcdc18_en_antiring_aon` reader - "]
pub struct DCDC18_EN_ANTIRING_AON_R(crate::FieldReader<bool, bool>);
impl DCDC18_EN_ANTIRING_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC18_EN_ANTIRING_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_EN_ANTIRING_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_en_antiring_aon` writer - "]
pub struct DCDC18_EN_ANTIRING_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_EN_ANTIRING_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `dcdc18_cfb_sel_aon` reader - "]
pub struct DCDC18_CFB_SEL_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_CFB_SEL_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_CFB_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_CFB_SEL_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_cfb_sel_aon` writer - "]
pub struct DCDC18_CFB_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_CFB_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `dcdc18_chf_sel_aon` reader - "]
pub struct DCDC18_CHF_SEL_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_CHF_SEL_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_CHF_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_CHF_SEL_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_chf_sel_aon` writer - "]
pub struct DCDC18_CHF_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_CHF_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `dcdc18_rc_sel_aon` reader - "]
pub struct DCDC18_RC_SEL_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_RC_SEL_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_RC_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_RC_SEL_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_rc_sel_aon` writer - "]
pub struct DCDC18_RC_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_RC_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `dcdc18_nonoverlap_td_aon` reader - "]
pub struct DCDC18_NONOVERLAP_TD_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_NONOVERLAP_TD_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_NONOVERLAP_TD_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_NONOVERLAP_TD_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_nonoverlap_td_aon` writer - "]
pub struct DCDC18_NONOVERLAP_TD_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_NONOVERLAP_TD_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `dcdc18_zvs_td_opt_aon` reader - "]
pub struct DCDC18_ZVS_TD_OPT_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_ZVS_TD_OPT_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_ZVS_TD_OPT_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_ZVS_TD_OPT_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_zvs_td_opt_aon` writer - "]
pub struct DCDC18_ZVS_TD_OPT_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_ZVS_TD_OPT_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `dcdc18_cs_delay_aon` reader - "]
pub struct DCDC18_CS_DELAY_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_CS_DELAY_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_CS_DELAY_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_CS_DELAY_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_cs_delay_aon` writer - "]
pub struct DCDC18_CS_DELAY_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_CS_DELAY_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `dcdc18_force_cs_zvs_aon` reader - "]
pub struct DCDC18_FORCE_CS_ZVS_AON_R(crate::FieldReader<bool, bool>);
impl DCDC18_FORCE_CS_ZVS_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC18_FORCE_CS_ZVS_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_FORCE_CS_ZVS_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_force_cs_zvs_aon` writer - "]
pub struct DCDC18_FORCE_CS_ZVS_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_FORCE_CS_ZVS_AON_W<'a> {
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
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dcdc18_pulldown_aon(&self) -> DCDC18_PULLDOWN_AON_R {
        DCDC18_PULLDOWN_AON_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dcdc18_en_antiring_aon(&self) -> DCDC18_EN_ANTIRING_AON_R {
        DCDC18_EN_ANTIRING_AON_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dcdc18_cfb_sel_aon(&self) -> DCDC18_CFB_SEL_AON_R {
        DCDC18_CFB_SEL_AON_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_chf_sel_aon(&self) -> DCDC18_CHF_SEL_AON_R {
        DCDC18_CHF_SEL_AON_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_rc_sel_aon(&self) -> DCDC18_RC_SEL_AON_R {
        DCDC18_RC_SEL_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn dcdc18_nonoverlap_td_aon(&self) -> DCDC18_NONOVERLAP_TD_AON_R {
        DCDC18_NONOVERLAP_TD_AON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc18_zvs_td_opt_aon(&self) -> DCDC18_ZVS_TD_OPT_AON_R {
        DCDC18_ZVS_TD_OPT_AON_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn dcdc18_cs_delay_aon(&self) -> DCDC18_CS_DELAY_AON_R {
        DCDC18_CS_DELAY_AON_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dcdc18_force_cs_zvs_aon(&self) -> DCDC18_FORCE_CS_ZVS_AON_R {
        DCDC18_FORCE_CS_ZVS_AON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dcdc18_pulldown_aon(&mut self) -> DCDC18_PULLDOWN_AON_W {
        DCDC18_PULLDOWN_AON_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dcdc18_en_antiring_aon(&mut self) -> DCDC18_EN_ANTIRING_AON_W {
        DCDC18_EN_ANTIRING_AON_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dcdc18_cfb_sel_aon(&mut self) -> DCDC18_CFB_SEL_AON_W {
        DCDC18_CFB_SEL_AON_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc18_chf_sel_aon(&mut self) -> DCDC18_CHF_SEL_AON_W {
        DCDC18_CHF_SEL_AON_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_rc_sel_aon(&mut self) -> DCDC18_RC_SEL_AON_W {
        DCDC18_RC_SEL_AON_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn dcdc18_nonoverlap_td_aon(&mut self) -> DCDC18_NONOVERLAP_TD_AON_W {
        DCDC18_NONOVERLAP_TD_AON_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc18_zvs_td_opt_aon(&mut self) -> DCDC18_ZVS_TD_OPT_AON_W {
        DCDC18_ZVS_TD_OPT_AON_W { w: self }
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn dcdc18_cs_delay_aon(&mut self) -> DCDC18_CS_DELAY_AON_W {
        DCDC18_CS_DELAY_AON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dcdc18_force_cs_zvs_aon(&mut self) -> DCDC18_FORCE_CS_ZVS_AON_W {
        DCDC18_FORCE_CS_ZVS_AON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dcdc18_top_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc18_top_1](index.html) module"]
pub struct DCDC18_TOP_1_SPEC;
impl crate::RegisterSpec for DCDC18_TOP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc18_top_1::R](R) reader structure"]
impl crate::Readable for DCDC18_TOP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc18_top_1::W](W) writer structure"]
impl crate::Writable for DCDC18_TOP_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dcdc18_top_1 to value 0x1818_0048"]
impl crate::Resettable for DCDC18_TOP_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1818_0048
    }
}
