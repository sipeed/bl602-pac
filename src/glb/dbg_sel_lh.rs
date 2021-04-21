#[doc = "Register `DBG_SEL_LH` reader"]
pub struct R(crate::R<DBG_SEL_LH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_SEL_LH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DBG_SEL_LH_SPEC>> for R {
    fn from(reader: crate::R<DBG_SEL_LH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_SEL_LH` writer"]
pub struct W(crate::W<DBG_SEL_LH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_SEL_LH_SPEC>;
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
impl core::convert::From<crate::W<DBG_SEL_LH_SPEC>> for W {
    fn from(writer: crate::W<DBG_SEL_LH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_dbg_lh_ctrl` reader - "]
pub struct REG_DBG_LH_CTRL_R(crate::FieldReader<u32, u32>);
impl REG_DBG_LH_CTRL_R {
    pub(crate) fn new(bits: u32) -> Self {
        REG_DBG_LH_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_DBG_LH_CTRL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_dbg_lh_ctrl` writer - "]
pub struct REG_DBG_LH_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_DBG_LH_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_lh_ctrl(&self) -> REG_DBG_LH_CTRL_R {
        REG_DBG_LH_CTRL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dbg_lh_ctrl(&mut self) -> REG_DBG_LH_CTRL_W {
        REG_DBG_LH_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBG_SEL_LH.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sel_lh](index.html) module"]
pub struct DBG_SEL_LH_SPEC;
impl crate::RegisterSpec for DBG_SEL_LH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_sel_lh::R](R) reader structure"]
impl crate::Readable for DBG_SEL_LH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_sel_lh::W](W) writer structure"]
impl crate::Writable for DBG_SEL_LH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBG_SEL_LH to value 0"]
impl crate::Resettable for DBG_SEL_LH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
