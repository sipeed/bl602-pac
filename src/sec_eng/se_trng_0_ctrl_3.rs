#[doc = "Register `se_trng_0_ctrl_3` reader"]
pub struct R(crate::R<SE_TRNG_0_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_TRNG_0_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_TRNG_0_CTRL_3_SPEC>> for R {
    fn from(reader: crate::R<SE_TRNG_0_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_trng_0_ctrl_3` writer"]
pub struct W(crate::W<SE_TRNG_0_CTRL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_TRNG_0_CTRL_3_SPEC>;
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
impl core::convert::From<crate::W<SE_TRNG_0_CTRL_3_SPEC>> for W {
    fn from(writer: crate::W<SE_TRNG_0_CTRL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_trng_0_rosc_en` reader - "]
pub struct SE_TRNG_0_ROSC_EN_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_ROSC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_ROSC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_ROSC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_rosc_en` writer - "]
pub struct SE_TRNG_0_ROSC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_ROSC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `se_trng_0_ht_od_en` reader - "]
pub struct SE_TRNG_0_HT_OD_EN_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_HT_OD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_HT_OD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_HT_OD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_ht_od_en` writer - "]
pub struct SE_TRNG_0_HT_OD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_OD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `se_trng_0_ht_apt_c` reader - "]
pub struct SE_TRNG_0_HT_APT_C_R(crate::FieldReader<u16, u16>);
impl SE_TRNG_0_HT_APT_C_R {
    pub(crate) fn new(bits: u16) -> Self {
        SE_TRNG_0_HT_APT_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_HT_APT_C_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_ht_apt_c` writer - "]
pub struct SE_TRNG_0_HT_APT_C_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_APT_C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `se_trng_0_ht_rct_c` reader - "]
pub struct SE_TRNG_0_HT_RCT_C_R(crate::FieldReader<u8, u8>);
impl SE_TRNG_0_HT_RCT_C_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_TRNG_0_HT_RCT_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_HT_RCT_C_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_ht_rct_c` writer - "]
pub struct SE_TRNG_0_HT_RCT_C_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_RCT_C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `se_trng_0_cp_ratio` reader - "]
pub struct SE_TRNG_0_CP_RATIO_R(crate::FieldReader<u8, u8>);
impl SE_TRNG_0_CP_RATIO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_TRNG_0_CP_RATIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_CP_RATIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_cp_ratio` writer - "]
pub struct SE_TRNG_0_CP_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_CP_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn se_trng_0_rosc_en(&self) -> SE_TRNG_0_ROSC_EN_R {
        SE_TRNG_0_ROSC_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn se_trng_0_ht_od_en(&self) -> SE_TRNG_0_HT_OD_EN_R {
        SE_TRNG_0_HT_OD_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn se_trng_0_ht_apt_c(&self) -> SE_TRNG_0_HT_APT_C_R {
        SE_TRNG_0_HT_APT_C_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_trng_0_ht_rct_c(&self) -> SE_TRNG_0_HT_RCT_C_R {
        SE_TRNG_0_HT_RCT_C_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_trng_0_cp_ratio(&self) -> SE_TRNG_0_CP_RATIO_R {
        SE_TRNG_0_CP_RATIO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn se_trng_0_rosc_en(&mut self) -> SE_TRNG_0_ROSC_EN_W {
        SE_TRNG_0_ROSC_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn se_trng_0_ht_od_en(&mut self) -> SE_TRNG_0_HT_OD_EN_W {
        SE_TRNG_0_HT_OD_EN_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn se_trng_0_ht_apt_c(&mut self) -> SE_TRNG_0_HT_APT_C_W {
        SE_TRNG_0_HT_APT_C_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn se_trng_0_ht_rct_c(&mut self) -> SE_TRNG_0_HT_RCT_C_W {
        SE_TRNG_0_HT_RCT_C_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn se_trng_0_cp_ratio(&mut self) -> SE_TRNG_0_CP_RATIO_W {
        SE_TRNG_0_CP_RATIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_trng_0_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_3](index.html) module"]
pub struct SE_TRNG_0_CTRL_3_SPEC;
impl crate::RegisterSpec for SE_TRNG_0_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_trng_0_ctrl_3::R](R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_3::W](W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_trng_0_ctrl_3 to value 0"]
impl crate::Resettable for SE_TRNG_0_CTRL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
