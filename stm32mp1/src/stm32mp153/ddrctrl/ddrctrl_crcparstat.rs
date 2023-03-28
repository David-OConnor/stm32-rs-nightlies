#[doc = "Register `DDRCTRL_CRCPARSTAT` reader"]
pub struct R(crate::R<DDRCTRL_CRCPARSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_CRCPARSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_CRCPARSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_CRCPARSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DFI_ALERT_ERR_CNT` reader - DFI_ALERT_ERR_CNT"]
pub type DFI_ALERT_ERR_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DFI_ALERT_ERR_INT` reader - DFI_ALERT_ERR_INT"]
pub type DFI_ALERT_ERR_INT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - DFI_ALERT_ERR_CNT"]
    #[inline(always)]
    pub fn dfi_alert_err_cnt(&self) -> DFI_ALERT_ERR_CNT_R {
        DFI_ALERT_ERR_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - DFI_ALERT_ERR_INT"]
    #[inline(always)]
    pub fn dfi_alert_err_int(&self) -> DFI_ALERT_ERR_INT_R {
        DFI_ALERT_ERR_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "DDRCTRL CRC parity status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_crcparstat](index.html) module"]
pub struct DDRCTRL_CRCPARSTAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_CRCPARSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_crcparstat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_CRCPARSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_CRCPARSTAT to value 0"]
impl crate::Resettable for DDRCTRL_CRCPARSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
