#[doc = "Register `lo_sdm_ctrl_hw7` reader"]
pub struct R(crate::R<LO_SDM_CTRL_HW7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SDM_CTRL_HW7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LO_SDM_CTRL_HW7_SPEC>> for R {
    fn from(reader: crate::R<LO_SDM_CTRL_HW7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_sdm_ctrl_hw7` writer"]
pub struct W(crate::W<LO_SDM_CTRL_HW7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SDM_CTRL_HW7_SPEC>;
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
impl core::convert::From<crate::W<LO_SDM_CTRL_HW7_SPEC>> for W {
    fn from(writer: crate::W<LO_SDM_CTRL_HW7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdmin_1m` reader - "]
pub struct LO_SDMIN_1M_R(crate::FieldReader<u32, u32>);
impl LO_SDMIN_1M_R {
    pub(crate) fn new(bits: u32) -> Self {
        LO_SDMIN_1M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDMIN_1M_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdmin_1m` writer - "]
pub struct LO_SDMIN_1M_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDMIN_1M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn lo_sdmin_1m(&self) -> LO_SDMIN_1M_R {
        LO_SDMIN_1M_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn lo_sdmin_1m(&mut self) -> LO_SDMIN_1M_W {
        LO_SDMIN_1M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_sdm_ctrl_hw7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_sdm_ctrl_hw7](index.html) module"]
pub struct LO_SDM_CTRL_HW7_SPEC;
impl crate::RegisterSpec for LO_SDM_CTRL_HW7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_sdm_ctrl_hw7::R](R) reader structure"]
impl crate::Readable for LO_SDM_CTRL_HW7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_sdm_ctrl_hw7::W](W) writer structure"]
impl crate::Writable for LO_SDM_CTRL_HW7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_sdm_ctrl_hw7 to value 0"]
impl crate::Resettable for LO_SDM_CTRL_HW7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
