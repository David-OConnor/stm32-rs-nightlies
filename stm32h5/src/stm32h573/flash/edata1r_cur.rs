#[doc = "Register `EDATA1R_CUR` reader"]
pub struct R(crate::R<EDATA1R_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDATA1R_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDATA1R_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDATA1R_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EDATA1_STRT` reader - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ..."]
pub type EDATA1_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDATA1_EN` reader - Bank1 flash high-cycle data enable"]
pub type EDATA1_EN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ..."]
    #[inline(always)]
    pub fn edata1_strt(&self) -> EDATA1_STRT_R {
        EDATA1_STRT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 15 - Bank1 flash high-cycle data enable"]
    #[inline(always)]
    pub fn edata1_en(&self) -> EDATA1_EN_R {
        EDATA1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "FLASH data sector configuration Bank 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edata1r_cur](index.html) module"]
pub struct EDATA1R_CUR_SPEC;
impl crate::RegisterSpec for EDATA1R_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edata1r_cur::R](R) reader structure"]
impl crate::Readable for EDATA1R_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EDATA1R_CUR to value 0"]
impl crate::Resettable for EDATA1R_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
