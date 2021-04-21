#[doc = "Register `urx_ir_position` reader"]
pub struct R(crate::R<URX_IR_POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<URX_IR_POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<URX_IR_POSITION_SPEC>> for R {
    fn from(reader: crate::R<URX_IR_POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `urx_ir_position` writer"]
pub struct W(crate::W<URX_IR_POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<URX_IR_POSITION_SPEC>;
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
impl core::convert::From<crate::W<URX_IR_POSITION_SPEC>> for W {
    fn from(writer: crate::W<URX_IR_POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_urx_ir_pos_s` reader - "]
pub struct CR_URX_IR_POS_S_R(crate::FieldReader<u16, u16>);
impl CR_URX_IR_POS_S_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_URX_IR_POS_S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_IR_POS_S_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_ir_pos_s` writer - "]
pub struct CR_URX_IR_POS_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_IR_POS_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_urx_ir_pos_s(&self) -> CR_URX_IR_POS_S_R {
        CR_URX_IR_POS_S_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_urx_ir_pos_s(&mut self) -> CR_URX_IR_POS_S_W {
        CR_URX_IR_POS_S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "urx_ir_position.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [urx_ir_position](index.html) module"]
pub struct URX_IR_POSITION_SPEC;
impl crate::RegisterSpec for URX_IR_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [urx_ir_position::R](R) reader structure"]
impl crate::Readable for URX_IR_POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [urx_ir_position::W](W) writer structure"]
impl crate::Writable for URX_IR_POSITION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets urx_ir_position to value 0x6f"]
impl crate::Resettable for URX_IR_POSITION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6f
    }
}
