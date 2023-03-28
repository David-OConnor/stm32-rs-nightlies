#[doc = "Register `FMC_SR` reader"]
pub struct R(crate::R<FMC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ISOST` reader - ISOST"]
pub type ISOST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PEF` reader - PEF"]
pub type PEF_R = crate::BitReader<bool>;
#[doc = "Field `NWRF` reader - NWRF"]
pub type NWRF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - ISOST"]
    #[inline(always)]
    pub fn isost(&self) -> ISOST_R {
        ISOST_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PEF"]
    #[inline(always)]
    pub fn pef(&self) -> PEF_R {
        PEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - NWRF"]
    #[inline(always)]
    pub fn nwrf(&self) -> NWRF_R {
        NWRF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sr](index.html) module"]
pub struct FMC_SR_SPEC;
impl crate::RegisterSpec for FMC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_sr::R](R) reader structure"]
impl crate::Readable for FMC_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_SR to value 0x40"]
impl crate::Resettable for FMC_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
