#[doc = "Register `DDRCTRL_SWSTAT` reader"]
pub struct R(crate::R<DDRCTRL_SWSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_SWSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_SWSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_SWSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SW_DONE_ACK` reader - SW_DONE_ACK"]
pub type SW_DONE_ACK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SW_DONE_ACK"]
    #[inline(always)]
    pub fn sw_done_ack(&self) -> SW_DONE_ACK_R {
        SW_DONE_ACK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DDRCTRL software register programming control status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_swstat](index.html) module"]
pub struct DDRCTRL_SWSTAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_SWSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_swstat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_SWSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_SWSTAT to value 0x01"]
impl crate::Resettable for DDRCTRL_SWSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
