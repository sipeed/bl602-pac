#[doc = "Register `dfe_ctrl_7` reader"]
pub struct R(crate::R<DFE_CTRL_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFE_CTRL_7_SPEC>> for R {
    fn from(reader: crate::R<DFE_CTRL_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_7` writer"]
pub struct W(crate::W<DFE_CTRL_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_7_SPEC>;
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
impl core::convert::From<crate::W<DFE_CTRL_7_SPEC>> for W {
    fn from(writer: crate::W<DFE_CTRL_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_pm_acc_len` reader - "]
pub struct RX_PM_ACC_LEN_R(crate::FieldReader<u16, u16>);
impl RX_PM_ACC_LEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_PM_ACC_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PM_ACC_LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_pm_acc_len` writer - "]
pub struct RX_PM_ACC_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_ACC_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `rx_pm_start_ofs` reader - "]
pub struct RX_PM_START_OFS_R(crate::FieldReader<u16, u16>);
impl RX_PM_START_OFS_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_PM_START_OFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PM_START_OFS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_pm_start_ofs` writer - "]
pub struct RX_PM_START_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PM_START_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rx_pm_acc_len(&self) -> RX_PM_ACC_LEN_R {
        RX_PM_ACC_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_pm_start_ofs(&self) -> RX_PM_START_OFS_R {
        RX_PM_START_OFS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rx_pm_acc_len(&mut self) -> RX_PM_ACC_LEN_W {
        RX_PM_ACC_LEN_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_pm_start_ofs(&mut self) -> RX_PM_START_OFS_W {
        RX_PM_START_OFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_7](index.html) module"]
pub struct DFE_CTRL_7_SPEC;
impl crate::RegisterSpec for DFE_CTRL_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_7::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_7::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_7 to value 0"]
impl crate::Resettable for DFE_CTRL_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
