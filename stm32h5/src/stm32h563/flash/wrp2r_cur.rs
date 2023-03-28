#[doc = "Register `WRP2R_CUR` reader"]
pub struct R(crate::R<WRP2R_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRP2R_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRP2R_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRP2R_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRPSG2` reader - Bank2 sector group protection option status byte Each FLASH_WRP2R_CUR bit reflects the write protection status of the corresponding group of 4 consecutive sectors in bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
pub type WRPSG2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bank2 sector group protection option status byte Each FLASH_WRP2R_CUR bit reflects the write protection status of the corresponding group of 4 consecutive sectors in bank 2 (0: group is write protected; 1: group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127"]
    #[inline(always)]
    pub fn wrpsg2(&self) -> WRPSG2_R {
        WRPSG2_R::new(self.bits)
    }
}
#[doc = "FLASH write sector group protection for Bank 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp2r_cur](index.html) module"]
pub struct WRP2R_CUR_SPEC;
impl crate::RegisterSpec for WRP2R_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrp2r_cur::R](R) reader structure"]
impl crate::Readable for WRP2R_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WRP2R_CUR to value 0"]
impl crate::Resettable for WRP2R_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
