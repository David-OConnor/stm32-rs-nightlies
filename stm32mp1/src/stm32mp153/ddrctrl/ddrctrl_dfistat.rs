#[doc = "Register `DDRCTRL_DFISTAT` reader"]
pub struct R(crate::R<DDRCTRL_DFISTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DFISTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DFISTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DFISTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DFI_INIT_COMPLETE` reader - DFI_INIT_COMPLETE"]
pub type DFI_INIT_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `DFI_LP_ACK` reader - DFI_LP_ACK"]
pub type DFI_LP_ACK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DFI_INIT_COMPLETE"]
    #[inline(always)]
    pub fn dfi_init_complete(&self) -> DFI_INIT_COMPLETE_R {
        DFI_INIT_COMPLETE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DFI_LP_ACK"]
    #[inline(always)]
    pub fn dfi_lp_ack(&self) -> DFI_LP_ACK_R {
        DFI_LP_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "DDRCTRL DFI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfistat](index.html) module"]
pub struct DDRCTRL_DFISTAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_DFISTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dfistat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DFISTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_DFISTAT to value 0"]
impl crate::Resettable for DDRCTRL_DFISTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
