#[doc = "Register `se_ctrl_prot_rd` reader"]
pub struct R(crate::R<SE_CTRL_PROT_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_CTRL_PROT_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_CTRL_PROT_RD_SPEC>> for R {
    fn from(reader: crate::R<SE_CTRL_PROT_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_ctrl_prot_rd` writer"]
pub struct W(crate::W<SE_CTRL_PROT_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_CTRL_PROT_RD_SPEC>;
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
impl core::convert::From<crate::W<SE_CTRL_PROT_RD_SPEC>> for W {
    fn from(writer: crate::W<SE_CTRL_PROT_RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_dbg_dis` reader - "]
pub struct SE_DBG_DIS_R(crate::FieldReader<bool, bool>);
impl SE_DBG_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_DBG_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_DBG_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_dbg_dis` writer - "]
pub struct SE_DBG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_DBG_DIS_W<'a> {
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
#[doc = "Field `se_gmac_id1_en_rd` reader - "]
pub struct SE_GMAC_ID1_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_GMAC_ID1_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_GMAC_ID1_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_GMAC_ID1_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_gmac_id1_en_rd` writer - "]
pub struct SE_GMAC_ID1_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_ID1_EN_RD_W<'a> {
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
#[doc = "Field `se_gmac_id0_en_rd` reader - "]
pub struct SE_GMAC_ID0_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_GMAC_ID0_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_GMAC_ID0_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_GMAC_ID0_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_gmac_id0_en_rd` writer - "]
pub struct SE_GMAC_ID0_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_ID0_EN_RD_W<'a> {
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
#[doc = "Field `se_gmac_prot_en_rd` reader - "]
pub struct SE_GMAC_PROT_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_GMAC_PROT_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_GMAC_PROT_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_GMAC_PROT_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_gmac_prot_en_rd` writer - "]
pub struct SE_GMAC_PROT_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_GMAC_PROT_EN_RD_W<'a> {
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
#[doc = "Field `se_cdet_id1_en_rd` reader - "]
pub struct SE_CDET_ID1_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_CDET_ID1_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_CDET_ID1_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_CDET_ID1_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_cdet_id1_en_rd` writer - "]
pub struct SE_CDET_ID1_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_ID1_EN_RD_W<'a> {
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
#[doc = "Field `se_cdet_id0_en_rd` reader - "]
pub struct SE_CDET_ID0_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_CDET_ID0_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_CDET_ID0_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_CDET_ID0_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_cdet_id0_en_rd` writer - "]
pub struct SE_CDET_ID0_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_ID0_EN_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `se_cdet_prot_en_rd` reader - "]
pub struct SE_CDET_PROT_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_CDET_PROT_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_CDET_PROT_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_CDET_PROT_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_cdet_prot_en_rd` writer - "]
pub struct SE_CDET_PROT_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_CDET_PROT_EN_RD_W<'a> {
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
#[doc = "Field `se_pka_id1_en_rd` reader - "]
pub struct SE_PKA_ID1_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_PKA_ID1_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_ID1_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_ID1_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_id1_en_rd` writer - "]
pub struct SE_PKA_ID1_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_ID1_EN_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `se_pka_id0_en_rd` reader - "]
pub struct SE_PKA_ID0_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_PKA_ID0_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_ID0_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_ID0_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_id0_en_rd` writer - "]
pub struct SE_PKA_ID0_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_ID0_EN_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `se_pka_prot_en_rd` reader - "]
pub struct SE_PKA_PROT_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_PKA_PROT_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_PROT_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_PROT_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_prot_en_rd` writer - "]
pub struct SE_PKA_PROT_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_PROT_EN_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `se_trng_id1_en_rd` reader - "]
pub struct SE_TRNG_ID1_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_ID1_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_ID1_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_ID1_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_id1_en_rd` writer - "]
pub struct SE_TRNG_ID1_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_ID1_EN_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `se_trng_id0_en_rd` reader - "]
pub struct SE_TRNG_ID0_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_ID0_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_ID0_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_ID0_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_id0_en_rd` writer - "]
pub struct SE_TRNG_ID0_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_ID0_EN_RD_W<'a> {
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
#[doc = "Field `se_trng_prot_en_rd` reader - "]
pub struct SE_TRNG_PROT_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_PROT_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_PROT_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_PROT_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_prot_en_rd` writer - "]
pub struct SE_TRNG_PROT_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_PROT_EN_RD_W<'a> {
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
#[doc = "Field `se_aes_id1_en_rd` reader - "]
pub struct SE_AES_ID1_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_AES_ID1_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_AES_ID1_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_ID1_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_id1_en_rd` writer - "]
pub struct SE_AES_ID1_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_ID1_EN_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `se_aes_id0_en_rd` reader - "]
pub struct SE_AES_ID0_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_AES_ID0_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_AES_ID0_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_ID0_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_id0_en_rd` writer - "]
pub struct SE_AES_ID0_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_ID0_EN_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `se_aes_prot_en_rd` reader - "]
pub struct SE_AES_PROT_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_AES_PROT_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_AES_PROT_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_PROT_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_prot_en_rd` writer - "]
pub struct SE_AES_PROT_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_PROT_EN_RD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `se_sha_id1_en_rd` reader - "]
pub struct SE_SHA_ID1_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_SHA_ID1_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_ID1_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_ID1_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_id1_en_rd` writer - "]
pub struct SE_SHA_ID1_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_ID1_EN_RD_W<'a> {
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
#[doc = "Field `se_sha_id0_en_rd` reader - "]
pub struct SE_SHA_ID0_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_SHA_ID0_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_ID0_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_ID0_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_id0_en_rd` writer - "]
pub struct SE_SHA_ID0_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_ID0_EN_RD_W<'a> {
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
#[doc = "Field `se_sha_prot_en_rd` reader - "]
pub struct SE_SHA_PROT_EN_RD_R(crate::FieldReader<bool, bool>);
impl SE_SHA_PROT_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_PROT_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_PROT_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_prot_en_rd` writer - "]
pub struct SE_SHA_PROT_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_PROT_EN_RD_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn se_dbg_dis(&self) -> SE_DBG_DIS_R {
        SE_DBG_DIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn se_gmac_id1_en_rd(&self) -> SE_GMAC_ID1_EN_RD_R {
        SE_GMAC_ID1_EN_RD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn se_gmac_id0_en_rd(&self) -> SE_GMAC_ID0_EN_RD_R {
        SE_GMAC_ID0_EN_RD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn se_gmac_prot_en_rd(&self) -> SE_GMAC_PROT_EN_RD_R {
        SE_GMAC_PROT_EN_RD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn se_cdet_id1_en_rd(&self) -> SE_CDET_ID1_EN_RD_R {
        SE_CDET_ID1_EN_RD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn se_cdet_id0_en_rd(&self) -> SE_CDET_ID0_EN_RD_R {
        SE_CDET_ID0_EN_RD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_cdet_prot_en_rd(&self) -> SE_CDET_PROT_EN_RD_R {
        SE_CDET_PROT_EN_RD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_pka_id1_en_rd(&self) -> SE_PKA_ID1_EN_RD_R {
        SE_PKA_ID1_EN_RD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_id0_en_rd(&self) -> SE_PKA_ID0_EN_RD_R {
        SE_PKA_ID0_EN_RD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_prot_en_rd(&self) -> SE_PKA_PROT_EN_RD_R {
        SE_PKA_PROT_EN_RD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_id1_en_rd(&self) -> SE_TRNG_ID1_EN_RD_R {
        SE_TRNG_ID1_EN_RD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_id0_en_rd(&self) -> SE_TRNG_ID0_EN_RD_R {
        SE_TRNG_ID0_EN_RD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_prot_en_rd(&self) -> SE_TRNG_PROT_EN_RD_R {
        SE_TRNG_PROT_EN_RD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_id1_en_rd(&self) -> SE_AES_ID1_EN_RD_R {
        SE_AES_ID1_EN_RD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_aes_id0_en_rd(&self) -> SE_AES_ID0_EN_RD_R {
        SE_AES_ID0_EN_RD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_aes_prot_en_rd(&self) -> SE_AES_PROT_EN_RD_R {
        SE_AES_PROT_EN_RD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_sha_id1_en_rd(&self) -> SE_SHA_ID1_EN_RD_R {
        SE_SHA_ID1_EN_RD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_id0_en_rd(&self) -> SE_SHA_ID0_EN_RD_R {
        SE_SHA_ID0_EN_RD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_prot_en_rd(&self) -> SE_SHA_PROT_EN_RD_R {
        SE_SHA_PROT_EN_RD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn se_dbg_dis(&mut self) -> SE_DBG_DIS_W {
        SE_DBG_DIS_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn se_gmac_id1_en_rd(&mut self) -> SE_GMAC_ID1_EN_RD_W {
        SE_GMAC_ID1_EN_RD_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn se_gmac_id0_en_rd(&mut self) -> SE_GMAC_ID0_EN_RD_W {
        SE_GMAC_ID0_EN_RD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn se_gmac_prot_en_rd(&mut self) -> SE_GMAC_PROT_EN_RD_W {
        SE_GMAC_PROT_EN_RD_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn se_cdet_id1_en_rd(&mut self) -> SE_CDET_ID1_EN_RD_W {
        SE_CDET_ID1_EN_RD_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn se_cdet_id0_en_rd(&mut self) -> SE_CDET_ID0_EN_RD_W {
        SE_CDET_ID0_EN_RD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_cdet_prot_en_rd(&mut self) -> SE_CDET_PROT_EN_RD_W {
        SE_CDET_PROT_EN_RD_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_pka_id1_en_rd(&mut self) -> SE_PKA_ID1_EN_RD_W {
        SE_PKA_ID1_EN_RD_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_id0_en_rd(&mut self) -> SE_PKA_ID0_EN_RD_W {
        SE_PKA_ID0_EN_RD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_prot_en_rd(&mut self) -> SE_PKA_PROT_EN_RD_W {
        SE_PKA_PROT_EN_RD_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_id1_en_rd(&mut self) -> SE_TRNG_ID1_EN_RD_W {
        SE_TRNG_ID1_EN_RD_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_id0_en_rd(&mut self) -> SE_TRNG_ID0_EN_RD_W {
        SE_TRNG_ID0_EN_RD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_prot_en_rd(&mut self) -> SE_TRNG_PROT_EN_RD_W {
        SE_TRNG_PROT_EN_RD_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_id1_en_rd(&mut self) -> SE_AES_ID1_EN_RD_W {
        SE_AES_ID1_EN_RD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_aes_id0_en_rd(&mut self) -> SE_AES_ID0_EN_RD_W {
        SE_AES_ID0_EN_RD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_aes_prot_en_rd(&mut self) -> SE_AES_PROT_EN_RD_W {
        SE_AES_PROT_EN_RD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_sha_id1_en_rd(&mut self) -> SE_SHA_ID1_EN_RD_W {
        SE_SHA_ID1_EN_RD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_id0_en_rd(&mut self) -> SE_SHA_ID0_EN_RD_W {
        SE_SHA_ID0_EN_RD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_prot_en_rd(&mut self) -> SE_SHA_PROT_EN_RD_W {
        SE_SHA_PROT_EN_RD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_ctrl_prot_rd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_ctrl_prot_rd](index.html) module"]
pub struct SE_CTRL_PROT_RD_SPEC;
impl crate::RegisterSpec for SE_CTRL_PROT_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_ctrl_prot_rd::R](R) reader structure"]
impl crate::Readable for SE_CTRL_PROT_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_ctrl_prot_rd::W](W) writer structure"]
impl crate::Writable for SE_CTRL_PROT_RD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_ctrl_prot_rd to value 0"]
impl crate::Resettable for SE_CTRL_PROT_RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
