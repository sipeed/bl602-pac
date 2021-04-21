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
#[doc = "sf_ctrl_prot_en_rd.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_prot_en_rd](index.html) module"]
pub struct SF_CTRL_PROT_EN_RD_SPEC;
impl crate::RegisterSpec for SF_CTRL_PROT_EN_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_prot_en_rd::R](R) reader structure"]
impl crate::Readable for SF_CTRL_PROT_EN_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sf_ctrl_prot_en_rd to value 0x07"]
impl crate::Resettable for SF_CTRL_PROT_EN_RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
