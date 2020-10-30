#[doc = "Register `sram_parm` reader"]
pub struct R(crate::R<SRAM_PARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_PARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SRAM_PARM_SPEC>> for R {
    fn from(reader: crate::R<SRAM_PARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sram_parm` writer"]
pub struct W(crate::W<SRAM_PARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_PARM_SPEC>;
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
impl core::convert::From<crate::W<SRAM_PARM_SPEC>> for W {
    fn from(writer: crate::W<SRAM_PARM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_sram_parm` reader - "]
pub struct REG_SRAM_PARM_R(crate::FieldReader<u32, u32>);
impl REG_SRAM_PARM_R {
    pub(crate) fn new(bits: u32) -> Self {
        REG_SRAM_PARM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SRAM_PARM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_sram_parm` writer - "]
pub struct REG_SRAM_PARM_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SRAM_PARM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_sram_parm(&self) -> REG_SRAM_PARM_R {
        REG_SRAM_PARM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_sram_parm(&mut self) -> REG_SRAM_PARM_W {
        REG_SRAM_PARM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sram_parm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_parm](index.html) module"]
pub struct SRAM_PARM_SPEC;
impl crate::RegisterSpec for SRAM_PARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_parm::R](R) reader structure"]
impl crate::Readable for SRAM_PARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_parm::W](W) writer structure"]
impl crate::Writable for SRAM_PARM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sram_parm to value 0"]
impl crate::Resettable for SRAM_PARM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
