#[doc = "Register `dfe_ctrl_18` reader"]
pub struct R(crate::R<DFE_CTRL_18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_18` writer"]
pub struct W(crate::W<DFE_CTRL_18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_18_SPEC>;
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
impl From<crate::W<DFE_CTRL_18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_dvga_gain_qdb_ble_gc2` reader - "]
pub struct TX_DVGA_GAIN_QDB_BLE_GC2_R(crate::FieldReader<u8, u8>);
impl TX_DVGA_GAIN_QDB_BLE_GC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_DVGA_GAIN_QDB_BLE_GC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DVGA_GAIN_QDB_BLE_GC2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dvga_gain_qdb_ble_gc2` writer - "]
pub struct TX_DVGA_GAIN_QDB_BLE_GC2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_BLE_GC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `tx_dvga_gain_qdb_ble_gc1` reader - "]
pub struct TX_DVGA_GAIN_QDB_BLE_GC1_R(crate::FieldReader<u8, u8>);
impl TX_DVGA_GAIN_QDB_BLE_GC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_DVGA_GAIN_QDB_BLE_GC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DVGA_GAIN_QDB_BLE_GC1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dvga_gain_qdb_ble_gc1` writer - "]
pub struct TX_DVGA_GAIN_QDB_BLE_GC1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_BLE_GC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `tx_dvga_gain_qdb_ble_gc0` reader - "]
pub struct TX_DVGA_GAIN_QDB_BLE_GC0_R(crate::FieldReader<u8, u8>);
impl TX_DVGA_GAIN_QDB_BLE_GC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_DVGA_GAIN_QDB_BLE_GC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DVGA_GAIN_QDB_BLE_GC0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dvga_gain_qdb_ble_gc0` writer - "]
pub struct TX_DVGA_GAIN_QDB_BLE_GC0_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_BLE_GC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc2(&self) -> TX_DVGA_GAIN_QDB_BLE_GC2_R {
        TX_DVGA_GAIN_QDB_BLE_GC2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc1(&self) -> TX_DVGA_GAIN_QDB_BLE_GC1_R {
        TX_DVGA_GAIN_QDB_BLE_GC1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc0(&self) -> TX_DVGA_GAIN_QDB_BLE_GC0_R {
        TX_DVGA_GAIN_QDB_BLE_GC0_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc2(&mut self) -> TX_DVGA_GAIN_QDB_BLE_GC2_W {
        TX_DVGA_GAIN_QDB_BLE_GC2_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc1(&mut self) -> TX_DVGA_GAIN_QDB_BLE_GC1_W {
        TX_DVGA_GAIN_QDB_BLE_GC1_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_ble_gc0(&mut self) -> TX_DVGA_GAIN_QDB_BLE_GC0_W {
        TX_DVGA_GAIN_QDB_BLE_GC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_18.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_18](index.html) module"]
pub struct DFE_CTRL_18_SPEC;
impl crate::RegisterSpec for DFE_CTRL_18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_18::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_18::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_18_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_18 to value 0"]
impl crate::Resettable for DFE_CTRL_18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
