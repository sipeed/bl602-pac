#[doc = "Register `bg_sys_top` reader"]
pub struct R(crate::R<BG_SYS_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_SYS_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BG_SYS_TOP_SPEC>> for R {
    fn from(reader: crate::R<BG_SYS_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bg_sys_top` writer"]
pub struct W(crate::W<BG_SYS_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BG_SYS_TOP_SPEC>;
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
impl core::convert::From<crate::W<BG_SYS_TOP_SPEC>> for W {
    fn from(writer: crate::W<BG_SYS_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bg_sys_start_ctrl_aon` reader - "]
pub struct BG_SYS_START_CTRL_AON_R(crate::FieldReader<bool, bool>);
impl BG_SYS_START_CTRL_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        BG_SYS_START_CTRL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BG_SYS_START_CTRL_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bg_sys_start_ctrl_aon` writer - "]
pub struct BG_SYS_START_CTRL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> BG_SYS_START_CTRL_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `pu_bg_sys_aon` reader - "]
pub struct PU_BG_SYS_AON_R(crate::FieldReader<bool, bool>);
impl PU_BG_SYS_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_BG_SYS_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_BG_SYS_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_bg_sys_aon` writer - "]
pub struct PU_BG_SYS_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_BG_SYS_AON_W<'a> {
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
#[doc = "Field `pmip_resv` reader - "]
pub struct PMIP_RESV_R(crate::FieldReader<u8, u8>);
impl PMIP_RESV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PMIP_RESV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMIP_RESV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pmip_resv` writer - "]
pub struct PMIP_RESV_W<'a> {
    w: &'a mut W,
}
impl<'a> PMIP_RESV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bg_sys_start_ctrl_aon(&self) -> BG_SYS_START_CTRL_AON_R {
        BG_SYS_START_CTRL_AON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&self) -> PU_BG_SYS_AON_R {
        PU_BG_SYS_AON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pmip_resv(&self) -> PMIP_RESV_R {
        PMIP_RESV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bg_sys_start_ctrl_aon(&mut self) -> BG_SYS_START_CTRL_AON_W {
        BG_SYS_START_CTRL_AON_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&mut self) -> PU_BG_SYS_AON_W {
        PU_BG_SYS_AON_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pmip_resv(&mut self) -> PMIP_RESV_W {
        PMIP_RESV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bg_sys_top.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_sys_top](index.html) module"]
pub struct BG_SYS_TOP_SPEC;
impl crate::RegisterSpec for BG_SYS_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_sys_top::R](R) reader structure"]
impl crate::Readable for BG_SYS_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bg_sys_top::W](W) writer structure"]
impl crate::Writable for BG_SYS_TOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets bg_sys_top to value 0x1100"]
impl crate::Resettable for BG_SYS_TOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1100
    }
}
