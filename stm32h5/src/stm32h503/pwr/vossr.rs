#[doc = "Register `VOSSR` reader"]
pub struct R(crate::R<VOSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VOSRDY` reader - Ready bit for V<sub>CORE</sub> voltage scaling output selection."]
pub type VOSRDY_R = crate::BitReader<bool>;
#[doc = "Field `ACTVOSRDY` reader - Voltage level ready for currently used VOS"]
pub type ACTVOSRDY_R = crate::BitReader<bool>;
#[doc = "Field `ACTVOS` reader - voltage output scaling currently applied to V<sub>CORE</sub> This field provides the last VOS value."]
pub type ACTVOS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 3 - Ready bit for V<sub>CORE</sub> voltage scaling output selection."]
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 13 - Voltage level ready for currently used VOS"]
    #[inline(always)]
    pub fn actvosrdy(&self) -> ACTVOSRDY_R {
        ACTVOSRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - voltage output scaling currently applied to V<sub>CORE</sub> This field provides the last VOS value."]
    #[inline(always)]
    pub fn actvos(&self) -> ACTVOS_R {
        ACTVOS_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "PWR voltage scaling status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vossr](index.html) module"]
pub struct VOSSR_SPEC;
impl crate::RegisterSpec for VOSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vossr::R](R) reader structure"]
impl crate::Readable for VOSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VOSSR to value 0x08"]
impl crate::Resettable for VOSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
