#[doc = "Register `utx_ir_position` reader"]
pub struct R(crate::R<UTX_IR_POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UTX_IR_POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UTX_IR_POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UTX_IR_POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `utx_ir_position` writer"]
pub struct W(crate::W<UTX_IR_POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UTX_IR_POSITION_SPEC>;
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
impl From<crate::W<UTX_IR_POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UTX_IR_POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_utx_ir_pos_p` reader - "]
pub struct CR_UTX_IR_POS_P_R(crate::FieldReader<u16, u16>);
impl CR_UTX_IR_POS_P_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_UTX_IR_POS_P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_IR_POS_P_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_ir_pos_p` writer - "]
pub struct CR_UTX_IR_POS_P_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_IR_POS_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `cr_utx_ir_pos_s` reader - "]
pub struct CR_UTX_IR_POS_S_R(crate::FieldReader<u16, u16>);
impl CR_UTX_IR_POS_S_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_UTX_IR_POS_S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_IR_POS_S_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_ir_pos_s` writer - "]
pub struct CR_UTX_IR_POS_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_IR_POS_S_W<'a> {
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
    pub fn cr_utx_ir_pos_p(&self) -> CR_UTX_IR_POS_P_R {
        CR_UTX_IR_POS_P_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_s(&self) -> CR_UTX_IR_POS_S_R {
        CR_UTX_IR_POS_S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_p(&mut self) -> CR_UTX_IR_POS_P_W {
        CR_UTX_IR_POS_P_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_utx_ir_pos_s(&mut self) -> CR_UTX_IR_POS_S_W {
        CR_UTX_IR_POS_S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "utx_ir_position.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utx_ir_position](index.html) module"]
pub struct UTX_IR_POSITION_SPEC;
impl crate::RegisterSpec for UTX_IR_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [utx_ir_position::R](R) reader structure"]
impl crate::Readable for UTX_IR_POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [utx_ir_position::W](W) writer structure"]
impl crate::Writable for UTX_IR_POSITION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets utx_ir_position to value 0x009f_0070"]
impl crate::Resettable for UTX_IR_POSITION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x009f_0070
    }
}
