#[doc = "Register `PSSI_SR` reader"]
pub struct R(crate::R<PSSI_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSI_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSI_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSI_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTT4B` reader - FIFO is ready to transfer four bytes"]
pub type RTT4B_R = crate::BitReader<bool>;
#[doc = "Field `RTT1B` reader - FIFO is ready to transfer one byte"]
pub type RTT1B_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - FIFO is ready to transfer four bytes"]
    #[inline(always)]
    pub fn rtt4b(&self) -> RTT4B_R {
        RTT4B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO is ready to transfer one byte"]
    #[inline(always)]
    pub fn rtt1b(&self) -> RTT1B_R {
        RTT1B_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PSSI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssi_sr](index.html) module"]
pub struct PSSI_SR_SPEC;
impl crate::RegisterSpec for PSSI_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssi_sr::R](R) reader structure"]
impl crate::Readable for PSSI_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSSI_SR to value 0"]
impl crate::Resettable for PSSI_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
