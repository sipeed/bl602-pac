#[doc = "Register `se_trng_0_test` reader"]
pub struct R(crate::R<SE_TRNG_0_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_TRNG_0_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_TRNG_0_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_TRNG_0_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_trng_0_test` writer"]
pub struct W(crate::W<SE_TRNG_0_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_TRNG_0_TEST_SPEC>;
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
impl From<crate::W<SE_TRNG_0_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_TRNG_0_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_trng_0_ht_alarm_n` reader - "]
pub struct SE_TRNG_0_HT_ALARM_N_R(crate::FieldReader<u8, u8>);
impl SE_TRNG_0_HT_ALARM_N_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_TRNG_0_HT_ALARM_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_HT_ALARM_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_ht_alarm_n` writer - "]
pub struct SE_TRNG_0_HT_ALARM_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_ALARM_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | ((value as u32 & 0xff) << 4);
        self.w
    }
}
#[doc = "Field `se_trng_0_ht_dis` reader - "]
pub struct SE_TRNG_0_HT_DIS_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_HT_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_HT_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_HT_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_ht_dis` writer - "]
pub struct SE_TRNG_0_HT_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_DIS_W<'a> {
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
#[doc = "Field `se_trng_0_cp_bypass` reader - "]
pub struct SE_TRNG_0_CP_BYPASS_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_CP_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_CP_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_CP_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_cp_bypass` writer - "]
pub struct SE_TRNG_0_CP_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_CP_BYPASS_W<'a> {
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
#[doc = "Field `se_trng_0_cp_test_en` reader - "]
pub struct SE_TRNG_0_CP_TEST_EN_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_CP_TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_CP_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_CP_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_cp_test_en` writer - "]
pub struct SE_TRNG_0_CP_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_CP_TEST_EN_W<'a> {
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
#[doc = "Field `se_trng_0_test_en` reader - "]
pub struct SE_TRNG_0_TEST_EN_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_test_en` writer - "]
pub struct SE_TRNG_0_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_TEST_EN_W<'a> {
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
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn se_trng_0_ht_alarm_n(&self) -> SE_TRNG_0_HT_ALARM_N_R {
        SE_TRNG_0_HT_ALARM_N_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_ht_dis(&self) -> SE_TRNG_0_HT_DIS_R {
        SE_TRNG_0_HT_DIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_cp_bypass(&self) -> SE_TRNG_0_CP_BYPASS_R {
        SE_TRNG_0_CP_BYPASS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_cp_test_en(&self) -> SE_TRNG_0_CP_TEST_EN_R {
        SE_TRNG_0_CP_TEST_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_test_en(&self) -> SE_TRNG_0_TEST_EN_R {
        SE_TRNG_0_TEST_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn se_trng_0_ht_alarm_n(&mut self) -> SE_TRNG_0_HT_ALARM_N_W {
        SE_TRNG_0_HT_ALARM_N_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_ht_dis(&mut self) -> SE_TRNG_0_HT_DIS_W {
        SE_TRNG_0_HT_DIS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_cp_bypass(&mut self) -> SE_TRNG_0_CP_BYPASS_W {
        SE_TRNG_0_CP_BYPASS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_cp_test_en(&mut self) -> SE_TRNG_0_CP_TEST_EN_W {
        SE_TRNG_0_CP_TEST_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_test_en(&mut self) -> SE_TRNG_0_TEST_EN_W {
        SE_TRNG_0_TEST_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_trng_0_test.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_test](index.html) module"]
pub struct SE_TRNG_0_TEST_SPEC;
impl crate::RegisterSpec for SE_TRNG_0_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_trng_0_test::R](R) reader structure"]
impl crate::Readable for SE_TRNG_0_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_trng_0_test::W](W) writer structure"]
impl crate::Writable for SE_TRNG_0_TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_trng_0_test to value 0"]
impl crate::Resettable for SE_TRNG_0_TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
