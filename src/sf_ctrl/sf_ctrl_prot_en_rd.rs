#[doc = "Register `sf_ctrl_prot_en_rd` reader"]
pub struct R(crate::R<SF_CTRL_PROT_EN_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_PROT_EN_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SF_CTRL_PROT_EN_RD_SPEC>> for R {
    fn from(reader: crate::R<SF_CTRL_PROT_EN_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_prot_en_rd` writer"]
pub struct W(crate::W<SF_CTRL_PROT_EN_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_PROT_EN_RD_SPEC>;
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
impl core::convert::From<crate::W<SF_CTRL_PROT_EN_RD_SPEC>> for W {
    fn from(writer: crate::W<SF_CTRL_PROT_EN_RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_dbg_dis` reader - "]
pub struct SF_DBG_DIS_R(crate::FieldReader<bool, bool>);
impl SF_DBG_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_DBG_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_DBG_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_dbg_dis` writer - "]
pub struct SF_DBG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_DBG_DIS_W<'a> {
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
#[doc = "Field `sf_if_0_trig_wr_lock` reader - "]
pub struct SF_IF_0_TRIG_WR_LOCK_R(crate::FieldReader<bool, bool>);
impl SF_IF_0_TRIG_WR_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_0_TRIG_WR_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_0_TRIG_WR_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_0_trig_wr_lock` writer - "]
pub struct SF_IF_0_TRIG_WR_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_TRIG_WR_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `sf_ctrl_id1_en_rd` reader - "]
pub struct SF_CTRL_ID1_EN_RD_R(crate::FieldReader<bool, bool>);
impl SF_CTRL_ID1_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CTRL_ID1_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CTRL_ID1_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_ctrl_id1_en_rd` writer - "]
pub struct SF_CTRL_ID1_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_ID1_EN_RD_W<'a> {
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
#[doc = "Field `sf_ctrl_id0_en_rd` reader - "]
pub struct SF_CTRL_ID0_EN_RD_R(crate::FieldReader<bool, bool>);
impl SF_CTRL_ID0_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CTRL_ID0_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CTRL_ID0_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_ctrl_id0_en_rd` writer - "]
pub struct SF_CTRL_ID0_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_ID0_EN_RD_W<'a> {
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
#[doc = "Field `sf_ctrl_prot_en_rd` reader - "]
pub struct SF_CTRL_PROT_EN_RD_R(crate::FieldReader<bool, bool>);
impl SF_CTRL_PROT_EN_RD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CTRL_PROT_EN_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CTRL_PROT_EN_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_ctrl_prot_en_rd` writer - "]
pub struct SF_CTRL_PROT_EN_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_PROT_EN_RD_W<'a> {
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
    pub fn sf_dbg_dis(&self) -> SF_DBG_DIS_R {
        SF_DBG_DIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_0_trig_wr_lock(&self) -> SF_IF_0_TRIG_WR_LOCK_R {
        SF_IF_0_TRIG_WR_LOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en_rd(&self) -> SF_CTRL_ID1_EN_RD_R {
        SF_CTRL_ID1_EN_RD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en_rd(&self) -> SF_CTRL_ID0_EN_RD_R {
        SF_CTRL_ID0_EN_RD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en_rd(&self) -> SF_CTRL_PROT_EN_RD_R {
        SF_CTRL_PROT_EN_RD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_dbg_dis(&mut self) -> SF_DBG_DIS_W {
        SF_DBG_DIS_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_0_trig_wr_lock(&mut self) -> SF_IF_0_TRIG_WR_LOCK_W {
        SF_IF_0_TRIG_WR_LOCK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en_rd(&mut self) -> SF_CTRL_ID1_EN_RD_W {
        SF_CTRL_ID1_EN_RD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en_rd(&mut self) -> SF_CTRL_ID0_EN_RD_W {
        SF_CTRL_ID0_EN_RD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en_rd(&mut self) -> SF_CTRL_PROT_EN_RD_W {
        SF_CTRL_PROT_EN_RD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_prot_en_rd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_prot_en_rd](index.html) module"]
pub struct SF_CTRL_PROT_EN_RD_SPEC;
impl crate::RegisterSpec for SF_CTRL_PROT_EN_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_prot_en_rd::R](R) reader structure"]
impl crate::Readable for SF_CTRL_PROT_EN_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_prot_en_rd::W](W) writer structure"]
impl crate::Writable for SF_CTRL_PROT_EN_RD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_ctrl_prot_en_rd to value 0"]
impl crate::Resettable for SF_CTRL_PROT_EN_RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
